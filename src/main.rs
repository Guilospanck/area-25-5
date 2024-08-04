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
        // INFO: this is used to generate meta files for the assets.
        // They are going to be generate at `imported_assets`.
        // Run with `cargo run --features bevy/asset_processor`
        // Just copy the contents and replace them on the assets/ folder.
        // Then, run the compilation to wasm.
        // .set(AssetPlugin {
        //     mode: AssetMode::Processed,
        //     ..default()
        // }),
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
        .insert_state(GameState::Menu)
        // system sets
        .configure_sets(
            FixedUpdate,
            (
                CollisionSet.run_if(in_state(GameState::Alive)),
                MoveSet.run_if(in_state(GameState::Alive)),
                InputSet.run_if(in_state(GameState::Alive)),
            ),
        )
        // systems
        .add_systems(
            Startup,
            (
                setup_base_camera,
                setup_player_camera,
                setup_overlay_camera,
                setup_menu_camera,
                setup_resources,
                setup_sprite,
                setup_ui,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::Alive),
            (
                cleanup_system::<MenuOverlay>,
                cleanup_system::<GameOverOverlay>,
                cleanup_system::<GameWonOverlay>,
                cleanup_system::<CleanupWhenPlayerDies>,
                reset_initial_state,
                setup_player,
            )
                .chain()
                .in_set(SetupSet),
        )
        .add_systems(
            FixedUpdate,
            (move_ammo, move_enemies_towards_player, animate_sprite).in_set(MoveSet),
        )
        .add_systems(FixedUpdate, (move_char, handle_click).in_set(InputSet))
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
        .add_systems(OnEnter(GameState::Menu), menu_screen)
        .add_systems(OnEnter(GameState::Dead), game_over_screen)
        .add_systems(OnEnter(GameState::Won), game_won_screen)
        .add_systems(
            FixedUpdate,
            (
                handle_start_game_click.run_if(in_state(GameState::Menu)),
                handle_restart_click.run_if(in_state(GameState::Dead)),
                handle_play_again_click.run_if(in_state(GameState::Won)),
            ),
        )
        // events
        .observe(on_player_spawned)
        .observe(on_mouse_click)
        .observe(on_player_health_changed)
        .observe(on_player_speed_changed)
        .observe(on_enemy_health_changed)
        .observe(on_all_enemies_died)
        .observe(on_game_over)
        .observe(on_restart_click)
        .observe(on_score_changed)
        .run();
}
