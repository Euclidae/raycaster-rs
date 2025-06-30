mod globals;
mod map;
mod player;
mod ray;
mod raycaster;
mod texture;
mod sprite;

use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use std::time::{Duration, Instant};

use crate::globals::{WINDOW_WIDTH, WINDOW_HEIGHT, TILE_SIZE};
use crate::map::Map;
use crate::player::Player;
use crate::ray::Ray;
use crate::raycaster::Raycaster;
use crate::texture::TextureManager;
use crate::sprite::Sprite;

fn main() -> Result<(), String> {
    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();
    
    let window = video
        .window("Raycaster-RS", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator)?;
    
    let mut sprites = vec![
        Sprite::new(5.5 * TILE_SIZE as f64, 3.5 * TILE_SIZE as f64, "enemy_caco_demon"),
        Sprite::new(8.5 * TILE_SIZE as f64, 7.5 * TILE_SIZE as f64, "enemy_cyber_demon"),
        Sprite::new(12.5 * TILE_SIZE as f64, 5.5 * TILE_SIZE as f64, "enemy_soldier"),
    ];
    
    let mut event_pump = sdl.event_pump()
        .map_err(|e| e.to_string())?;
    
    let map = Map::new();
    let mut player = Player::new();
    let mut raycaster = Raycaster::new();
    let mut last_frame_time = Instant::now();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        
        // Get keyboard state
        let keyboard_state = event_pump.keyboard_state();
        
        player.turn_direction = 0;
        player.move_direction = 0;
        
        if keyboard_state.is_scancode_pressed(Scancode::Right) || keyboard_state.is_scancode_pressed(Scancode::D) {
            player.turn_direction = 1;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) || keyboard_state.is_scancode_pressed(Scancode::A) {
            player.turn_direction = -1;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Up) || keyboard_state.is_scancode_pressed(Scancode::W) {
            player.move_direction = 1;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) || keyboard_state.is_scancode_pressed(Scancode::S) {
            player.move_direction = -1;
        }
        
        // Update animations
        let frame_time = last_frame_time.elapsed();
        let delta_time = frame_time.as_secs_f32();
        for sprite in &mut sprites {
            sprite.update_animation(delta_time);
        }
        
        player.update(&map);
        raycaster.cast_all_rays(&player, &map);
        
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        
        raycaster.render(&mut canvas, &mut texture_manager, &map, &player); 
        map.render(&mut canvas);
        raycaster.render_sprites(&mut canvas, &mut texture_manager, &player, &sprites);
        player.render(&mut canvas);
        raycaster.render_all_rays(&mut canvas,&player);

        if let Some(weapon_texture) = texture_manager.get_texture_mut("weapon") {
            canvas.copy(
                weapon_texture,
                None,
                Rect::new(
                    (WINDOW_WIDTH as i32 / 2 - 100) as i32,
                    (WINDOW_HEIGHT as i32 - 200) as i32,
                    200,
                    200
                )
            ).unwrap();// Yes I am unwrapping everything. Don't question it.
        }
        
        canvas.present();
        
        // Frame rate control
        let frame_time = last_frame_time.elapsed();
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
        last_frame_time = Instant::now();
    }
    
    Ok(())
}

// TODO work on adding mouse inputs for player movement and rotation
// TODO add a weapon system with animations and firing mechanics