use crate::prelude::*;

#[derive(Component)]
pub struct InGameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), InGameCamera, GAME_LAYER));
}
