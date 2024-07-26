use bevy::color::palettes::css::{BLUE, VIOLET, YELLOW};
pub use bevy::render::view::RenderLayers;
use bevy::{
    color::palettes::css::{GREEN, ORANGE},
    prelude::*,
};

pub(crate) const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const TILE_Z_INDEX: f32 = 0.;
pub(crate) const CHAR_Z_INDEX: f32 = 1.;

pub(crate) const ALIEN_MOVE_SPEED: f32 = 150.0;
pub(crate) const ALIEN_ARMOR: f32 = 100.0;
pub(crate) const ALIEN_HEALTH: f32 = 10000.;

pub(crate) const ENEMY_MOVE_SPEED: f32 = 100.0;
pub(crate) const ENEMY_DAMAGE: f32 = 10.0;
pub(crate) const ENEMY_HEALTH: f32 = 100.0;
pub(crate) const ENEMY_COLLISION_BOX_WIDTH: f32 = 19.;
pub(crate) const ENEMY_COLLISION_BOX_HEIGHT: f32 = 32.;

pub(crate) const AMMO_MOVE_SPEED: f32 = 100.0;
pub(crate) const AMMO_DAMAGE: f32 = 50.0;

pub(crate) const CAPSULE_LENGTH: f32 = 8.;
pub(crate) const CAPSULE_RADIUS: f32 = 4.;

pub(crate) const ITEM_SPEED_VALUE: f32 = 50.;

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

pub enum EnemyClassEnum {
    Orc,
}

pub struct EnemyType {
    pub damage: f32,
    pub scale: Vec3,
    pub class: EnemyClassEnum,
}

const ENEMY_LVL_1: EnemyType = EnemyType {
    damage: 5.0,
    scale: Vec3::splat(1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_2: EnemyType = EnemyType {
    damage: 10.0,
    scale: Vec3::new(1.2, 1.2, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_3: EnemyType = EnemyType {
    damage: 15.0,
    scale: Vec3::new(1.4, 1.4, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_4: EnemyType = EnemyType {
    damage: 20.0,
    scale: Vec3::new(1.6, 1.6, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_5: EnemyType = EnemyType {
    damage: 25.0,
    scale: Vec3::new(1.8, 1.8, 1.0),
    class: EnemyClassEnum::Orc,
};

pub struct EnemyByLevel {
    pub level: usize,
    pub enemy: EnemyType,
    pub quantity: u32,
}
pub const ENEMIES_PER_WAVE: [EnemyByLevel; NUMBER_OF_WAVES] = [
    EnemyByLevel {
        level: 1,
        enemy: ENEMY_LVL_1,
        quantity: 5,
    },
    EnemyByLevel {
        level: 2,
        enemy: ENEMY_LVL_2,
        quantity: 10,
    },
    EnemyByLevel {
        level: 3,
        enemy: ENEMY_LVL_3,
        quantity: 15,
    },
    EnemyByLevel {
        level: 4,
        enemy: ENEMY_LVL_4,
        quantity: 20,
    },
    EnemyByLevel {
        level: 5,
        enemy: ENEMY_LVL_5,
        quantity: 25,
    },
];

pub struct WeaponType {
    pub damage: f32,
    pub color: Color,
}

const WEAPON_LVL_1: WeaponType = WeaponType {
    damage: 10.0,
    color: Color::Srgba(GREEN),
};

const WEAPON_LVL_2: WeaponType = WeaponType {
    damage: 20.0,
    color: Color::Srgba(YELLOW),
};

const WEAPON_LVL_3: WeaponType = WeaponType {
    damage: 30.0,
    color: Color::Srgba(ORANGE),
};

const WEAPON_LVL_4: WeaponType = WeaponType {
    damage: 40.0,
    color: Color::Srgba(VIOLET),
};

const WEAPON_LVL_5: WeaponType = WeaponType {
    damage: 50.0,
    color: Color::Srgba(BLUE),
};

pub struct WeaponByLevel {
    pub level: usize,
    pub weapon: WeaponType,
    pub quantity: u32,
}

pub const WEAPONS_PER_WAVE: [WeaponByLevel; NUMBER_OF_WAVES] = [
    WeaponByLevel {
        level: 1,
        weapon: WEAPON_LVL_1,
        quantity: 1,
    },
    WeaponByLevel {
        level: 2,
        weapon: WEAPON_LVL_2,
        quantity: 1,
    },
    WeaponByLevel {
        level: 3,
        weapon: WEAPON_LVL_3,
        quantity: 1,
    },
    WeaponByLevel {
        level: 4,
        weapon: WEAPON_LVL_4,
        quantity: 1,
    },
    WeaponByLevel {
        level: 5,
        weapon: WEAPON_LVL_5,
        quantity: 1,
    },
];
