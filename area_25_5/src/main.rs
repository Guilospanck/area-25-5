use area_25_5::*;

use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_RESOLUTION.x_px,
                            WINDOW_RESOLUTION.y_px,
                        )
                        .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
            Wireframe2dPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite, spawn_enemy))
        .add_systems(FixedUpdate, (animate_sprite, move_char, handle_click))
        .add_systems(Update, (move_ammo, move_enemies_towards_alien))
        .add_systems(
            Update,
            (check_for_ammo_collisions, check_for_alien_collisions),
        )
        .observe(on_mouse_click)
        .observe(on_alien_health_changed)
        .run();
}
