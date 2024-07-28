use crate::prelude::*;

#[derive(Component, Clone)]
pub struct Health(pub f32);

#[derive(Component, Clone)]
pub struct Armor(pub f32);

#[derive(Component, Clone)]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Clone)]
pub struct Direction(pub Vec3);