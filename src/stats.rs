use crate::prelude::*;

/*
* These are things intrinsic to the entity.
*
* */

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub struct Health(pub f32);

#[derive(Component, Clone)]
pub struct Mana(pub f32);

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub struct Armor(pub f32);

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Clone)]
pub struct Direction(pub Vec3);
