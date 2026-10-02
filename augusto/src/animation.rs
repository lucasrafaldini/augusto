//! ASCII Animation Module
//!
//! This module provides animated ASCII art of 3D shapes including donuts, cubes,
//! hypercubes, spheres, mandalas, and pyramids with embedded text.
//!
//! # Features
//!
//! - **Donut/Torus**: Classic rotating torus with text on surface
//! - **Cube**: 3D wireframe cube rotating on all axes
//! - **5D Cube (Tesseract)**: 4D hypercube projected to 3D then 2D
//! - **Sphere**: Wireframe sphere with latitude/longitude lines
//! - **Mandala**: Hypnotic rotational symmetry patterns
//! - **Pyramid**: 3D pyramid with text on faces
//!
//! # Example
//!
//! ```rust
//! use augusto::animation::{AnimationConfig, AnimationShape, animate};
//!
//! let config = AnimationConfig {
//!     shape: AnimationShape::Donut,
//!     word: "RUST".to_string(),
//!     filler: Some("code".to_string()),
//!     speed_ms: 100,
//!     frames: None, // infinite
//!     color: false,
//!     width: 80,
//!     height: 40,
//! };
//!
//! animate(config);
//! ```

use std::f32::consts::PI;
use std::thread;
use std::time::{Duration, Instant};

/// 3D vector for calculations
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x,
            y: self.y * cos_a - self.z * sin_a,
            z: self.y * sin_a + self.z * cos_a,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x * cos_a + self.z * sin_a,
            y: self.y,
            z: -self.x * sin_a + self.z * cos_a,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
            z: self.z,
        }
    }

    pub fn rotate(&self, rx: f32, ry: f32, rz: f32) -> Self {
        self.rotate_x(rx).rotate_y(ry).rotate_z(rz)
    }
}

/// 2D point for screen projection
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Represents a 3D edge (line between two points)
#[derive(Debug, Clone)]
pub struct Edge {
    pub start: Vec3,
    pub end: Vec3,
}

/// Animation shape types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationShape {
    Donut,
    Cube,
    Cube5D,
    Sphere,
    Mandala,
    Pyramid,
}

impl AnimationShape {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "donut" | "torus" => Some(Self::Donut),
            "cube" => Some(Self::Cube),
            "cube5d" | "hypercube" | "tesseract" | "5d" => Some(Self::Cube5D),
            "sphere" => Some(Self::Sphere),
            "mandala" => Some(Self::Mandala),
            "pyramid" => Some(Self::Pyramid),
            _ => None,
        }
    }
}

/// Configuration for animation
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub shape: AnimationShape,
    pub word: String,
    pub filler: Option<String>,
    pub speed_ms: u64,
    pub frames: Option<usize>,
    pub color: bool,
    pub width: usize,
    pub height: usize,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            shape: AnimationShape::Donut,
            word: "RUST".to_string(),
            filler: None,
            speed_ms: 100,
            frames: None,
            color: false,
            width: 80,
            height: 40,
        }
    }
}

/// Color palette for ANSI colors
const COLORS: &[&str] = &[
    "\x1b[38;5;196m", // Red
    "\x1b[38;5;202m", // Orange
    "\x1b[38;5;226m", // Yellow
    "\x1b[38;5;46m",  // Green
    "\x1b[38;5;21m",  // Blue
    "\x1b[38;5;93m",  // Purple
    "\x1b[38;5;201m", // Pink
    "\x1b[38;5;51m",  // Cyan
];

const RESET: &str = "\x1b[0m";

/// Background color for the AUGUSTO title frame
const BG_COLOR: &str = "\x1b[48;5;234m"; // Dark gray background

