use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::GameState;

/// Create the player sprite.
pub fn setup(game: &mut Game<GameState>) {
    let player = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player.translation.x = PLAYER_START_X;
    player.layer = PLAYER_LAYER;
    player.collision = true;
}

/// Move player based on keyboard input; returns true if out of bounds.
pub fn update(engine: &mut Engine, game_state: &GameState) -> bool {
    let mut direction = 0.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let speed = game_state.effective_player_speed();
    let player = engine.sprites.get_mut("player1").unwrap();
    player.translation.y += direction * speed * engine.delta_f32;
    player.rotation = direction * PLAYER_TILT_ANGLE;

    // Shield visual feedback — slight scale pulse
    if game_state.has_shield() {
        let pulse = 1.0 + 0.05 * (game_state.shield_timer * 6.0).sin();
        player.scale = pulse;
    } else {
        player.scale = 1.0;
    }

    // Bounds check
    player.translation.y < BOUND_BOTTOM || player.translation.y > BOUND_TOP
}
