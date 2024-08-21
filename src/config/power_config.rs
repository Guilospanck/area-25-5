use super::*;

pub(crate) const POWER_RANDOM_SEED: u64 = 1242436746771;
pub(crate) const POWER_SPRITE_SIZE: u8 = 32;
pub(crate) const POWER_MOVE_SPEED: f32 = 100.0;

// Laser
pub(crate) const LASER_POWER_WIDTH: f32 = 300.;
pub(crate) const LASER_POWER_HEIGHT: f32 = 2.;

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone, PartialEq))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone, PartialEq))]
pub enum PowerTypeEnum {
    Explosions,
    CircleOfDeath,
    Laser,
}

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub enum StoppingCondition {
    #[default]
    Instances,
    Limit,
    ScreenBounces,
}

pub struct PowerType {
    pub damage: f32,
    pub mana_needed: f32,
    pub power_type: PowerTypeEnum,
    pub stopping_condition: StoppingCondition,
    pub max_value: u32,
}

const POWER_LVL_1: PowerType = PowerType {
    damage: 0.1,
    mana_needed: 10.0,
    power_type: PowerTypeEnum::Laser,
    stopping_condition: StoppingCondition::ScreenBounces,
    max_value: 5,
};

const POWER_LVL_2: PowerType = PowerType {
    damage: 10.0,
    mana_needed: 10.0,
    power_type: PowerTypeEnum::CircleOfDeath,
    stopping_condition: StoppingCondition::Limit,
    max_value: 0,
};

const POWER_LVL_3: PowerType = PowerType {
    damage: 10.0,
    mana_needed: 10.0,
    power_type: PowerTypeEnum::Explosions,
    stopping_condition: StoppingCondition::Instances,
    max_value: 5,
};

pub struct PowerByLevel {
    pub level: usize,
    pub power: PowerType,
    pub quantity: u32,
}

pub const POWERS_PER_WAVE: [PowerByLevel; NUMBER_OF_WAVES] = [
    PowerByLevel {
        level: 2,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 3,
        power: POWER_LVL_2,
        quantity: 1,
    },
    PowerByLevel {
        level: 4,
        power: POWER_LVL_3,
        quantity: 1,
    },
    PowerByLevel {
        level: 5,
        power: POWER_LVL_1,
        quantity: 1,
    },
    // TODO: remove as the powers are only spawned *after* the wave is done
    PowerByLevel {
        level: 6,
        power: POWER_LVL_1,
        quantity: 1,
    },
];
