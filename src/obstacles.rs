use rand::prelude::*;
use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::GameState;

/// All available obstacle sprite presets (more variety than before).
const OBSTACLE_PRESETS: &[SpritePreset] = &[
    SpritePreset::RacingBarrelBlue,
    SpritePreset::RacingBarrelRed,
    SpritePreset::RacingConeStraight,
    SpritePreset::RacingBarrierRed,
    SpritePreset::RacingBarrelBlue,
    SpritePreset::RacingBarrelRed,
];

/// Pick a random obstacle preset.
fn random_preset() -> SpritePreset {
    let idx = thread_rng().gen_range(0..OBSTACLE_PRESETS.len());
    OBSTACLE_PRESETS[idx]
}

/// Spawn the initial set of obstacles.
pub fn setup(game: &mut Game<GameState>) {
    for i in 0..INITIAL_OBSTACLE_COUNT {
        spawn_obstacle(game, i);
    }
}

/// Add a single obstacle sprite at a random off-screen position.
fn spawn_obstacle(game: &mut Game<GameState>, index: usize) {
    let preset = random_preset();
    let obstacle = game.add_sprite(format!("obstacle{}", index), preset);
    obstacle.layer = OBSTACLE_LAYER;
    obstacle.collision = true;
    obstacle.translation.x = thread_rng().gen_range(SCREEN_RIGHT_MIN..SCREEN_RIGHT_MAX);
    obstacle.translation.y = thread_rng().gen_range(OBSTACLE_Y_MIN..OBSTACLE_Y_MAX);
}

/// Scroll obstacles left; recycle when they leave the screen.
/// Returns the number of obstacles that were recycled this frame (dodged).
pub fn update(engine: &mut Engine, game_state: &GameState) -> u32 {
    let speed = game_state.effective_road_speed();
    let mut dodged = 0u32;

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= speed * engine.delta_f32;
            if sprite.translation.x < SCREEN_LEFT {
                sprite.translation.x = thread_rng().gen_range(SCREEN_RIGHT_MIN..SCREEN_RIGHT_MAX);
                sprite.translation.y = thread_rng().gen_range(OBSTACLE_Y_MIN..OBSTACLE_Y_MAX);
                dodged += 1;
            }
        }
    }

    dodged
}

/// Dynamically add more obstacles as difficulty increases.
pub fn add_more(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.obstacle_count < MAX_OBSTACLE_COUNT
        && game_state.elapsed - game_state.last_obstacle_add >= OBSTACLE_ADD_INTERVAL
    {
        game_state.last_obstacle_add = game_state.elapsed;
        let idx = game_state.obstacle_count;
        game_state.obstacle_count += 1;

        let preset = random_preset();
        let obstacle = engine.add_sprite(format!("obstacle{}", idx), preset);
        obstacle.layer = OBSTACLE_LAYER;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(SCREEN_RIGHT_MIN..SCREEN_RIGHT_MAX);
        obstacle.translation.y = thread_rng().gen_range(OBSTACLE_Y_MIN..OBSTACLE_Y_MAX);
    }
}

/// Remove all obstacle sprites (used on reset).
pub fn clear(engine: &mut Engine) {
    let labels: Vec<String> = engine
        .sprites
        .keys()
        .filter(|k| k.starts_with("obstacle"))
        .cloned()
        .collect();
    for label in labels {
        engine.sprites.remove(&label);
    }
}

/// Re-spawn initial obstacles after a reset.
pub fn respawn(engine: &mut Engine, count: usize) {
    for i in 0..count {
        let preset = random_preset();
        let obstacle = engine.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = OBSTACLE_LAYER;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(SCREEN_RIGHT_MIN..SCREEN_RIGHT_MAX);
        obstacle.translation.y = thread_rng().gen_range(OBSTACLE_Y_MIN..OBSTACLE_Y_MAX);
    }
}