/// Add a decorative coin-style frame around the animation output
/// with "AUGUSTO" repeated on all four borders like Greek/Roman coins
fn add_frame(output: &str, width: usize, _config: &AnimationConfig) -> String {
    const COIN_TEXT: &str = "AUGUSTO";
    const COIN_LEN: usize = 7; // "AUGUSTO".len()
    
    let border_bg = format!("{}", BG_COLOR);
    let border_fg = "\x1b[38;5;231m"; // White text for border
    let reset = RESET;
    
    // Create top/bottom border with repeated "AUGUSTO"
    let repeat_count = (width + COIN_LEN - 1) / COIN_LEN;
    let coin_pattern: String = COIN_TEXT.repeat(repeat_count);
    let coin_pattern = &coin_pattern[..width.min(coin_pattern.len())];
    
    let top_border = format!("{}{}{}╭{}╮{}\n", border_bg, border_fg, " ".repeat(0), coin_pattern, reset);
    let bottom_border = format!("{}{}{}╰{}╯{}\n", border_bg, border_fg, " ".repeat(0), coin_pattern, reset);
    
    // Side borders - vertical "AUGUSTO" 
    let side_text = COIN_TEXT;
    
    let mut framed = String::new();
    framed.push_str(&top_border);
    
    // Add side borders to each line of the animation
    let lines: Vec<&str> = output.lines().collect();
    let anim_height = lines.len();
    
    for (i, line) in lines.iter().enumerate() {
        let visible_width = strip_ansi_codes(line).chars().count();
        let padding = if visible_width < width { width - visible_width } else { 0 };
        
        // Get character for left/right border (cycle through AUGUSTO)
        let left_char = side_text.chars().nth(i % COIN_LEN).unwrap_or('A');
        let right_char = side_text.chars().nth((COIN_LEN - 1 - (i % COIN_LEN)) % COIN_LEN).unwrap_or('O');
        
        let left_border = format!("{}{}{}{}", border_bg, border_fg, left_char, reset);
        let right_border = format!("{}{}{}{}", border_bg, border_fg, right_char, reset);
        
        let padded_line = format!("{}│{}{}│{}\n", left_border, line, " ".repeat(padding), right_border);
        framed.push_str(&padded_line);
    }
    
    framed.push_str(&bottom_border);
    
    framed
}

/// Strip ANSI escape codes from a string for width calculation
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if let Some(&'[') = chars.peek() {
                chars.next(); // consume '['
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Project 3D point to 2D screen coordinates
fn project(point: &Vec3, width: f32, height: f32, fov: f32, distance: f32) -> Vec2 {
    let factor = fov / (distance + point.z);
    Vec2 {
        x: width / 2.0 + point.x * factor * width / 2.0,
        y: height / 2.0 - point.y * factor * height / 2.0,
    }
}

/// Bresenham's line algorithm for drawing lines in ASCII
fn draw_line(buffer: &mut [Vec<char>], x0: i32, y0: i32, x1: i32, y1: i32, ch: char) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let height = buffer.len() as i32;
    let width = if height > 0 {
        buffer[0].len() as i32
    } else {
        0
    };

    loop {
        if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
            buffer[y0 as usize][x0 as usize] = ch;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Generate donut (torus) geometry
fn generate_donut(r_major: f32, r_minor: f32, segments: usize, rings: usize) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..segments {
        let u = 2.0 * PI * i as f32 / segments as f32;
        let cos_u = u.cos();
        let sin_u = u.sin();

        for j in 0..rings {
            let v = 2.0 * PI * j as f32 / rings as f32;
            let cos_v = v.cos();
            let sin_v = v.sin();

            // Point on torus surface
            let x = (r_major + r_minor * cos_v) * cos_u;
            let y = (r_major + r_minor * cos_v) * sin_u;
            let z = r_minor * sin_v;

            let current = Vec3::new(x, y, z);

            // Edge along u direction (next segment)
            let u_next = 2.0 * PI * ((i + 1) % segments) as f32 / segments as f32;
            let cos_u_next = u_next.cos();
            let sin_u_next = u_next.sin();
            let x_next = (r_major + r_minor * cos_v) * cos_u_next;
            let y_next = (r_major + r_minor * cos_v) * sin_u_next;
            let z_next = r_minor * sin_v;
            edges.push(Edge {
                start: current,
                end: Vec3::new(x_next, y_next, z_next),
            });

            // Edge along v direction (next ring)
            let v_next = 2.0 * PI * ((j + 1) % rings) as f32 / rings as f32;
            let cos_v_next = v_next.cos();
            let sin_v_next = v_next.sin();
            let x_ring = (r_major + r_minor * cos_v_next) * cos_u;
            let y_ring = (r_major + r_minor * cos_v_next) * sin_u;
            let z_ring = r_minor * sin_v_next;
            edges.push(Edge {
                start: current,
                end: Vec3::new(x_ring, y_ring, z_ring),
            });
        }
    }

    edges
}

/// Generate cube geometry
fn generate_cube(size: f32) -> Vec<Edge> {
    let h = size / 2.0;
    let vertices = [
        Vec3::new(-h, -h, -h), // 0
        Vec3::new(h, -h, -h),  // 1
        Vec3::new(h, h, -h),   // 2
        Vec3::new(-h, h, -h),  // 3
        Vec3::new(-h, -h, h),  // 4
        Vec3::new(h, -h, h),   // 5
        Vec3::new(h, h, h),    // 6
        Vec3::new(-h, h, h),   // 7
    ];

    let edges_idx = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // back face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // front face
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // connecting edges
    ];

    edges_idx
        .iter()
        .map(|&(a, b)| Edge {
            start: vertices[a],
            end: vertices[b],
        })
        .collect()
}

