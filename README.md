# Race Car

2D top-down racing game built in Rust using the [rusty_engine](https://github.com/CleanCut/rusty_engine) game engine — dodge obstacles, survive as long as possible.

## Gameplay

- Dodge barrels and cones scrolling towards you
- Each collision costs 1 health point (start with 5)
- Hit 0 health → Game Over
- Go out of bounds → instant Game Over

**Controls**: `↑` `↓` to move the car up/down

## Features

- Sprite-based 2D rendering with collision detection
- Background music + sound effects (OGG)
- Randomised obstacle spawning and recycling
- Health HUD displayed on screen
- Delta-time movement for consistent speed at any frame rate

## Build & Run

```bash
# Requires Rust toolchain (https://rustup.rs)
cargo run --release
```

## Tech Stack

- **Rust** 2021 edition
- **rusty_engine** 5.2 — 2D game engine (sprites, audio, collisions, input)
- **rand** — random obstacle placement

## Project Structure

```
src/
└── main.rs        # Game setup, obstacle/road logic, collision handling
assets/
├── sprite/racing/ # Car, barrel, barrier, cone sprites + colliders
└── audio/
    ├── music/     # Background tracks (OGG)
    └── sfx/       # Sound effects (impact, jingle, etc.)
```
