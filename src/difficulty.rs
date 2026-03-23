use crate::constants::*;
use crate::game_state::GameState;

/// Ramp up difficulty over time: increase road speed and spawn more obstacles.
pub fn update(game_state: &mut GameState) {
    // Speed ramp
    if game_state.elapsed - game_state.last_difficulty_bump >= DIFFICULTY_INTERVAL {
        game_state.last_difficulty_bump = game_state.elapsed;
        game_state.road_speed =
            (game_state.road_speed + SPEED_INCREMENT).min(MAX_ROAD_SPEED);
    }

    // Level progression
    let new_level = (game_state.score / LEVEL_UP_SCORE) + 1;
    if new_level > game_state.level {
        game_state.level = new_level;
    }
}
