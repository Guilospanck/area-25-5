use crate::{Armor, ItemTypeEnum, Shield, Speed};

use super::*;

pub(crate) const ITEM_RANDOM_SEED: u64 = 1937836746771;
pub(crate) const ITEM_SPRITE_SIZE: u8 = 32;
pub(crate) const CHANCE_TO_SPAWN_HEALTH_POINTS_PACK: f32 = 0.7;

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

pub struct ItemType<'a> {
    pub source: &'a str,
    pub item: ItemTypeEnum,
    pub item_type: ItemStatsType,
}

const ITEM_LVL_2: ItemType = ItemType {
    source: "textures/Items/lightning.png",
    item: ItemTypeEnum::Speed(Speed(30.0)),
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_1: ItemType = ItemType {
    source: "textures/Items/Diamond.png",
    item: ItemTypeEnum::Shield(Shield {
        offensive: 0.01,
        defensive: 10.0,
        shield_type: crate::ShieldType::Physical,
        duration_seconds: Some(20),
    }),
    item_type: ItemStatsType::Shield,
};

const ITEM_LVL_3: ItemType = ItemType {
    source: "textures/Items/shield.png",
    item: ItemTypeEnum::Armor(Armor(20.0)),
    item_type: ItemStatsType::Armor,
};

const ITEM_LVL_4: ItemType = ItemType {
    source: "textures/Items/lightning.png",
    item: ItemTypeEnum::Speed(Speed(30.0)),
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_5: ItemType = ItemType {
    source: "textures/Items/lightning.png",
    item: ItemTypeEnum::Speed(Speed(30.0)),
    item_type: ItemStatsType::Speed,
};

pub struct ItemByLevel<'a> {
    pub level: usize,
    pub item: ItemType<'a>,
    pub quantity: u32,
}

pub const ITEMS_PER_WAVE: [ItemByLevel; NUMBER_OF_WAVES] = [
    ItemByLevel {
        level: 1,
        item: ITEM_LVL_1,
        quantity: 1,
    },
    ItemByLevel {
        level: 2,
        item: ITEM_LVL_2,
        quantity: 2,
    },
    ItemByLevel {
        level: 3,
        item: ITEM_LVL_3,
        quantity: 2,
    },
    ItemByLevel {
        level: 4,
        item: ITEM_LVL_4,
        quantity: 1,
    },
    ItemByLevel {
        level: 5,
        item: ITEM_LVL_5,
        quantity: 3,
    },
];
