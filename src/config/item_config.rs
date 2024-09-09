use crate::{Armor, ItemTypeEnum, Shield, Speed};

use super::*;

pub(crate) const ITEM_RANDOM_SEED: u64 = 1937836746771;
pub(crate) const ITEM_SPRITE_SIZE: u8 = 32;
pub(crate) const CHANCE_TO_SPAWN_HEALTH_POINTS_PACK: f32 = 0.7;
pub(crate) const CHANCE_TO_SPAWN_MANA_POINTS_PACK: f32 = 0.6;
pub(crate) const ITEM_BASE_MULTIPLIER_BASED_ON_LEVEL: f32 = 0.2;

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub enum ItemStatsType {
    #[default]
    Speed,
    Armor,
    Shield,
}

#[derive(Debug, Clone)]
pub struct ItemType<'a> {
    pub source: &'a str,
    pub item_type: ItemTypeEnum,
    pub item_stats_type: ItemStatsType,
}

#[derive(Debug, Clone)]
pub struct ItemByWave<'a> {
    pub wave: usize,
    pub item: ItemType<'a>,
    pub quantity: u32,
}

const ITEM_WAVE_1: ItemType = ItemType {
    source: "textures/Items/lightning.png",
    item_type: ItemTypeEnum::Speed(Speed(30.0)),
    item_stats_type: ItemStatsType::Speed,
};

const ITEM_WAVE_2: ItemType = ItemType {
    source: "textures/Items/Diamond.png",
    item_type: ItemTypeEnum::Shield(Shield {
        offensive: 0.01,
        defensive: 10.0,
        shield_type: crate::ShieldType::Physical,
        duration_seconds: Some(20),
    }),
    item_stats_type: ItemStatsType::Shield,
};

const ITEM_WAVE_3: ItemType = ItemType {
    source: "textures/Items/shield.png",
    item_type: ItemTypeEnum::Armor(Armor(20.0)),
    item_stats_type: ItemStatsType::Armor,
};

const ITEM_WAVE_4: ItemType = ItemType {
    source: "textures/Items/Diamond.png",
    item_type: ItemTypeEnum::Shield(Shield {
        offensive: 0.01,
        defensive: 10.0,
        shield_type: crate::ShieldType::Physical,
        duration_seconds: Some(20),
    }),
    item_stats_type: ItemStatsType::Shield,
};

const ITEM_WAVE_5: ItemType = ItemType {
    source: "textures/Items/lightning.png",
    item_type: ItemTypeEnum::Speed(Speed(30.0)),
    item_stats_type: ItemStatsType::Speed,
};

pub const ITEMS_PER_WAVE: [ItemByWave; NUMBER_OF_WAVES] = [
    ItemByWave {
        wave: 1,
        item: ITEM_WAVE_1,
        quantity: 3,
    },
    ItemByWave {
        wave: 2,
        item: ITEM_WAVE_2,
        quantity: 2,
    },
    ItemByWave {
        wave: 3,
        item: ITEM_WAVE_3,
        quantity: 2,
    },
    ItemByWave {
        wave: 4,
        item: ITEM_WAVE_4,
        quantity: 2,
    },
    ItemByWave {
        wave: 5,
        item: ITEM_WAVE_5,
        quantity: 3,
    },
];