/// Generate 4D hypercube (tesseract) vertices
fn generate_tesseract_vertices(size: f32) -> Vec<[f32; 4]> {
    let h = size / 2.0;
    let mut vertices = Vec::new();
    for i in 0..16 {
        vertices.push([
            if i & 1 != 0 { h } else { -h },
            if i & 2 != 0 { h } else { -h },
            if i & 4 != 0 { h } else { -h },
            if i & 8 != 0 { h } else { -h },
        ]);
    }
    vertices
}

/// Project 4D to 3D (perspective projection)
fn project_4d_to_3d(v: &[f32; 4], distance: f32) -> Vec3 {
    let factor = distance / (distance - v[3]);
    Vec3::new(v[0] * factor, v[1] * factor, v[2] * factor)
}

/// Generate 5D cube (tesseract) edges
fn generate_cube5d(size: f32) -> Vec<Edge> {
    let vertices_4d = generate_tesseract_vertices(size);
    let mut edges = Vec::new();

    // Edges of tesseract: vertices that differ in exactly one coordinate
    for i in 0..16 {
        for dim in 0..4 {
            let j = i ^ (1 << dim);
            if j > i {
                let v1 = project_4d_to_3d(&vertices_4d[i], 3.0);
                let v2 = project_4d_to_3d(&vertices_4d[j], 3.0);
                edges.push(Edge { start: v1, end: v2 });
            }
        }
    }

    edges
}

/// Generate sphere geometry (wireframe with latitude/longitude lines)
fn generate_sphere(radius: f32, lat_segments: usize, lon_segments: usize) -> Vec<Edge> {
    let mut edges = Vec::new();

    for i in 0..=lat_segments {
        let lat = PI * i as f32 / lat_segments as f32;
        let sin_lat = lat.sin();
        let cos_lat = lat.cos();

        for j in 0..lon_segments {
            let lon = 2.0 * PI * j as f32 / lon_segments as f32;
            let sin_lon = lon.sin();
            let cos_lon = lon.cos();

            let x = radius * sin_lat * cos_lon;
            let y = radius * cos_lat;
            let z = radius * sin_lat * sin_lon;
            let current = Vec3::new(x, y, z);

            // Longitude edge (next longitude)
            if i < lat_segments {
                let lat_next = PI * (i + 1) as f32 / lat_segments as f32;
                let sin_lat_next = lat_next.sin();
                let x_next = radius * sin_lat_next * cos_lon;
                let y_next = radius * lat_next.cos();
                let z_next = radius * sin_lat_next * sin_lon;
                edges.push(Edge {
                    start: current,
                    end: Vec3::new(x_next, y_next, z_next),
                });
            }

            // Latitude edge (next latitude ring)
            let lon_next = 2.0 * PI * ((j + 1) % lon_segments) as f32 / lon_segments as f32;
            let sin_lon_next = lon_next.sin();
            let cos_lon_next = lon_next.cos();
            let x_lat = radius * sin_lat * cos_lon_next;
            let y_lat = radius * cos_lat;
            let z_lat = radius * sin_lat * sin_lon_next;
            edges.push(Edge {
                start: current,
                end: Vec3::new(x_lat, y_lat, z_lat),
            });
        }
    }

    edges
}

