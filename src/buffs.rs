use crate::prelude::*;

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
    pub duration_seconds: Option<f32>,
}
