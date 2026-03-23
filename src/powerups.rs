use rand::prelude::*;
use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::{GameState, PowerUpKind};

/// Sprite presets used for each power-up kind.
/// We use rolling ball sprites so they're visually distinct from obstacles.
fn preset_for(kind: PowerUpKind) -> SpritePreset {
    match kind {
        PowerUpKind::Health => SpritePreset::RollingBallBlue,
        PowerUpKind::Shield => SpritePreset::RollingBallBlueAlt,
        PowerUpKind::SpeedBoost => SpritePreset::RollingBallRed,
    }
}

/// Choose a random power-up kind.
fn random_kind() -> PowerUpKind {
    match thread_rng().gen_range(0..3) {
        0 => PowerUpKind::Health,
        1 => PowerUpKind::Shield,
        _ => PowerUpKind::SpeedBoost,
    }
}

/// Periodically spawn a power-up.
pub fn spawn(engine: &mut Engine, game_state: &mut GameState) {
    game_state.powerup_spawn_timer += engine.delta_f32;
    if game_state.powerup_spawn_timer < POWERUP_SPAWN_INTERVAL {
        return;
    }
    game_state.powerup_spawn_timer = 0.0;

    let kind = random_kind();
    let label = format!("powerup{}", game_state.powerup_counter);
    game_state.powerup_counter += 1;

    let sprite = engine.add_sprite(label.clone(), preset_for(kind));
    sprite.layer = POWERUP_LAYER;
    sprite.collision = true;
    sprite.scale = 0.7;
    sprite.translation.x = thread_rng().gen_range(SCREEN_RIGHT_MIN..SCREEN_RIGHT_MAX);
    sprite.translation.y = thread_rng().gen_range(POWERUP_Y_MIN..POWERUP_Y_MAX);
}

/// Scroll power-ups left, remove if off-screen.
pub fn update(engine: &mut Engine, game_state: &GameState) {
    let speed = game_state.effective_road_speed();
    let labels: Vec<String> = engine
        .sprites
        .keys()
        .filter(|k| k.starts_with("powerup"))
        .cloned()
        .collect();

    for label in labels {
        if let Some(sprite) = engine.sprites.get_mut(&label) {
            sprite.translation.x -= speed * engine.delta_f32;
            // Gentle float animation
            sprite.translation.y += (game_state.elapsed * 3.0).sin() * 0.5;
            if sprite.translation.x < SCREEN_LEFT {
                engine.sprites.remove(&label);
            }
        }
    }
}

/// Tick down active power-up timers.
pub fn tick_timers(game_state: &mut GameState, dt: f32) {
    if game_state.shield_timer > 0.0 {
        game_state.shield_timer = (game_state.shield_timer - dt).max(0.0);
    }
    if game_state.speed_boost_timer > 0.0 {
        game_state.speed_boost_timer = (game_state.speed_boost_timer - dt).max(0.0);
    }
}

/// Determine the kind of a collected power-up from its sprite label index.
/// We use a simple deterministic mapping: counter mod 3.
pub fn kind_from_label(label: &str) -> PowerUpKind {
    let num: usize = label
        .trim_start_matches("powerup")
        .parse()
        .unwrap_or(0);
    match num % 3 {
        0 => PowerUpKind::Health,
        1 => PowerUpKind::Shield,
        _ => PowerUpKind::SpeedBoost,
    }
}

/// Apply a collected power-up to game state.
pub fn apply(game_state: &mut GameState, kind: PowerUpKind) {
    match kind {
        PowerUpKind::Health => {
            game_state.health = (game_state.health + 1).min(PLAYER_MAX_HEALTH);
        }
        PowerUpKind::Shield => {
            game_state.shield_timer = SHIELD_DURATION;
        }
        PowerUpKind::SpeedBoost => {
            game_state.speed_boost_timer = SPEED_BOOST_DURATION;
        }
    }
}

/// Remove all power-up sprites (used on reset).
pub fn clear(engine: &mut Engine) {
    let labels: Vec<String> = engine
        .sprites
        .keys()
        .filter(|k| k.starts_with("powerup"))
        .cloned()
        .collect();
    for label in labels {
        engine.sprites.remove(&label);
    }
}
