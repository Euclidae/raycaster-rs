use crate::globals::TILE_SIZE;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Map {
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            grid: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 4, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 4, 0, 1],
                vec![1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 5, 0, 0, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 5, 0, 0, 0, 1, 0, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 0, 0, 0, 1, 0, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 3, 3, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 3, 3, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 5, 5, 1],
                vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 0, 0, 5, 5, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        }
    }

    pub fn get_wall_texture_id(&self, x: usize, y: usize) -> u8 {
        if y < self.grid.len() && x < self.grid[y].len() {
            let wall_value = self.grid[y][x];
            if wall_value > 0 {
                wall_value // Use the actual wall value from the grid
            } else {
                1 // Default texture if somehow we get here
            }
        } else {
            1 // Default texture for out of bounds
        }
    }

    pub fn has_wall_at(&self, x: f64, y: f64) -> bool {
        let col = (x / TILE_SIZE as f64) as usize;
        let row = (y / TILE_SIZE as f64) as usize;
       
        if row < self.grid.len() && col < self.grid[0].len() {
            self.grid[row][col] > 0
        } else {
            true // Treat out of bounds as walls
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                let tile_x = j as i32 * TILE_SIZE;
                let tile_y = i as i32 * TILE_SIZE;
               
                let color = if tile == 0 {
                    (240, 240, 240) // Light gray for floor
                } else {
                    // Different colors for different wall types
                    match tile {
                        1 => (100, 100, 100), // Dark gray
                        2 => (150, 100, 100), // Reddish
                        3 => (100, 150, 100), // Greenish
                        4 => (100, 100, 150), // Bluish
                        5 => (150, 150, 100), // Yellowish
                        _ => (80, 80, 80),     // Default dark
                    }
                };
               
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(
                    tile_x,
                    tile_y,
                    (TILE_SIZE - 1) as u32,
                    (TILE_SIZE - 1) as u32
                )).unwrap();
            }
        }
    }
}