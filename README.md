# 🎯 Crosshair Overlay

A customizable and transparent crosshair overlay for games, developed in Rust with advanced visual effects and real-time configuration.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## ✨ Features

### 🎨 Crosshair Styles
- **Classic** : Traditional straight lines with configurable gap
- **Dot** : Simple and discreet center point
- **Circle** : Filled or outline circle
- **T-Shape** : T-shaped with gap
- **X-Shape** : Diagonal cross
- **Square** : Square with center gap
- **Diamond** : Diamond with gap

### 🌈 Visual Effects
- **Pulse** : Pulsing/blinking effect
- **Shake** : Dynamic trembling
- **Rainbow** : Rainbow color rotation
- **Mouse Effects** : Interactive mouse click effects
  - Gap Effect : Expands/contracts crosshair on click
  - Visibility Effect : Hides parts of the crosshair

### 🔧 Advanced Customization
- **Transparency** : Configurable alpha for crosshair and effects
- **Rotation** : Free rotation from 0° to 360°
- **Colors** : Full hexadecimal color support
- **Center dot** : Independently configurable center dot
- **Outline** : Customizable black outline
- **Positioning** : Automatic centering or manual position

### 🚀 Technical Features
- **Click-through** : Clicks pass through the overlay
- **Always on top** : Stays above all windows
- **Optimized performance** : Limited to 30 FPS for animations
- **Hot reload** : Press F5 to reload configuration
- **Transparent** : Completely transparent window

## 📦 Installation

### Prerequisites
- Windows 10/11
- Rust 1.70+ (for compiling from source)

### Option 1: Download binaries
1. Download the latest version from [Releases](../../releases)
2. Extract the archive
3. Run `crosshair-overlay.exe`

### Option 2: Compile from source
```bash
# Clone the repository
git clone https://github.com/your-username/crosshair-overlay.git
cd crosshair-overlay

# Compile in release mode
cargo build --release

# Run the application
./target/release/crosshair-overlay.exe
```

## 🎮 Usage

### Quick start
1. Run `crosshair-overlay.exe`
2. A green crosshair appears at the center of the screen
3. Press **F5** to reload configuration
4. Press **Escape** to quit

### Configuration

The application uses the `config.toml` file for configuration. If the file doesn't exist, it will be created automatically with default settings.

#### Basic configuration example:
```toml
[crosshair]
size = 25
thickness = 2
gap = 5
color = "#00FF00"  # Green
alpha = 1.0
style = "classic"

[effects.rainbow]
enabled = true
speed = 1.0
```

#### Recommended presets:

**Gaming Discrete:**
```toml
[crosshair]
style = "classic"
size = 15
thickness = 1
gap = 3
alpha = 0.8
color = "#FFFFFF"

[crosshair.outline]
enabled = true
```

**Neon Style:**
```toml
[crosshair]
style = "circle"
size = 20
color = "#00FFFF"

[effects.rainbow]
enabled = true
speed = 0.5
```

**Sniper Dot:**
```toml
[crosshair]
style = "dot"
size = 3
color = "#FF0000"

[crosshair.center_dot]
enabled = false
```

### Controls
- **F5** : Reload configuration
- **Escape** : Quit application
- **Mouse click** : Trigger mouse effects (if enabled)

## ⚙️ Detailed Configuration

### config.toml file structure

```toml
[crosshair]
size = 25              # Size in pixels
thickness = 2          # Line thickness
gap = 5               # Center gap
color = "#00FF00"     # Hex color
alpha = 1.0           # Transparency (0.0-1.0)
rotation = 0          # Rotation in degrees
style = "classic"     # Style (see list below)
triangle_bars = false # Triangles instead of rectangles (classic)
filled_circle = false # Filled circle (circle)

[crosshair.center_dot]
enabled = true
size = 2
color = "#FF0000"
alpha = 1.0

[crosshair.outline]
enabled = false
thickness = 1
color = "#000000"
alpha = 0.8

[window]
size = 300            # Window size
opacity = 1.0         # Global opacity

[window.position]
center_screen = true  # Automatic centering
follow_cursor = false # Follow cursor
offset_x = 0         # X offset
offset_y = 0         # Y offset

[effects.pulse]
enabled = false
speed = 2.0          # Hz
min_alpha = 0.3
max_alpha = 1.0

[effects.shake]
enabled = false
intensity = 1.0      # Pixels
speed = 1.0         # Hz

[effects.rainbow]
enabled = false
speed = 1.0         # Hz
saturation = 1.0
brightness = 1.0

[effects.mouse]
enabled = true

[effects.mouse.gap_effect]
enabled = true
mode = "expand"      # "expand", "contract", "toggle"
intensity = 5        # Multiplier

[effects.mouse.visibility_effect]
enabled = false
hide_mode = "center" # "full", "left", "right", "top", "bottom", etc.
fade_percentage = 0.2
```

### Available styles
- `"classic"` - Traditional crosshair
- `"dot"` - Simple point
- `"circle"` - Circle
- `"t"` - T-shape
- `"x"` - Diagonal cross
- `"square"` - Square
- `"diamond"` - Diamond

## 🛠️ Development

### Architecture
- **Rust** with `winit` for window management
- **softbuffer** for software rendering
- **serde + toml** for configuration
- **Windows API** for system features

### Project structure
```
src/
├── main.rs              # Main entry point
├── config/              # Configuration management
│   ├── mod.rs
│   ├── effects/         # Visual effects
│   └── window.rs        # Window configuration
└── crosshair/           # Crosshair styles
    ├── mod.rs
    ├── classic.rs
    ├── circle.rs
    └── ...
```

### Contributing
1. Fork the project
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤝 Support

- 🐛 **Bugs** : Open an [issue](../../issues)
- 💡 **Suggestions** : Open a [discussion](../../discussions)
- 📧 **Contact** : OsefCode@gmail.com

## 🏆 Credits

Developed with ❤️ in Rust by Glubus

---

⭐ **If you like this project, please give it a star!** 
