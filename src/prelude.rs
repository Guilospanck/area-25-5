pub(crate) use crate::config::*;
pub(crate) use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

#[allow(unused_imports)]
#[cfg(not(web))]
pub(crate) use bevy_inspector_egui::prelude::*;
