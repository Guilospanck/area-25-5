use bevy::prelude::*;
pub use bevy::render::view::RenderLayers;

// config modules
pub mod enemy_config;
pub mod item_config;
pub mod weapon_config;

// re-export all config modules
pub use enemy_config::*;
pub use item_config::*;
pub use weapon_config::*;

pub(crate) const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const TILE_Z_INDEX: f32 = 0.;
pub(crate) const CHAR_Z_INDEX: f32 = 1.;
pub(crate) const UI_Z_INDEX: f32 = 2.;

pub(crate) const PLAYER_MOVE_SPEED: f32 = 150.0;
pub(crate) const PLAYER_ARMOR: f32 = 100.0;
pub(crate) const PLAYER_HEALTH: f32 = 10000.;

pub(crate) const AMMO_MOVE_SPEED: f32 = 100.0;
pub(crate) const AMMO_DAMAGE: f32 = 10.0;

pub(crate) const CAPSULE_LENGTH: f32 = 8.;
pub(crate) const CAPSULE_RADIUS: f32 = 4.;

pub struct CustomWindowResolution {
    pub x_px: f32,
    pub y_px: f32,
}

pub const WINDOW_RESOLUTION: CustomWindowResolution = CustomWindowResolution {
    x_px: 1920.0,
    y_px: 1080.0,
};

pub const CAPSULE_COLLIDER: Vec2 =
    Vec2::new((CAPSULE_LENGTH + CAPSULE_RADIUS * 2.) / 2., CAPSULE_RADIUS);

pub const NUMBER_OF_WAVES: usize = 5;