/// Generate mandala pattern (rotational symmetry)
fn generate_mandala(petals: usize, rings: usize, max_radius: f32) -> Vec<Edge> {
    let mut edges = Vec::new();

    for ring in 1..=rings {
        let r = max_radius * ring as f32 / rings as f32;

        for petal in 0..petals {
            let angle = 2.0 * PI * petal as f32 / petals as f32;
            let angle_next = 2.0 * PI * ((petal + 1) % petals) as f32 / petals as f32;

            let x1 = r * angle.cos();
            let y1 = r * angle.sin();
            let x2 = r * angle_next.cos();
            let y2 = r * angle_next.sin();

            // Outer ring edges
            edges.push(Edge {
                start: Vec3::new(x1, y1, 0.0),
                end: Vec3::new(x2, y2, 0.0),
            });

            // Spokes from center
            if ring == 1 {
                edges.push(Edge {
                    start: Vec3::new(0.0, 0.0, 0.0),
                    end: Vec3::new(x1, y1, 0.0),
                });
            }

            // Inner ring connections
            if ring > 1 {
                let r_inner = max_radius * (ring - 1) as f32 / rings as f32;
                let x_inner = r_inner * angle.cos();
                let y_inner = r_inner * angle.sin();
                edges.push(Edge {
                    start: Vec3::new(x1, y1, 0.0),
                    end: Vec3::new(x_inner, y_inner, 0.0),
                });
            }
        }
    }

    edges
}

/// Generate pyramid geometry
fn generate_pyramid(base_size: f32, height: f32) -> Vec<Edge> {
    let h = base_size / 2.0;
    let vertices = [
        Vec3::new(-h, -h, 0.0),      // 0 - base corner
        Vec3::new(h, -h, 0.0),       // 1 - base corner
        Vec3::new(h, h, 0.0),        // 2 - base corner
        Vec3::new(-h, h, 0.0),       // 3 - base corner
        Vec3::new(0.0, 0.0, height), // 4 - apex
    ];

    let edges_idx = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // base edges
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4), // sides to apex
    ];

    edges_idx
        .iter()
        .map(|&(a, b)| Edge {
            start: vertices[a],
            end: vertices[b],
        })
        .collect()
}

/// Render edges to ASCII buffer with text
fn render_edges(
    edges: &[Edge],
    rotation: (f32, f32, f32),
    config: &AnimationConfig,
    frame: usize,
) -> String {
    let mut buffer: Vec<Vec<char>> = vec![vec![' '; config.width]; config.height];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![f32::NEG_INFINITY; config.width]; config.height];

    let filler = config.filler.as_deref().unwrap_or(&config.word);
    let word_chars: Vec<char> = config.word.chars().collect();
    let filler_chars: Vec<char> = filler.chars().collect();

    let mut char_index = 0;

    for edge in edges {
        let start = edge.start.rotate(rotation.0, rotation.1, rotation.2);
        let end = edge.end.rotate(rotation.0, rotation.1, rotation.2);

        let proj_start = project(&start, config.width as f32, config.height as f32, 1.0, 5.0);
        let proj_end = project(&end, config.width as f32, config.height as f32, 1.0, 5.0);

        let x0 = proj_start.x.round() as i32;
        let y0 = proj_start.y.round() as i32;
        let x1 = proj_end.x.round() as i32;
        let y1 = proj_end.y.round() as i32;

        // Simple line drawing with character placement
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let steps = dx.max(dy).max(1) as usize;

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let x = (x0 as f32 + (x1 - x0) as f32 * t).round() as i32;
            let y = (y0 as f32 + (y1 - y0) as f32 * t).round() as i32;
            let z = start.z + (end.z - start.z) * t;

            if x >= 0 && x < config.width as i32 && y >= 0 && y < config.height as i32 {
                let xi = x as usize;
                let yi = y as usize;

                if z > z_buffer[yi][xi] {
                    z_buffer[yi][xi] = z;

                    let ch = if char_index < word_chars.len() {
                        word_chars[char_index]
                    } else {
                        filler_chars[(char_index - word_chars.len()) % filler_chars.len()]
                    };

                    buffer[yi][xi] = ch;

                    char_index += 1;
                }
            }
        }
    }

    // Build output string with optional color
    let mut output = String::new();
    for row in buffer {
        let mut line = String::new();
        let mut last_color = None;

        for (i, &ch) in row.iter().enumerate() {
            if config.color && ch != ' ' {
                let color = COLORS[(i + frame) % COLORS.len()];
                if last_color != Some(color) {
                    line.push_str(color);
                    last_color = Some(color);
                }
                line.push(ch);
            } else {
                if last_color.is_some() {
                    line.push_str(RESET);
                    last_color = None;
                }
                line.push(ch);
            }
        }
        if last_color.is_some() {
            line.push_str(RESET);
        }
        output.push_str(&line);
        output.push('\n');
    }

    output
}

