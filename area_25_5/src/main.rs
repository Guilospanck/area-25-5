use area_25_5::*;

use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::WindowResolution};
// use bevy_inspector_egui::{quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};

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
        // INFO: uncomment to inspect the world elements
        // .register_type::<Weapon>()
        // .register_type::<Ammo>()
        // .register_type::<Item>()
        // .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Msaa::Off)
        // systems
        .add_systems(
            Startup,
            (setup_resources, setup_camera, setup_sprite, setup_ui).chain(),
        )
        .add_systems(FixedUpdate, (animate_sprite, move_char, handle_click))
        .add_systems(FixedUpdate, (move_ammo, move_enemies_towards_player))
        .add_systems(
            FixedUpdate,
            (
                check_for_ammo_collisions_with_enemy,
                check_for_player_collisions_to_enemy,
                check_for_item_collisions,
                check_for_weapon_collisions,
            ),
        )
        // events
        .observe(on_player_spawned)
        .observe(on_mouse_click)
        .observe(on_player_health_changed)
        .observe(on_player_speed_changed)
        .observe(on_all_enemies_died)
        .run();
}
