use crate::constants::*;

/// Phases the game can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Menu,
    Playing,
    Paused,
    GameOver,
}

/// Types of power-ups that can spawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpKind {
    Health,
    Shield,
    SpeedBoost,
}

/// All mutable game state lives here.
pub struct GameState {
    pub phase: Phase,
    pub health: u8,
    pub score: u32,
    pub high_score: u32,
    pub level: u32,

    // Difficulty
    pub road_speed: f32,
    pub elapsed: f32,
    pub last_difficulty_bump: f32,
    pub last_obstacle_add: f32,
    pub obstacle_count: usize,

    // Power-ups
    pub shield_timer: f32,
    pub speed_boost_timer: f32,
    pub powerup_spawn_timer: f32,
    pub powerup_counter: usize,

    // Tracking
    pub obstacles_dodged: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            phase: Phase::Menu,
            health: PLAYER_INITIAL_HEALTH,
            score: 0,
            high_score: 0,
            level: 1,
            road_speed: BASE_ROAD_SPEED,
            elapsed: 0.0,
            last_difficulty_bump: 0.0,
            last_obstacle_add: 0.0,
            obstacle_count: INITIAL_OBSTACLE_COUNT,
            shield_timer: 0.0,
            speed_boost_timer: 0.0,
            powerup_spawn_timer: 0.0,
            powerup_counter: 0,
            obstacles_dodged: 0,
        }
    }
}

impl GameState {
    /// Reset for a new round, preserving high score.
    pub fn reset(&mut self) {
        let hs = self.high_score;
        *self = Self::default();
        self.high_score = hs;
        self.phase = Phase::Playing;
    }

    pub fn has_shield(&self) -> bool {
        self.shield_timer > 0.0
    }

    pub fn has_speed_boost(&self) -> bool {
        self.speed_boost_timer > 0.0
    }

    pub fn effective_player_speed(&self) -> f32 {
        if self.has_speed_boost() {
            PLAYER_SPEED * SPEED_BOOST_MULTIPLIER
        } else {
            PLAYER_SPEED
        }
    }

    pub fn effective_road_speed(&self) -> f32 {
        self.road_speed
    }
}
