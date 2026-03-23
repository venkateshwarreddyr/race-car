use crate::constants::*;
use crate::game_state::GameState;

/// Accumulate score from survival time and dodge bonuses.
pub fn update(game_state: &mut GameState, dt: f32, obstacles_dodged: u32) {
    // Time-based score
    game_state.score += (SCORE_PER_SECOND * dt) as u32;

    // Dodge bonus
    if obstacles_dodged > 0 {
        game_state.score += obstacles_dodged * DODGE_BONUS;
        game_state.obstacles_dodged += obstacles_dodged;
    }

    // Track high score
    if game_state.score > game_state.high_score {
        game_state.high_score = game_state.score;
    }
}
