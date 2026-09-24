use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;
use crate::level::tile::{Tile, TilePayload};
use crate::object_name::ObjectName;
use crate::registry::Registry;
use crate::renderer::texture::Texture;
use crate::renderer::texture_registry::TextureRegistry;

pub struct TileRegistry {
    tiles: Vec<Tile>,
    names: HashMap<ObjectName, usize>,
}

impl TileRegistry {
    pub fn new(texture_registry: &TextureRegistry) -> anyhow::Result<Self> {
        let mut tiles = Vec::new();
        let mut names = HashMap::new();

        for entry in std::fs::read_dir("res/tiles/")? {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let tile_payload: TilePayload = ron::from_str(&std::fs::read_to_string(entry.path())?)?;

                    let object_name = ObjectName::new(&tile_payload.name);

                    names.insert(object_name, names.len());

                    tiles.push(Tile::new(tile_payload, texture_registry));
                }
            }
        }

        Ok(Self { tiles, names})
    }
}

impl Index<usize> for TileRegistry {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl Index<&str> for TileRegistry {
    type Output = usize;

    fn index(&self, index: &str) -> &Self::Output {
        &self.names[&ObjectName::new(index)]
    }
}