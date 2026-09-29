use crate::math::Rect;
use crate::renderer::BlitFlags;
use crate::renderer::palette::Colors;

pub struct SpriteComponent {
    pub colors: Colors,
    pub src: Rect<u32>,
    pub texture_id: usize,
    pub flags: BlitFlags,
}

pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
}

pub struct PlayerComponent;
