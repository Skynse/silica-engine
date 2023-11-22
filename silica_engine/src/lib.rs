pub mod api;
pub mod colors;

pub mod group;
pub mod particle;
pub mod prelude;
pub mod variant;
pub mod variant_type;
pub mod world;

//BLUE but not strikingly blue
pub const BLUE: (u8, u8, u8) = (101, 131, 181);
pub const RED: (u8, u8, u8) = (255, 0, 0);
pub const GREEN: (u8, u8, u8) = (0, 255, 0);
pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const LIGHT_BLUE: (u8, u8, u8) = (0, 255, 255);
pub const PURPLE: (u8, u8, u8) = (255, 0, 255);
pub const ORANGE: (u8, u8, u8) = (255, 165, 0);
pub const BROWN: (u8, u8, u8) = (165, 42, 42);
pub const GRAY: (u8, u8, u8) = (128, 128, 128);
pub const DARK_GRAY: (u8, u8, u8) = (64, 64, 64);
pub const LIGHT_GRAY: (u8, u8, u8) = (192, 192, 192);
pub const LIGHT_GREEN: (u8, u8, u8) = (0, 255, 0);
pub const LIGHT_RED: (u8, u8, u8) = (255, 0, 0);
pub const LIGHT_YELLOW: (u8, u8, u8) = (255, 255, 0);
pub const LIGHT_PURPLE: (u8, u8, u8) = (255, 0, 255);
pub const LIGHT_ORANGE: (u8, u8, u8) = (255, 165, 0);
pub const LIGHT_BROWN: (u8, u8, u8) = (165, 42, 42);
pub const LIGHT_PINK: (u8, u8, u8) = (255, 192, 203);
pub const LIGHT_CYAN: (u8, u8, u8) = (224, 255, 255);
pub const LIGHT_MAGENTA: (u8, u8, u8) = (255, 0, 255);
pub const DARK_BLUE: (u8, u8, u8) = (0, 0, 128);
pub const DARK_RED: (u8, u8, u8) = (128, 0, 0);
pub const RUST: (u8, u8, u8) = (183, 65, 14);
pub const IRON: (u8, u8, u8) = (183, 65, 14);

pub const MAX_TEMP: f32 = 9275.0;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self;
}
