use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::{GameState, Phase};

/// Create all HUD text elements.
pub fn setup(game: &mut Game<GameState>) {
    // Health
    let health = game.add_text("health_message", "Health: 5");
    health.translation = Vec2::new(HUD_HEALTH_POS.0, HUD_HEALTH_POS.1);
    health.font_size = 24.0;

    // Score
    let score = game.add_text("score_message", "Score: 0");
    score.translation = Vec2::new(HUD_SCORE_POS.0, HUD_SCORE_POS.1);
    score.font_size = 24.0;

    // Level
    let level = game.add_text("level_message", "Level: 1");
    level.translation = Vec2::new(HUD_LEVEL_POS.0, HUD_LEVEL_POS.1);
    level.font_size = 24.0;

    // High score
    let hs = game.add_text("highscore_message", "High Score: 0");
    hs.translation = Vec2::new(HUD_HIGHSCORE_POS.0, HUD_HIGHSCORE_POS.1);
    hs.font_size = 20.0;

    // Shield indicator
    let shield = game.add_text("shield_message", "");
    shield.translation = Vec2::new(HUD_SHIELD_POS.0, HUD_SHIELD_POS.1);
    shield.font_size = 20.0;

    // Speed boost indicator
    let spd = game.add_text("speed_message", "");
    spd.translation = Vec2::new(HUD_SPEED_POS.0, HUD_SPEED_POS.1);
    spd.font_size = 20.0;

    // Title / menu text
    let title = game.add_text("title_text", "RACE CAR");
    title.translation = Vec2::new(0.0, 80.0);
    title.font_size = 96.0;

    let subtitle = game.add_text("subtitle_text", "Press ENTER to Start");
    subtitle.translation = Vec2::new(0.0, -40.0);
    subtitle.font_size = 32.0;

    let controls = game.add_text(
        "controls_text",
        "UP/DOWN to move | P to pause | R to restart",
    );
    controls.translation = Vec2::new(0.0, -100.0);
    controls.font_size = 20.0;
}

/// Update all HUD text each frame.
pub fn update(engine: &mut Engine, game_state: &GameState) {
    // Health
    if let Some(t) = engine.texts.get_mut("health_message") {
        t.value = format!("Health: {}", game_state.health);
    }

    // Score
    if let Some(t) = engine.texts.get_mut("score_message") {
        t.value = format!("Score: {}", game_state.score);
    }

    // Level
    if let Some(t) = engine.texts.get_mut("level_message") {
        t.value = format!("Level: {}", game_state.level);
    }

    // High score
    if let Some(t) = engine.texts.get_mut("highscore_message") {
        t.value = format!("High Score: {}", game_state.high_score);
    }

    // Shield
    if let Some(t) = engine.texts.get_mut("shield_message") {
        if game_state.has_shield() {
            t.value = format!("SHIELD: {:.1}s", game_state.shield_timer);
        } else {
            t.value = String::new();
        }
    }

    // Speed boost
    if let Some(t) = engine.texts.get_mut("speed_message") {
        if game_state.has_speed_boost() {
            t.value = format!("BOOST: {:.1}s", game_state.speed_boost_timer);
        } else {
            t.value = String::new();
        }
    }
}

/// Show / hide menu and game-over overlays based on phase.
pub fn update_overlays(engine: &mut Engine, game_state: &GameState) {
    let in_menu = game_state.phase == Phase::Menu;
    let in_game_over = game_state.phase == Phase::GameOver;
    let is_paused = game_state.phase == Phase::Paused;

    // Title
    if let Some(t) = engine.texts.get_mut("title_text") {
        if in_game_over {
            t.value = "GAME OVER".to_string();
            t.font_size = 96.0;
        } else if is_paused {
            t.value = "PAUSED".to_string();
            t.font_size = 96.0;
        } else if in_menu {
            t.value = "RACE CAR".to_string();
            t.font_size = 96.0;
        } else {
            t.value = String::new();
        }
    }

    // Subtitle
    if let Some(t) = engine.texts.get_mut("subtitle_text") {
        if in_game_over {
            t.value = format!("Final Score: {} | Press R to Restart", game_state.score);
        } else if is_paused {
            t.value = "Press P to Resume".to_string();
        } else if in_menu {
            t.value = "Press ENTER to Start".to_string();
        } else {
            t.value = String::new();
        }
    }

    // Controls help
    if let Some(t) = engine.texts.get_mut("controls_text") {
        if in_menu {
            t.value = "UP/DOWN to move | P to pause | R to restart".to_string();
        } else {
            t.value = String::new();
        }
    }
}
