use super::*;

#[derive(Debug, Clone)]
pub enum MarketTypes {
    Weapon(WeaponTypeEnum),
}

#[derive(Debug, Clone)]
pub struct MarketItem {
    pub sprite: &'static str,
    pub cost: f32,
    pub stat: f32, // this would be `damage` for weapons, for example
    pub market_type: MarketTypes,
}

const WAND_MARKET: MarketItem = MarketItem {
    sprite: "textures/Weapon/Wand.png",
    cost: 40.0,
    stat: 12.0,
    market_type: MarketTypes::Weapon(WeaponTypeEnum::Wand),
};

const BOW_MARKET: MarketItem = MarketItem {
    sprite: "textures/Weapon/Bow.png",
    cost: 80.0,
    stat: 22.0,
    market_type: MarketTypes::Weapon(WeaponTypeEnum::Bow),
};

pub const NUMBER_OF_MARKET_ITEMS: usize = 2;

pub const MARKET_ITEMS: [MarketItem; NUMBER_OF_MARKET_ITEMS] = [WAND_MARKET, BOW_MARKET];
