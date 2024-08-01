use crate::prelude::*;

pub trait CustomCamera: Component + Clone {
    fn new() -> Self;
    fn spawn(&self, commands: &mut Commands) -> Entity;
}

#[derive(Component, Clone)]
pub struct MenuCamera;

impl CustomCamera for MenuCamera {
    fn new() -> Self {
        Self
    }

    fn spawn(&self, commands: &mut Commands) -> Entity {
        spawn_menu_camera(commands)
    }
}

#[derive(Component, Clone)]
pub struct PlayerCamera;

impl CustomCamera for PlayerCamera {
    fn new() -> Self {
        Self
    }

    fn spawn(&self, commands: &mut Commands) -> Entity {
        spawn_player_camera(commands)
    }
}

pub fn setup_menu_camera(mut commands: Commands) {
    spawn_menu_camera(&mut commands);
}

pub fn setup_swap_camera<R: CustomCamera, S: CustomCamera>(
    commands: Commands,
    out_camera: Query<(Entity, &R)>,
) {
    let potato = S::new();
    swap_camera(commands, out_camera, potato);
}

pub fn swap_camera<R: Component, S: CustomCamera>(
    mut commands: Commands,
    out_camera: Query<(Entity, &R)>,
    in_camera: S,
) {
    if let Ok((entity, _)) = out_camera.get_single() {
        commands.entity(entity).despawn();
    }

    in_camera.spawn(&mut commands);
}

pub fn spawn_menu_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((Camera2dBundle::default(), MenuCamera, MENU_LAYER))
        .id()
}

pub fn spawn_player_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform {
                    translation: Vec3::splat(0.),
                    rotation: Quat::default(),
                    scale: Vec3::new(1.5, 1.5, 1.),
                },
                ..default()
            },
            PlayerCamera,
            GAME_LAYER,
        ))
        .id()
}
