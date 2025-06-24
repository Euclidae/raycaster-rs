use crate::globals::{WINDOW_WIDTH, WINDOW_HEIGHT};
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::f64::consts::PI;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub radius: f64, //I'll find 
    pub turn_direction: i8,
    pub move_direction: i8,
    pub rotation_angle: f64,
    pub move_speed: f64,
    pub rotation_speed: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: WINDOW_WIDTH as f64 / 2.0,
            y: WINDOW_HEIGHT as f64 / 2.0,
            radius: 3.0,
            turn_direction: 0,
            move_direction: 0,
            rotation_angle: 0.0,
            move_speed: 2.5,
            rotation_speed: 2.0 * (PI / 180.0),
        }
    }

    pub fn update(&mut self) {
        self.rotation_angle += self.turn_direction as f64 * self.rotation_speed;

        // Normalize angle to 0..2PI
        if self.rotation_angle < 0.0 {
            self.rotation_angle += 2.0 * PI;
        }
        if self.rotation_angle > 2.0 * PI {
            self.rotation_angle -= 2.0 * PI;
        }

        let move_step = self.move_direction as f64 * self.move_speed;
        self.x += self.rotation_angle.cos() * move_step;
        self.y += self.rotation_angle.sin() * move_step;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color((255, 0, 0));
        canvas.draw_point((self.x as i32, self.y as i32)).unwrap();

        // Draw direction line
        let end_x = self.x + self.rotation_angle.cos() * 50.0;
        let end_y = self.y + self.rotation_angle.sin() * 50.0;
        canvas.draw_line(
            (self.x as i32, self.y as i32),
            (end_x as i32, end_y as i32)
        ).unwrap();
    }
}