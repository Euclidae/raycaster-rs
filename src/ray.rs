use crate::{globals::{TILE_SIZE, WINDOW_WIDTH, WINDOW_HEIGHT}, map::Map, player::Player};
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::f64::consts::PI;

fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % (2.0 * PI);
    if normalized < 0.0 {
        normalized += 2.0 * PI;
    }
    normalized
}

fn distance_between(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub struct Ray {
    pub ray_angle: f64,
    pub wall_hit_x: f64,
    pub wall_hit_y: f64,
    pub distance: f64,
    pub color: u8,
}

impl Ray {
    pub fn new(angle: f64) -> Self {
        Ray {
            ray_angle: normalize_angle(angle),
            wall_hit_x: 0.0,
            wall_hit_y: 0.0,
            distance: 0.0,
            color: 255,
        }
    }

    pub fn cast(&mut self, player: &Player, map: &Map) {
        let is_facing_down = self.ray_angle > 0.0 && self.ray_angle < PI;
        let is_facing_up = !is_facing_down;
        let is_facing_right = self.ray_angle < 0.5 * PI || self.ray_angle > 1.5 * PI;
        let is_facing_left = !is_facing_right;

        // Horizontal wall check
        let mut horizontal_hit = (0.0, 0.0);
        let mut found_horizontal_wall = false;
        let mut next_horizontal = (0.0, 0.0);
        
        // First intersection point
        if is_facing_up {
            next_horizontal.1 = (player.y / TILE_SIZE as f64).floor() * TILE_SIZE as f64 - 0.01;
        } else {
            next_horizontal.1 = (player.y / TILE_SIZE as f64).floor() * TILE_SIZE as f64 + TILE_SIZE as f64;
        }
        next_horizontal.0 = player.x + (next_horizontal.1 - player.y) / self.ray_angle.tan();

        // Step size
        let ya = if is_facing_up { -TILE_SIZE as f64 } else { TILE_SIZE as f64 };
        let xa = ya / self.ray_angle.tan();
        
        let mut current = next_horizontal;
        while current.0 >= 0.0 && current.0 < WINDOW_WIDTH as f64 && 
              current.1 >= 0.0 && current.1 < WINDOW_HEIGHT as f64 {
            if map.has_wall_at(current.0, current.1) {
                horizontal_hit = current;
                found_horizontal_wall = true;
                break;
            }
            current.0 += xa;
            current.1 += ya;
        }

        // Vertical wall check
        let mut vertical_hit = (0.0, 0.0);
        let mut found_vertical_wall = false;
        let mut next_vertical = (0.0, 0.0);
        
        if is_facing_left {
            next_vertical.0 = (player.x / TILE_SIZE as f64).floor() * TILE_SIZE as f64 - 0.01;
        } else {
            next_vertical.0 = (player.x / TILE_SIZE as f64).floor() * TILE_SIZE as f64 + TILE_SIZE as f64;
        }
        next_vertical.1 = player.y + (next_vertical.0 - player.x) * self.ray_angle.tan();

        // Step size
        let xa = if is_facing_left { -TILE_SIZE as f64 } else { TILE_SIZE as f64 };
        let ya = xa * self.ray_angle.tan();
        
        let mut current = next_vertical;
        while current.0 >= 0.0 && current.0 < WINDOW_WIDTH as f64 && 
              current.1 >= 0.0 && current.1 < WINDOW_HEIGHT as f64 {
            if map.has_wall_at(current.0, current.1) {
                vertical_hit = current;
                found_vertical_wall = true;
                break;
            }
            current.0 += xa;
            current.1 += ya;
        }

        // Calculate distances
        let horz_distance = if found_horizontal_wall {
            distance_between(player.x, player.y, horizontal_hit.0, horizontal_hit.1)
        } else {
            f64::MAX
        };

        let vert_distance = if found_vertical_wall {
            distance_between(player.x, player.y, vertical_hit.0, vertical_hit.1)
        } else {
            f64::MAX
        };

        // Choose the closest hit
        if horz_distance < vert_distance {
            self.wall_hit_x = horizontal_hit.0;
            self.wall_hit_y = horizontal_hit.1;
            self.distance = horz_distance;
            self.color = 160;  // Darker color for horizontal hits
        } else {
            self.wall_hit_x = vertical_hit.0;
            self.wall_hit_y = vertical_hit.1;
            self.distance = vert_distance;
            self.color = 255;  // Brighter color for vertical hits
        }

        // Correct fish-eye effect
        let angle_diff = player.rotation_angle - self.ray_angle;
        self.distance *= angle_diff.cos();

        // Adjust color based on distance
        let mut color_val = (self.color as f64 * (60.0 / self.distance)) as u8;
        if color_val > 255 { color_val = 255; }
        self.color = color_val;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, player: &Player) {
        canvas.set_draw_color((255, 0, 0));
        canvas.draw_line(
            (player.x as i32, player.y as i32),
            (self.wall_hit_x as i32, self.wall_hit_y as i32)
        ).unwrap();
    }
}