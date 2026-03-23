use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::GameState;

/// Lay down the scrolling road-line sprites.
pub fn setup(game: &mut Game<GameState>) {
    for i in 0..ROAD_LINE_COUNT {
        let roadline = game.add_sprite(
            format!("roadline{}", i),
            SpritePreset::RacingBarrierWhite,
        );
        roadline.scale = ROAD_LINE_SCALE;
        roadline.translation.x = ROAD_LINE_START_X + ROAD_LINE_SPACING * i as f32;
        roadline.layer = 1.0;
    }
}

/// Scroll road lines to the left; wrap around when off-screen.
pub fn update(engine: &mut Engine, game_state: &GameState) {
    let speed = game_state.effective_road_speed();
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= speed * engine.delta_f32;
            if sprite.translation.x < ROAD_LINE_WRAP_LEFT {
                sprite.translation.x += ROAD_LINE_WRAP_OFFSET;
            }
        }
    }
}
