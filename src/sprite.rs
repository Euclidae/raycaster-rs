use crate::globals::{WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Clone)]
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub texture_name: String,
    pub visible: bool,
    pub animation_frame: u8,
    pub animation_timer: f32,
}

impl Sprite {
    pub fn new(x: f64, y: f64, texture_name: &str) -> Self {
        Sprite {
            x,
            y,
            texture_name: texture_name.to_string(),
            visible: true,
            animation_frame: 0,
            animation_timer: 0.0,
        }
    }

    pub fn get_screen_position(&self, player_x: f64, player_y: f64, player_angle: f64) -> (f64, f64) {
        // Calculate relative position to player
        let dx = self.x - player_x;
        let dy = self.y - player_y;
        
        // Calculate distance for scaling
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Rotate based on player view
        let angle = dy.atan2(dx) - player_angle;
        let screen_x = (WINDOW_WIDTH as f64 / 2.0) + (angle.cos() * distance * 100.0);
        let screen_y = (WINDOW_HEIGHT as f64 / 2.0) - (distance * 10.0);
        
        (screen_x, screen_y)
    }

    pub fn update_animation(&mut self, delta_time: f32) {
        self.animation_timer += delta_time;
        if self.animation_timer > 0.1 {
            self.animation_frame = (self.animation_frame + 1) % 8;
            self.animation_timer = 0.0;
        }
    }
}