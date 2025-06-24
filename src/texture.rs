use sdl3::render::{Texture, TextureCreator};
use sdl3::video::WindowContext;
use sdl3::image::LoadTexture;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Texture<'a>>,
}
impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut textures = HashMap::new();
       
        // Get project root path - works with cargo run
        let project_root = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            // When running with cargo run, use CARGO_MANIFEST_DIR
            PathBuf::from(manifest_dir)
        } else {
            // Fallback for when running the executable directly
            let exe_path = env::current_exe()
                .map_err(|e| format!("Failed to get executable path: {}", e))?;
            exe_path.parent()
                .and_then(|p| p.parent()) // Go up to target directory
                .and_then(|p| p.parent()) // Go up to project root
                .ok_or("Failed to find project root")?
                .to_path_buf()
        };
       
        // Load wall textures
        for i in 1..=5 {
            let path = project_root.join("src").join("resources").join("textures").join(format!("{}.png", i));
            let texture = texture_creator.load_texture(&path)
                .map_err(|e| format!("Failed to load texture {}: {}", path.display(), e))?;
            textures.insert(format!("wall_{}", i), texture);
        }
       
        // Load weapon texture
        let weapon_path = project_root.join("src").join("resources").join("sprites").join("weapon").join("shotgun").join("0.png");
        let weapon_texture = texture_creator.load_texture(&weapon_path)
            .map_err(|e| format!("Failed to load weapon texture: {}", e))?;
        textures.insert("weapon".to_string(), weapon_texture);
       
        // Load enemy textures
        let enemy_textures = [
            "caco_demon", "cyber_demon", "soldier"
        ];
       
        for enemy in enemy_textures {
            let path = project_root.join("src").join("resources").join("sprites").join("npc").join(enemy).join("idle").join("0.png");
            let texture = texture_creator.load_texture(&path)
                .map_err(|e| format!("Failed to load enemy texture: {}", e))?;
            textures.insert(format!("enemy_{}", enemy), texture);
        }
       
        // Load UI textures
        let ui_textures = [
            "blood_screen", "game_over", "sky", "win"
        ];
       
        for ui in ui_textures {
            let path = project_root.join("src").join("resources").join("textures").join(format!("{}.png", ui));
            let texture = texture_creator.load_texture(&path)
                .map_err(|e| format!("Failed to load UI texture: {}", e))?;
            textures.insert(ui.to_string(), texture);
        }
       
        Ok(TextureManager { texture_creator, textures })
    }
    pub fn get_texture_mut(&mut self, name: &str) -> Option<&mut Texture<'a>> {
        self.textures.get_mut(name)
    }
}