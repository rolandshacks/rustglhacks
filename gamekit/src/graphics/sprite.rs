//
// Sprite
//

use super::{types::{Rectangle, Color, Vector2}};

defaults!();

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct SpriteData {
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,
    pub texture_coords: Rectangle
}

pub trait Sprite {
    fn get_sprite_data(&self) -> &SpriteData;
}
