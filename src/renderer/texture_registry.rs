use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;
use crate::registry::Registry;
use crate::renderer::texture::Texture;

pub struct TextureRegistry {
    textures: Vec<Texture>,
    names: HashMap<String, usize>,
}

impl TextureRegistry {
    pub fn new() -> anyhow::Result<Self> {
        let mut textures = Vec::new();

        let mut names = HashMap::new();

        for entry in std::fs::read_dir("res/textures/")? {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    let texture = Texture::new(entry.path())?;

                    names.insert(entry.path().file_stem().unwrap().to_string_lossy().to_string(), textures.len());
                    textures.push(texture);
                }
            }
        }

        Ok(Self {
            textures,
            names,
        })
    }
}

impl Index<usize> for TextureRegistry {
    type Output = Texture;

    fn index(&self, index: usize) -> &Self::Output {
        &self.textures[index]
    }
}

impl Index<&str> for TextureRegistry {
    type Output = usize;
    
    fn index(&self, index: &str) -> &Self::Output {
        &self.names[index]
    }
}