/// Render mandala (special case - 2D pattern with rotation)
fn render_mandala(edges: &[Edge], rotation: f32, config: &AnimationConfig, frame: usize) -> String {
    let mut buffer: Vec<Vec<char>> = vec![vec![' '; config.width]; config.height];

    let filler = config.filler.as_deref().unwrap_or(&config.word);
    let word_chars: Vec<char> = config.word.chars().collect();
    let filler_chars: Vec<char> = filler.chars().collect();

    let mut char_index = 0;

    for edge in edges {
        let start = edge.start.rotate(0.0, 0.0, rotation);
        let end = edge.end.rotate(0.0, 0.0, rotation);

        let proj_start = project(&start, config.width as f32, config.height as f32, 1.0, 5.0);
        let proj_end = project(&end, config.width as f32, config.height as f32, 1.0, 5.0);

        let x0 = proj_start.x.round() as i32;
        let y0 = proj_start.y.round() as i32;
        let x1 = proj_end.x.round() as i32;
        let y1 = proj_end.y.round() as i32;

        draw_line(&mut buffer, x0, y0, x1, y1, ' ');
    }

    // Fill with characters
    for row in buffer.iter_mut().take(config.height) {
        for cell in row.iter_mut().take(config.width) {
            if *cell != ' ' {
                let ch = if char_index < word_chars.len() {
                    word_chars[char_index]
                } else {
                    filler_chars[(char_index - word_chars.len()) % filler_chars.len()]
                };
                *cell = ch;
                char_index += 1;
            }
        }
    }

    let mut output = String::new();
    for row in buffer {
        let mut line = String::new();
        for (i, &ch) in row.iter().enumerate() {
            if config.color && ch != ' ' {
                let color_idx = (i + frame) % COLORS.len();
                line.push_str(COLORS[color_idx]);
                line.push(ch);
                line.push_str(RESET);
            } else {
                line.push(ch);
            }
        }
        output.push_str(&line);
        output.push('\n');
    }

    output
}

/// Main animation function
pub fn animate(config: AnimationConfig) {
    // Generate geometry based on shape
    let edges = match config.shape {
        AnimationShape::Donut => generate_donut(2.0, 1.0, 40, 20),
        AnimationShape::Cube => generate_cube(3.0),
        AnimationShape::Cube5D => generate_cube5d(2.5),
        AnimationShape::Sphere => generate_sphere(2.5, 16, 32),
        AnimationShape::Mandala => generate_mandala(12, 6, 15.0),
        AnimationShape::Pyramid => generate_pyramid(3.0, 4.0),
    };

    let mut frame = 0;
    let max_frames = config.frames.unwrap_or(usize::MAX);
    let rotation_speed = 0.05;

    // Clear screen and hide cursor
    print!("\x1b[2J\x1b[?25l");
    std::io::Write::flush(&mut std::io::stdout()).ok();

    loop {
        let start_time = Instant::now();

        // Calculate rotation angles
        let rx = frame as f32 * rotation_speed * 0.7;
        let ry = frame as f32 * rotation_speed;
        let rz = frame as f32 * rotation_speed * 0.3;

        // Render frame
        let frame_output = match config.shape {
            AnimationShape::Mandala => {
                render_mandala(&edges, frame as f32 * rotation_speed * 2.0, &config, frame)
            }
            _ => render_edges(&edges, (rx, ry, rz), &config, frame),
        };

        // Add decorative frame with AUGUSTO title
        let framed_output = add_frame(&frame_output, config.width, &config);

        // Move cursor to top-left and print frame
        print!("\x1b[H{}", framed_output);
        std::io::Write::flush(&mut std::io::stdout()).ok();

        frame += 1;
        if frame >= max_frames {
            break;
        }

        // Control frame rate
        let elapsed = start_time.elapsed();
        let target_duration = Duration::from_millis(config.speed_ms);
        if elapsed < target_duration {
            thread::sleep(target_duration - elapsed);
        }
    }

    // Show cursor and clear
    print!("\x1b[?25h\x1b[2J\x1b[H");
    std::io::Write::flush(&mut std::io::stdout()).ok();
}

