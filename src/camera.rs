use crate::prelude::*;

#[derive(Component, Clone)]
pub struct BaseCamera;

#[derive(Component, Clone)]
pub struct PlayerCamera;

#[derive(Component, Clone)]
pub struct OverlayCamera;

#[derive(Component, Clone)]
pub struct MenuCamera;

pub fn setup_base_camera(mut commands: Commands) {
    spawn_base_camera(&mut commands);
}

pub fn setup_player_camera(mut commands: Commands) {
    spawn_player_camera(&mut commands);
}

pub fn setup_overlay_camera(mut commands: Commands) {
    spawn_overlay_camera(&mut commands);
}

pub fn setup_menu_camera(mut commands: Commands) {
    spawn_menu_camera(&mut commands);
}

pub fn spawn_base_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 0,
                    ..default()
                },
                ..default()
            },
            BaseCamera,
            BASE_LAYER,
        ))
        .id()
}

pub fn spawn_player_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                ..default()
            },
            PlayerCamera,
            PLAYER_LAYER,
        ))
        .id()
}

pub fn spawn_overlay_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 2,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                ..default()
            },
            MenuCamera,
            OVERLAY_LAYER,
        ))
        .id()
}

pub fn spawn_menu_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 3,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                ..default()
            },
            MenuCamera,
            MENU_UI_LAYER,
        ))
        .id()
}
