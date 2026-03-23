use rusty_engine::prelude::*;

use crate::constants::*;
use crate::game_state::GameState;
use crate::powerups;

/// Process all collision events for the current frame.
/// Returns true if the player just died.
pub fn handle(engine: &mut Engine, game_state: &mut GameState) -> bool {
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player1") || event.state.is_end() {
            continue;
        }

        // Identify the other sprite
        let other = if event.pair.0 == "player1" {
            &event.pair.1
        } else {
            &event.pair.0
        };

        // Power-up collision
        if other.starts_with("powerup") {
            let kind = powerups::kind_from_label(other);
            powerups::apply(game_state, kind);
            engine.sprites.remove(other);
            engine.audio_manager.play_sfx(SfxPreset::Jingle1, SFX_VOLUME);
            continue;
        }

        // Obstacle collision
        if other.starts_with("obstacle") {
            if game_state.has_shield() {
                // Shield absorbs the hit
                engine.audio_manager.play_sfx(SfxPreset::Impact1, SFX_VOLUME * 0.5);
                continue;
            }

            if game_state.health > 0 {
                game_state.health -= 1;
                engine.audio_manager.play_sfx(SfxPreset::Impact3, SFX_VOLUME);
            }
        }
    }

    game_state.health == 0
}
