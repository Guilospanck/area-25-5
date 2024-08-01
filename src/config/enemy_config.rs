use super::*;

pub(crate) const ENEMY_MOVE_SPEED: f32 = 100.0;
pub(crate) const ENEMY_HEALTH: f32 = 100.0;
pub(crate) const ENEMY_COLLISION_BOX_WIDTH: f32 = 19.;
pub(crate) const ENEMY_COLLISION_BOX_HEIGHT: f32 = 32.;
pub(crate) const ENEMY_RANDOM_SEED: u64 = 1987836746771;

pub enum EnemyClassEnum {
    Orc,
}

pub struct EnemyType {
    pub damage: f32,
    pub health: f32,
    pub scale: Vec3,
    pub class: EnemyClassEnum,
}

const ENEMY_LVL_1: EnemyType = EnemyType {
    damage: 5.0,
    health: ENEMY_HEALTH,
    scale: Vec3::splat(1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_2: EnemyType = EnemyType {
    damage: 10.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.2, 1.2, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_3: EnemyType = EnemyType {
    damage: 15.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.4, 1.4, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_4: EnemyType = EnemyType {
    damage: 20.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.6, 1.6, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_5: EnemyType = EnemyType {
    damage: 25.0,
    health: ENEMY_HEALTH,
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