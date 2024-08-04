use super::*;

pub(crate) const ITEM_RANDOM_SEED: u64 = 1937836746771;
pub(crate) const ITEM_SPRITE_SIZE: u8 = 32;

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum ItemStatsType {
    #[default]
    Speed,
    Armor,
}

pub struct ItemType<'a> {
    pub value: f32,
    pub source: &'a str,
    pub item_type: ItemStatsType,
}

const ITEM_LVL_1: ItemType = ItemType {
    value: 10.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_2: ItemType = ItemType {
    value: 20.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_3: ItemType = ItemType {
    value: 30.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_4: ItemType = ItemType {
    value: 40.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_5: ItemType = ItemType {
    value: 50.0,
    source: "textures/Effects/speed_potion.png",
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
