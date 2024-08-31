use std::time::Duration;

use area_25_5::*;

use bevy::{
    log::LogPlugin,
    prelude::*,
    sprite::Wireframe2dPlugin,
    time::common_conditions::on_timer,
    window::{WindowResolution, WindowTheme},
};

#[cfg(not(feature = "web"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Area 25.5".into(),
                    resolution: WindowResolution::new(1600., 900.),
                    name: Some("area_25_5.app".into()),
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: bevy::log::Level::ERROR,
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

    #[cfg(not(feature = "web"))]
    fn add_debug_related_info(app: &mut App) {
        // INFO: uncomment to inspect the world elements
        app.register_type::<Weapon>()
            .register_type::<Ammo>()
            .register_type::<Item>()
            .register_type::<Power>()
            .register_type::<CircleOfDeath>()
            .add_plugins(WorldInspectorPlugin::new());
    }

    #[cfg(feature = "web")]
    fn add_debug_related_info(_app: &mut App) {}

    add_debug_related_info(&mut app);

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
                TimeBasedSet.run_if(in_state(GameState::Alive)),
            ),
        )
        // systems
        .add_systems(Update, on_window_resize)
        .add_systems(
            Startup,
            (
                setup_base_camera,
                setup_player_camera,
                setup_overlay_camera,
                setup_menu_camera,
                setup_resources,
                setup_sprite,
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
                setup_ui,
                reset_initial_state,
                setup_player,
            )
                .chain()
                .in_set(SetupSet),
        )
        .add_systems(
            FixedUpdate,
            (
                move_player_ammo,
                move_enemy_ammo,
                animate_sprite,
                move_laser_power,
                move_enemies_towards_player,
            )
                .in_set(MoveSet),
        )
        .add_systems(
            FixedUpdate,
            (
                move_player,
                handle_click,
                handle_show_player_stats_ui,
                power_up,
            )
                .in_set(InputSet),
        )
        .add_systems(
            FixedUpdate,
            (
                check_for_ammo_collisions_with_enemy,
                check_for_power_collisions_with_enemy,
                check_for_player_collisions_to_enemy,
                check_for_item_collisions,
                check_for_weapon_collisions,
                check_for_offensive_buff_collisions_with_enemy,
                check_for_ammo_collisions_with_player,
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
        .add_systems(
            FixedUpdate,
            (
                tick_timer.run_if(on_timer(Duration::from_secs(1))),
                remove_outdated_buffs.run_if(on_timer(Duration::from_secs(1))),
                animate_player_buffs.run_if(on_timer(Duration::from_nanos(100))),
                refill_mana.run_if(on_timer(Duration::from_secs(1))),
                expand_circle_of_death.run_if(on_timer(Duration::from_millis(50))),
                change_enemy_direction.run_if(on_timer(Duration::from_secs(5))),
                shoot_at_player.run_if(on_timer(Duration::from_secs(2))),
                make_boss_spawn_enemies.run_if(on_timer(Duration::from_secs(10))),
            )
                .in_set(TimeBasedSet),
        )
        .observe(on_player_spawned)
        .observe(on_mouse_click)
        .observe(on_player_health_changed)
        .observe(on_player_mana_changed)
        .observe(on_enemy_health_changed)
        .observe(on_all_enemies_died)
        .observe(on_game_over)
        .observe(on_restart_click)
        .observe(on_score_changed)
        .observe(on_wave_changed)
        .observe(on_current_time_changed)
        .observe(on_buff_added)
        .observe(on_buff_add_ui)
        .observe(on_buff_remove_ui)
        .observe(on_weapon_found)
        .observe(on_player_profile_ui_set)
        .observe(on_power_found)
        .observe(despawn_powers)
        .observe(update_power_ui)
        .observe(maybe_spawn_health_points_pack)
        .run();
}
