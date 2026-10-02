## Summary

This PR introduces a complete **ASCII Animation System** to augusto and sets up the project for **Hacktoberfest 2026**.

### 🎬 New Features

#### ASCII Animations (`augusto animate`)
Added 6 animated 3D shapes with embedded text:
- **donut/torus** — Classic rotating torus with text on surface
- **cube** — 3D wireframe cube rotating on all axes  
- **cube5d/hypercube/tesseract** — 4D hypercube projected to 3D→2D
- **sphere** — Wireframe sphere with latitude/longitude lines
- **mandala** — Hypnotic rotational symmetry patterns
- **pyramid** — 3D pyramid with text on faces

**CLI Options:**
- `--speed <ms>` — Frame delay (default: 100ms)
- `--frames <n>` — Number of frames (default: infinite)
- `--color` — Enable ANSI color output
- `--filler <word>` — Custom filler characters
- `--width/--height` — Terminal dimensions

#### Hacktoberfest 2026 Ready
- **8 issue templates** for contributors (shapes, color, export, interactive, particles, Mallarmé themes)
- **25 GitHub labels** configured (difficulty, type, area, status)
- Updated README with participation guide and quick-start

### 📁 Files Changed
- `augusto/src/animation.rs` — New module (808 lines, 10 tests)
- `augusto/src/main.rs` — Added animate command
- `README.md` — Comprehensive documentation + examples
- `.github/ISSUE_TEMPLATE/*.yml` — 8 templates
- `.github/labels.yml` — 25 labels

### 🧪 Testing
```bash
cd augusto
cargo test          # All tests pass including 10 new animation tests
cargo run -- animate donut "RUST"
cargo run -- animate cube "CODE" --speed 50
cargo run -- animate cube5d "HACKTOBERFEST" --color
```