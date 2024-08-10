use super::*;

pub(crate) const WEAPON_RANDOM_SEED: u64 = 1936836746771;
pub(crate) const DEFAULT_WEAPON_SPRITE_SOURCE: &str = "textures/Weapon/Bow.png";

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum WeaponTypeEnum {
    #[default]
    Bow,
    Wand,
}

pub struct WeaponType<'a> {
    pub damage: f32,
    pub source: &'a str,
    pub ammo_source: &'a str,
    pub weapon_type: WeaponTypeEnum,
}

const WEAPON_LVL_1: WeaponType = WeaponType {
    damage: 10.0,
    source: "textures/Weapon/Wand.png",
    ammo_source: "textures/Weapon/MagicBall.png",
    weapon_type: WeaponTypeEnum::Wand,
};

const WEAPON_LVL_2: WeaponType = WeaponType {
    damage: 20.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_3: WeaponType = WeaponType {
    damage: 30.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_4: WeaponType = WeaponType {
    damage: 40.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_5: WeaponType = WeaponType {
    damage: 50.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

pub struct WeaponByLevel<'a> {
    pub level: usize,
    pub weapon: WeaponType<'a>,
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
