use std::f32::consts::PI;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 800;
pub const HALF_HEIGHT: i32 = WINDOW_HEIGHT as i32 / 2;
pub const ROWS: u32 = 20;
pub const COLS: u32 = 24;
pub const TILE_SIZE: i32 = 32;
pub const PLAYER_SPEED: f64 = 5.0;
pub const PLAYER_TURN_SPEED: f64 = 3.0;
pub const FOV: f64 = (60.0 as f64 * PI as f64)/180.0 as f64; // 60 degrees in radians
pub const NUM_RAYS: u32 = WINDOW_WIDTH;
pub const MAX_DEPTH: f64 = 800.0;
pub const FLOOR_COLOR: (u8, u8, u8) = (64, 64, 64);
pub const SKY_COLOR: (u8, u8, u8) = (135, 206, 235); // Sky blue
pub const MINI_MAP_SCALE_FACTOR: f64 = 0.3; // Scale factor for mini-map rendering
