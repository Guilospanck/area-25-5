use crate::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Debug)]
pub(crate) struct AnimationInfo {
    pub(crate) indices: AnimationIndices,
    pub(crate) timer: AnimationTimer,
}

pub fn animate_sprite(
    time: Res<Time>,
    // This will get only entities that have all of these components
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
