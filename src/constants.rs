// Game-wide constants

// Player
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_START_X: f32 = -500.0;
pub const PLAYER_LAYER: f32 = 10.0;
pub const PLAYER_INITIAL_HEALTH: u8 = 5;
pub const PLAYER_MAX_HEALTH: u8 = 8;
pub const PLAYER_TILT_ANGLE: f32 = 0.15;

// Bounds
pub const BOUND_TOP: f32 = 360.0;
pub const BOUND_BOTTOM: f32 = -360.0;
pub const SCREEN_LEFT: f32 = -800.0;
pub const SCREEN_RIGHT_MIN: f32 = 800.0;
pub const SCREEN_RIGHT_MAX: f32 = 1600.0;

// Road
pub const ROAD_LINE_COUNT: usize = 100;
pub const ROAD_LINE_SCALE: f32 = 0.1;
pub const ROAD_LINE_START_X: f32 = -600.0;
pub const ROAD_LINE_SPACING: f32 = 150.0;
pub const ROAD_LINE_WRAP_LEFT: f32 = -670.9;
pub const ROAD_LINE_WRAP_OFFSET: f32 = 1500.0;

// Base speeds (modified by difficulty)
pub const BASE_ROAD_SPEED: f32 = 400.0;
pub const MAX_ROAD_SPEED: f32 = 900.0;

// Obstacles
pub const INITIAL_OBSTACLE_COUNT: usize = 3;
pub const MAX_OBSTACLE_COUNT: usize = 12;
pub const OBSTACLE_Y_MIN: f32 = -300.0;
pub const OBSTACLE_Y_MAX: f32 = 300.0;
pub const OBSTACLE_LAYER: f32 = 5.0;

// Power-ups
pub const POWERUP_SPAWN_INTERVAL: f32 = 8.0;
pub const POWERUP_LAYER: f32 = 7.0;
pub const POWERUP_Y_MIN: f32 = -250.0;
pub const POWERUP_Y_MAX: f32 = 250.0;
pub const SHIELD_DURATION: f32 = 5.0;
pub const SPEED_BOOST_DURATION: f32 = 4.0;
pub const SPEED_BOOST_MULTIPLIER: f32 = 1.6;

// Difficulty
pub const DIFFICULTY_INTERVAL: f32 = 10.0;
pub const SPEED_INCREMENT: f32 = 30.0;
pub const OBSTACLE_ADD_INTERVAL: f32 = 15.0;

// Scoring
pub const SCORE_PER_SECOND: f32 = 10.0;
pub const DODGE_BONUS: u32 = 25;
pub const LEVEL_UP_SCORE: u32 = 500;

// HUD positions
pub const HUD_HEALTH_POS: (f32, f32) = (550.0, 320.0);
pub const HUD_SCORE_POS: (f32, f32) = (-550.0, 320.0);
pub const HUD_LEVEL_POS: (f32, f32) = (0.0, 320.0);
pub const HUD_HIGHSCORE_POS: (f32, f32) = (-550.0, 280.0);
pub const HUD_SHIELD_POS: (f32, f32) = (550.0, 280.0);
pub const HUD_SPEED_POS: (f32, f32) = (550.0, 240.0);

// Music
pub const MUSIC_VOLUME: f32 = 0.2;
pub const SFX_VOLUME: f32 = 0.5;
