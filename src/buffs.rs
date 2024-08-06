use std::time::Instant;

use crate::{prelude::*, Armor, Speed};

/*
* These are things that the player will get throughout the game
* and they might be temporary or not.
* */

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum ShieldType {
    #[default]
    Physical,
    Magical,
}

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Shield {
    pub offensive: f32,
    pub defensive: f32,
    pub shield_type: ShieldType,
    pub duration_seconds: Option<u64>,
}

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub enum ItemTypeEnum {
    Speed(Speed),
    Armor(Armor),
    Shield(Shield),
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        Self::Speed(Speed::default())
    }
}

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub struct Buff {
    pub id: String,
    pub start_time: Instant,
    pub item: ItemTypeEnum,
}

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub struct Buffs(pub Vec<Buff>);
