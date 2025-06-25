use crate::globals::*;
use crate::map::Map;
use crate::player::Player;
use crate::ray::Ray;
use crate::sprite::Sprite;
use crate::texture::TextureManager;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Raycaster {
    pub rays: Vec<Ray>,
}

impl Raycaster {
    pub fn new() -> Self {
        Raycaster {
            rays: Vec::new(),
        }
    }

    pub fn cast_all_rays(&mut self, player: &Player, map: &Map) {
        self.rays.clear();
        
        let ray_angle_step = FOV / NUM_RAYS as f64;
        let mut ray_angle = player.rotation_angle - FOV / 2.0;
        
        for i in 0..NUM_RAYS {
            let mut ray = Ray::new(ray_angle);
            ray.cast(player, map);
            self.rays.push(ray);
            ray_angle += ray_angle_step;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, texture_manager: &mut TextureManager, map: &Map, player: &Player) {
        // Render sky
        let half_height_u32 = HALF_HEIGHT as u32;
        
        // Render sky with texture
        if let Some(sky_texture) = texture_manager.get_texture_mut("sky") {
            // Calculate sky offset based on player rotation
            let sky_width = sky_texture.query().width;
            let offset_x = (player.rotation_angle / (2.0 * std::f64::consts::PI) * sky_width as f64) as i32;
            
            // Draw sky in two parts to handle wrapping
            let part1_width = sky_width - offset_x as u32;
            let part2_width = WINDOW_WIDTH - part1_width;
            
            // First part of sky
            let src_rect1 = Rect::new(offset_x, 0, part1_width, sky_texture.query().height);
            let dst_rect1 = Rect::new(0, 0, part1_width, half_height_u32);
            canvas.copy(sky_texture, src_rect1, dst_rect1).unwrap();
            
            // Second part of sky (wrapped around)
            if part2_width > 0 {
                let src_rect2 = Rect::new(0, 0, part2_width, sky_texture.query().height);
                let dst_rect2 = Rect::new(part1_width as i32, 0, part2_width, half_height_u32);
                canvas.copy(sky_texture, src_rect2, dst_rect2).unwrap();
            }
        } else {
            canvas.set_draw_color(SKY_COLOR);
            canvas.fill_rect(Rect::new(0, 0, WINDOW_WIDTH, half_height_u32)).unwrap();
        }
        canvas.set_draw_color(FLOOR_COLOR);
        canvas.fill_rect(Rect::new(0, HALF_HEIGHT, WINDOW_WIDTH, HALF_HEIGHT.try_into().unwrap())).unwrap();

        for (i, ray) in self.rays.iter().enumerate() {
            if ray.distance < MAX_DEPTH {
                let wall_height = (TILE_SIZE as f64 / ray.distance * 300.0) as i32;
                let wall_top = HALF_HEIGHT - wall_height / 2;
                
                let texture_id = map.get_wall_texture_id(
                    (ray.wall_hit_x / TILE_SIZE as f64) as usize,
                    (ray.wall_hit_y / TILE_SIZE as f64) as usize
                );
                
                let texture_name = format!("wall_{}", texture_id);
                
                if let Some(texture) = texture_manager.get_texture_mut(&texture_name) {
                    let wall_x = ray.wall_hit_x % TILE_SIZE as f64;
                    let tex_x = (wall_x / TILE_SIZE as f64 * 64.0) as i32;
                    
                    let shade_factor = (1.0 - (ray.distance / MAX_DEPTH).min(1.0)) * 0.8 + 0.2; //I don't even remember what this is for. todo reference pikuma
                    let shade_value = (255.0 * shade_factor) as u8;
                    
                    texture.set_color_mod(shade_value, shade_value, shade_value);
                    
                    let src_rect = Rect::new(tex_x, 0, TEX_X_DIM,TEX_Y_DIM);
                    let dst_rect = Rect::new(i as i32, wall_top, 1, wall_height as u32);
                    
                    canvas.copy(texture, src_rect, dst_rect).unwrap();
                    texture.set_color_mod(255, 255, 255);
                } else {
                    // Fallback rendering
                    let shade_factor = (1.0 - (ray.distance / MAX_DEPTH).min(1.0)) * 0.8 + 0.2;
                    let color_value = (100.0 * shade_factor) as u8;
                    canvas.set_draw_color(Color::RGB(color_value, color_value, color_value));
                    canvas.fill_rect(Rect::new(i as i32, wall_top, 1, wall_height as u32)).unwrap();
                }
            }
        }
    }

    pub fn render_sprites(&self, canvas: &mut Canvas<Window>, texture_manager: &mut TextureManager, player: &Player, sprites: &[Sprite]) {
        let mut sprite_distances: Vec<(usize, f64)> = sprites.iter()
            .enumerate()
            .map(|(i, sprite)| {
                let dx = sprite.x - player.x;
                let dy = sprite.y - player.y;
                (i, (dx * dx + dy * dy).sqrt())
            })
            .collect();
        
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (sprite_index, distance) in sprite_distances {
            let sprite = &sprites[sprite_index];
            if !sprite.visible || distance > MAX_DEPTH { continue; }
            
            let dx = sprite.x - player.x;
            let dy = sprite.y - player.y;
            let mut angle_to_sprite = dy.atan2(dx) - player.rotation_angle;
            
            // Normalize angle
            angle_to_sprite = angle_to_sprite.rem_euclid(2.0 * std::f64::consts::PI);
            if angle_to_sprite > std::f64::consts::PI {
                angle_to_sprite -= 2.0 * std::f64::consts::PI;
            }
            
            if angle_to_sprite.abs() > FOV / 2.0 { continue; }
            
            let screen_x = ((WINDOW_WIDTH as f64 / 2.0) 
                + (angle_to_sprite / FOV * WINDOW_WIDTH as f64)) as i32;
                
            let sprite_size = (TILE_SIZE as f64 / distance * 300.0) as i32;
            let sprite_screen_y = HALF_HEIGHT - sprite_size / 2;
            
            let ray_index = (screen_x as f64 / WINDOW_WIDTH as f64 * NUM_RAYS as f64) as usize;
            if ray_index < self.rays.len() && self.rays[ray_index].distance > distance {
                if let Some(texture) = texture_manager.get_texture_mut(&sprite.texture_name) {
                    let shade_factor = (1.0 - (distance / MAX_DEPTH).min(1.0)) * 0.8 + 0.2;
                    let shade_value = (255.0 * shade_factor) as u8;
                    texture.set_color_mod(shade_value, shade_value, shade_value);
                    
                    canvas.copy(
                        texture,
                        None,
                        Rect::new(
                            screen_x - sprite_size / 2,
                            sprite_screen_y,
                            sprite_size as u32,
                            sprite_size as u32
                        )
                    ).unwrap();
                    texture.set_color_mod(255, 255, 255);
                }
            }
        }
    }
}