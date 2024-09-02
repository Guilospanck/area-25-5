use super::*;

pub(crate) const WEAPON_RANDOM_SEED: u64 = 1936836746771;
pub(crate) const DEFAULT_WEAPON_SPRITE_SOURCE: &str = "textures/Weapon/Bow.png";
pub(crate) const WEAPON_SPRITE_SIZE: u8 = 32;
pub(crate) const AMMO_SPRITE_SIZE: u8 = 32;
pub(crate) const WEAPON_SCALE: f32 = 2.0;

// Each level the base damage of all weapons is updated
pub(crate) const WEAPON_BASE_DAMAGE_MULTIPLIER_BASED_ON_LEVEL: f32 = 0.05;

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub enum WeaponTypeEnum {
    #[default]
    Bow,
    Wand,
}

#[derive(Debug, Clone)]
pub struct WeaponType<'a> {
    pub base_damage: f32,
    pub source: &'a str,
    pub ammo_source: &'a str,
    pub weapon_type: WeaponTypeEnum,
}

#[derive(Debug, Clone)]
pub struct WeaponByWave<'a> {
    pub wave: usize,
    pub weapon: WeaponType<'a>,
    pub quantity: u32,
}

const WEAPON_WAVE_1: WeaponType = WeaponType {
    base_damage: 10.0,
    source: "textures/Weapon/Wand.png",
    ammo_source: "textures/Weapon/MagicBall.png",
    weapon_type: WeaponTypeEnum::Wand,
};

const WEAPON_WAVE_2: WeaponType = WeaponType {
    base_damage: 15.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_WAVE_3: WeaponType = WeaponType {
    base_damage: 20.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_WAVE_4: WeaponType = WeaponType {
    base_damage: 25.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_WAVE_5: WeaponType = WeaponType {
    base_damage: 30.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

pub const WEAPONS_PER_WAVE: [WeaponByWave; NUMBER_OF_WAVES] = [
    WeaponByWave {
        wave: 1,
        weapon: WEAPON_WAVE_1,
        quantity: 10,
    },
    WeaponByWave {
        wave: 2,
        weapon: WEAPON_WAVE_2,
        quantity: 1,
    },
    WeaponByWave {
        wave: 3,
        weapon: WEAPON_WAVE_3,
        quantity: 1,
    },
    WeaponByWave {
        wave: 4,
        weapon: WEAPON_WAVE_4,
        quantity: 1,
    },
    WeaponByWave {
        wave: 5,
        weapon: WEAPON_WAVE_5,
        quantity: 1,
    },
];
