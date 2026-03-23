mod collisions;
mod constants;
mod difficulty;
mod game_state;
mod hud;
mod obstacles;
mod player;
mod powerups;
mod road;
mod scoring;

use rusty_engine::prelude::*;

use constants::*;
use game_state::{GameState, Phase};

fn main() {
    let mut game = Game::new();

    // Setup all subsystems
    player::setup(&mut game);
    road::setup(&mut game);
    obstacles::setup(&mut game);
    hud::setup(&mut game);

    // Background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, MUSIC_VOLUME);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let dt = engine.delta_f32;

    // ── Phase transitions via keyboard ──────────────────────────────
    handle_input(engine, game_state);

    match game_state.phase {
        Phase::Menu => {
            hud::update_overlays(engine, game_state);
            return;
        }
        Phase::Paused => {
            hud::update_overlays(engine, game_state);
            return;
        }
        Phase::GameOver => {
            hud::update_overlays(engine, game_state);
            return;
        }
        Phase::Playing => {}
    }

    // ── Active gameplay ─────────────────────────────────────────────
    game_state.elapsed += dt;

    // Player movement
    let out_of_bounds = player::update(engine, game_state);
    if out_of_bounds {
        game_state.health = 0;
    }

    // Road scrolling
    road::update(engine, game_state);

    // Obstacles
    let dodged = obstacles::update(engine, game_state);
    obstacles::add_more(engine, game_state);

    // Power-ups
    powerups::spawn(engine, game_state);
    powerups::update(engine, game_state);
    powerups::tick_timers(game_state, dt);

    // Collisions
    let died = collisions::handle(engine, game_state);

    // Scoring
    scoring::update(game_state, dt, dodged);

    // Difficulty
    difficulty::update(game_state);

    // HUD
    hud::update(engine, game_state);
    hud::update_overlays(engine, game_state);

    // Check death
    if died || out_of_bounds {
        trigger_game_over(engine, game_state);
    }
}

/// Handle keyboard input for phase transitions.
fn handle_input(engine: &mut Engine, game_state: &mut GameState) {
    // Enter → start from menu
    if game_state.phase == Phase::Menu
        && engine.keyboard_state.just_pressed(KeyCode::Return)
    {
        game_state.phase = Phase::Playing;
    }

    // P → toggle pause
    if engine.keyboard_state.just_pressed(KeyCode::P) {
        match game_state.phase {
            Phase::Playing => game_state.phase = Phase::Paused,
            Phase::Paused => game_state.phase = Phase::Playing,
            _ => {}
        }
    }

    // R → restart (from game over or during play)
    if engine.keyboard_state.just_pressed(KeyCode::R)
        && (game_state.phase == Phase::GameOver || game_state.phase == Phase::Playing)
    {
        restart_game(engine, game_state);
    }
}

/// Transition to game-over state.
fn trigger_game_over(engine: &mut Engine, game_state: &mut GameState) {
    game_state.phase = Phase::GameOver;
    if game_state.score > game_state.high_score {
        game_state.high_score = game_state.score;
    }
    engine.audio_manager.stop_music();
    engine.audio_manager.play_sfx(SfxPreset::Jingle3, SFX_VOLUME);
}

/// Reset everything for a fresh round.
fn restart_game(engine: &mut Engine, game_state: &mut GameState) {
    // Clear dynamic sprites
    obstacles::clear(engine);
    powerups::clear(engine);

    // Reset player position
    if let Some(player) = engine.sprites.get_mut("player1") {
        player.translation.x = PLAYER_START_X;
        player.translation.y = 0.0;
        player.rotation = 0.0;
        player.scale = 1.0;
    }

    // Reset state (preserves high score)
    game_state.reset();

    // Respawn obstacles
    obstacles::respawn(engine, INITIAL_OBSTACLE_COUNT);

    // Restart music
    engine
        .audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, MUSIC_VOLUME);
}
