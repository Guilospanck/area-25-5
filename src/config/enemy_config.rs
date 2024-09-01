use super::*;

pub(crate) const ENEMY_MOVE_SPEED: f32 = 100.0;
// When charging the player, the enemy gains a boost of speed.
pub(crate) const ENEMY_BOOST_SPEED_WHEN_CHARGING: f32 = 1.5;
pub(crate) const ENEMY_HEALTH: f32 = 100.0;
pub(crate) const ENEMY_COLLISION_BOX_WIDTH: f32 = 19.;
pub(crate) const ENEMY_COLLISION_BOX_HEIGHT: f32 = 32.;
pub(crate) const ENEMY_RANDOM_SEED: u64 = 1987836746771;
// Orc Boss
pub(crate) const BOSS_SCALE: f32 = 5.0;

// Each level the base damage of all enemies is updated
pub(crate) const ENEMY_BASE_DAMAGE_MULTIPLIER_BASED_ON_LEVEL: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnemyClassEnum {
    Orc,
    Mage,
    BossOrc,
    BossMage,
    BossAlien,
}

#[derive(Debug, Clone)]
pub struct EnemyType {
    pub base_damage: f32,
    pub health: f32,
    pub scale: Vec3,
    pub class: EnemyClassEnum,
}

#[derive(Debug, Clone)]
pub struct EnemyByWave {
    pub wave: usize,
    pub enemy: EnemyType,
    pub quantity: u32,
}

pub(crate) const BOSS_LVL_1: EnemyType = EnemyType {
    base_damage: 50.0,
    health: ENEMY_HEALTH * BOSS_SCALE,
    scale: Vec3::splat(BOSS_SCALE),
    class: EnemyClassEnum::BossOrc,
};

pub(crate) const BOSS_LVL_2: EnemyType = EnemyType {
    base_damage: 100.0,
    health: ENEMY_HEALTH * BOSS_SCALE * 2.,
    scale: Vec3::splat(BOSS_SCALE),
    class: EnemyClassEnum::BossMage,
};

pub(crate) const BOSS_LVL_3: EnemyType = EnemyType {
    base_damage: 150.0,
    health: ENEMY_HEALTH * BOSS_SCALE * 3.,
    scale: Vec3::splat(BOSS_SCALE),
    class: EnemyClassEnum::BossAlien,
};

const ENEMY_WAVE_1: EnemyType = EnemyType {
    base_damage: 5.0,
    health: ENEMY_HEALTH,
    scale: Vec3::splat(2.0),
    class: EnemyClassEnum::Mage,
};
const ENEMY_WAVE_2: EnemyType = EnemyType {
    base_damage: 10.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.2, 1.2, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_WAVE_3: EnemyType = EnemyType {
    base_damage: 15.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.4, 1.4, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_WAVE_4: EnemyType = EnemyType {
    base_damage: 20.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.6, 1.6, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_WAVE_5: EnemyType = EnemyType {
    base_damage: 25.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.8, 1.8, 1.0),
    class: EnemyClassEnum::Orc,
};

pub const ENEMIES_PER_WAVE: [EnemyByWave; NUMBER_OF_WAVES] = [
    EnemyByWave {
        wave: 1,
        enemy: ENEMY_WAVE_1,
        quantity: 1,
    },
    EnemyByWave {
        wave: 2,
        enemy: ENEMY_WAVE_2,
        quantity: 3,
    },
    EnemyByWave {
        wave: 3,
        enemy: ENEMY_WAVE_3,
        quantity: 15,
    },
    EnemyByWave {
        wave: 4,
        enemy: ENEMY_WAVE_4,
        quantity: 20,
    },
    EnemyByWave {
        wave: 5,
        enemy: ENEMY_WAVE_5,
        quantity: 25,
    },
];
