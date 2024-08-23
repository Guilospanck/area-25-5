use bevy::prelude::*;
pub use bevy::render::view::RenderLayers;

// config modules
pub mod buff_config;
pub mod enemy_config;
pub mod item_config;
pub mod power_config;
pub mod weapon_config;

// re-export all config modules
pub(crate) use buff_config::*;
pub use enemy_config::*;
pub use item_config::*;
pub use power_config::*;
pub use weapon_config::*;

// Layers control which entities should be rendered by cameras
pub(crate) const BASE_LAYER: RenderLayers = RenderLayers::layer(0);
pub(crate) const PLAYER_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const OVERLAY_LAYER: RenderLayers = RenderLayers::layer(2);
pub(crate) const MENU_UI_LAYER: RenderLayers = RenderLayers::layer(3);

pub(crate) const TILE_Z_INDEX: f32 = 0.;
pub(crate) const CHAR_Z_INDEX: f32 = 1.;
pub(crate) const UI_Z_INDEX: f32 = 2.;

pub(crate) const PLAYER_MOVE_SPEED: f32 = 150.0;
pub(crate) const PLAYER_ARMOR: f32 = 0.0;
pub(crate) const PLAYER_HEALTH: f32 = 1000.;
pub(crate) const PLAYER_MANA: f32 = 100.;
pub(crate) const PLAYER_SPRITE_SIZE: u8 = 32;

// These are margins so when we move the player
// it doesn't show only his half on the screen
// (remember that the point of translation is at the center of the player,
// therefore we must have these so we translate it properly and his head/legs
// are not outside the screen)
pub(crate) const PLAYER_X_MARGIN: f32 = 20.;
pub(crate) const PLAYER_Y_MARGIN: f32 = 80.;

pub(crate) const AMMO_MOVE_SPEED: f32 = 100.0;
pub(crate) const AMMO_DAMAGE: f32 = 10.0;

pub(crate) const CAPSULE_LENGTH: f32 = 8.;
pub(crate) const CAPSULE_RADIUS: f32 = 4.;

pub(crate) const BASE_CAMERA_PROJECTION_SCALE: f32 = 0.5;
pub(crate) const SCORE_MULTIPLIER: f32 = 0.1;

pub const CAPSULE_COLLIDER: Vec2 =
    Vec2::new((CAPSULE_LENGTH + CAPSULE_RADIUS * 2.) / 2., CAPSULE_RADIUS);

pub const NUMBER_OF_WAVES: usize = 5;
