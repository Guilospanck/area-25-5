use super::*;

pub(crate) const POWER_RANDOM_SEED: u64 = 1242436746771;
pub(crate) const POWER_SPRITE_SIZE: u8 = 32;

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub enum PowerTypeEnum {
    Explosions,
}

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum StoppingCondition {
    #[default]
    Instances,
    // Limit,
    // ScreenBounces,
}

pub struct PowerType<'a> {
    pub damage: f32,
    pub mana_needed: f32,
    pub source: &'a str,
    pub power_type: PowerTypeEnum,
    pub stopping_condition: StoppingCondition,
    pub max_value: u32,
}

const POWER_LVL_1: PowerType = PowerType {
    damage: 10.0,
    mana_needed: 10.0,
    source: "textures/Powers/Diamond.png",
    power_type: PowerTypeEnum::Explosions,
    stopping_condition: StoppingCondition::Instances,
    max_value: 5,
};

pub struct PowerByLevel<'a> {
    pub level: usize,
    pub power: PowerType<'a>,
    pub quantity: u32,
}

pub const POWERS_PER_WAVE: [PowerByLevel; NUMBER_OF_WAVES] = [
    PowerByLevel {
        level: 1,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 2,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 3,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 4,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 5,
        power: POWER_LVL_1,
        quantity: 1,
    },
];
