use crate::prelude::*;

/*
* These are things intrinsic to the entity.
*
* */

#[derive(Component, Clone)]
pub struct Health(pub f32);

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Armor(pub f32);

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Clone)]
pub struct Direction(pub Vec3);
