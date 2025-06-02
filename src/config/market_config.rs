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
    cost: 10.0,
    stat: 12.0,
    market_type: MarketTypes::Weapon(WeaponTypeEnum::Wand),
};

pub const MARKET_ITEMS: [MarketItem; 1] = [WAND_MARKET];
