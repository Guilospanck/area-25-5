use area_25_5::*;

use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::WindowResolution};

fn main() {
    let mut app = App::new();

    app.add_plugins((
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
    ));

    if cfg!(not(target_family = "wasm")) {
        // INFO: uncomment to inspect the world elements
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.register_type::<Weapon>()
            .register_type::<Ammo>()
            .register_type::<Item>()
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.insert_resource(Msaa::Off)
        // states
        .insert_state(PlayerState::Alive)
        // system sets
        .configure_sets(
            FixedUpdate,
            (
                CollisionSet.run_if(in_state(PlayerState::Alive)),
                MoveSet.run_if(in_state(PlayerState::Alive)),
                InputSet.run_if(in_state(PlayerState::Alive)),
            ),
        )
        // systems
        .add_systems(
            Startup,
            (setup_resources, setup_camera, setup_sprite, setup_ui).chain(),
        )
        .add_systems(
            FixedUpdate,
            (move_ammo, move_enemies_towards_player, animate_sprite).in_set(MoveSet),
        )
        .add_systems(FixedUpdate, (move_char, handle_click).in_set(InputSet))
        .add_systems(
            FixedUpdate,
            handle_restart_click.run_if(in_state(PlayerState::Dead)),
        )
        .add_systems(
            FixedUpdate,
            (
                check_for_ammo_collisions_with_enemy,
                check_for_player_collisions_to_enemy,
                check_for_item_collisions,
                check_for_weapon_collisions,
            )
                .in_set(CollisionSet),
        )
        .add_systems(
            OnEnter(PlayerState::Alive),
            (
                cleanup_system::<GameOverOverlay>,
                reset_initial_state,
                setup_player,
            ),
        )
        .add_systems(
            OnEnter(PlayerState::Dead),
            (cleanup_system::<CleanupWhenPlayerDies>, game_over),
        )
        // events
        .observe(on_player_spawned)
        .observe(on_mouse_click)
        .observe(on_player_health_changed)
        .observe(on_player_speed_changed)
        .observe(on_all_enemies_died)
        .observe(on_game_over)
        .observe(on_restart_click)
        .run();
}