/// Get a single frame as string (for testing/export)
#[allow(dead_code)]
pub fn get_frame(config: &AnimationConfig, frame: usize) -> String {
    let edges = match config.shape {
        AnimationShape::Donut => generate_donut(2.0, 1.0, 40, 20),
        AnimationShape::Cube => generate_cube(3.0),
        AnimationShape::Cube5D => generate_cube5d(2.5),
        AnimationShape::Sphere => generate_sphere(2.5, 16, 32),
        AnimationShape::Mandala => generate_mandala(12, 6, 15.0),
        AnimationShape::Pyramid => generate_pyramid(3.0, 4.0),
    };

    let rotation_speed = 0.05;
    let rx = frame as f32 * rotation_speed * 0.7;
    let ry = frame as f32 * rotation_speed;
    let rz = frame as f32 * rotation_speed * 0.3;

    match config.shape {
        AnimationShape::Mandala => {
            render_mandala(&edges, frame as f32 * rotation_speed * 2.0, config, frame)
        }
        _ => render_edges(&edges, (rx, ry, rz), config, frame),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_rotation() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let rotated = v.rotate_y(PI / 2.0);
        assert!((rotated.x - 0.0).abs() < 0.001);
        assert!((rotated.z - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_donut_generation() {
        let edges = generate_donut(2.0, 1.0, 10, 5);
        assert!(!edges.is_empty());
    }

    #[test]
    fn test_cube_generation() {
        let edges = generate_cube(2.0);
        assert_eq!(edges.len(), 12);
    }

    #[test]
    fn test_cube5d_generation() {
        let edges = generate_cube5d(2.0);
        // Tesseract has 32 edges
        assert_eq!(edges.len(), 32);
    }

    #[test]
    fn test_sphere_generation() {
        let edges = generate_sphere(1.0, 8, 16);
        assert!(!edges.is_empty());
    }

    #[test]
    fn test_mandala_generation() {
        let edges = generate_mandala(8, 4, 10.0);
        assert!(!edges.is_empty());
    }

    #[test]
    fn test_pyramid_generation() {
        let edges = generate_pyramid(2.0, 3.0);
        assert_eq!(edges.len(), 8);
    }

    #[test]
    fn test_animation_config_default() {
        let config = AnimationConfig::default();
        assert_eq!(config.shape, AnimationShape::Donut);
        assert_eq!(config.speed_ms, 100);
        assert!(!config.color);
    }

    #[test]
    fn test_shape_from_str() {
        assert_eq!(
            AnimationShape::from_str("donut"),
            Some(AnimationShape::Donut)
        );
        assert_eq!(AnimationShape::from_str("cube"), Some(AnimationShape::Cube));
        assert_eq!(
            AnimationShape::from_str("cube5d"),
            Some(AnimationShape::Cube5D)
        );
        assert_eq!(
            AnimationShape::from_str("sphere"),
            Some(AnimationShape::Sphere)
        );
        assert_eq!(
            AnimationShape::from_str("mandala"),
            Some(AnimationShape::Mandala)
        );
        assert_eq!(
            AnimationShape::from_str("pyramid"),
            Some(AnimationShape::Pyramid)
        );
        assert_eq!(AnimationShape::from_str("invalid"), None);
    }

    #[test]
    fn test_get_frame() {
        let config = AnimationConfig {
            shape: AnimationShape::Cube,
            word: "TEST".to_string(),
            filler: Some("CODE".to_string()),
            speed_ms: 100,
            frames: Some(1),
            color: false,
            width: 40,
            height: 20,
        };

        let frame = get_frame(&config, 0);
        assert!(!frame.is_empty());
        assert!(
            frame.contains('T')
                || frame.contains('C')
                || frame.contains('O')
                || frame.contains('D')
                || frame.contains('E')
        );
    }
}
