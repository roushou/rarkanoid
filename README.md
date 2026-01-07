# Rarkanoid

An Arkanoid/Breakout clone built with Rust and Raylib with sounds, particle effects and screen shake.

## Features

- **Smooth Controls** - Keyboard (A/D, Arrow Keys) and mouse support with lerped movement
- **Ball Physics** - Angle-based reflection off paddle, speed increases over time
- **Brick Types** - Normal, Hard (2 hits with crack visuals), and Indestructible
- **Visual Polish** - Particle explosions, screen shake, ball trail, gradient background
- **Combo System** - Score multiplier for consecutive hits
- **3 Levels** - Progressive difficulty with different brick layouts
- **Sound Support** - Ready for custom sound effects

## Getting started

### Prerequisites

- CMake

```bash
# macOS
brew install cmake

# Ubuntu/Debian
sudo apt install cmake

# Windows
winget install cmake
```

## Controls

| Input | Action |
|-------|--------|
| A / Left Arrow | Move paddle left |
| D / Right Arrow | Move paddle right |
| Mouse | Move paddle |
| Space / Click | Launch ball / Start game |
| ESC / P | Pause |

## Adding Sound Effects

Place `.wav` files in `assets/sounds/`:

```
assets/sounds/
├── paddle_hit.wav
├── brick_hit.wav
├── brick_destroy.wav
├── lose_life.wav
└── level_complete.wav
```

Generate retro sounds at [sfxr.me](https://sfxr.me):
- **Pickup/Coin** → paddle_hit.wav
- **Hit/Hurt** → brick_hit.wav
- **Explosion** → brick_destroy.wav
- **Hit/Hurt** (lower) → lose_life.wav
- **Powerup** → level_complete.wav

## License

[MIT](./LICENSE)
