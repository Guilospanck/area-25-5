use crate::{
    enemy::Enemy,
    events::ShootBullets,
    player::Player,
    powers::spawn_power,
    prelude::*,
    spawn_player_stats_ui,
    util::{get_unit_direction_vector, get_weapon_sprite_based_on_weapon_type},
    AmmoBundle, Armor, BaseCamera, Damage, Health, Mana, PlayAgainButton, PlayerCamera,
    PlayerManaChanged, PlayerStatsUI, RestartGame, RestartGameButton, Speed, SpritesResources,
    StartGameButton, Weapon,
};

pub fn move_enemies_towards_player(
    // The reason the `Without` is needed here, even though it wouldn't be
    // as we are only querying for the Player on the second group,
    // is that rust mutability does not allow a variable to be mutable and
    // immutable at the same time. See https://bevyengine.org/learn/errors/#b0001
    // for more.
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    timer: Res<Time>,
    player: Query<(&Transform, &Player)>,
) {
    for mut transform in enemies.iter_mut() {
        let position = match player.get_single() {
            Ok(player_position) => Vec2::new(
                player_position.0.translation.x,
                player_position.0.translation.y,
            ),
            Err(_) => Vec2::splat(0.),
        };

        let unit_direction = get_unit_direction_vector(position, transform.translation.truncate());
        // See that these `-=` and `+=` are the opposite of what we use when spawning bullets
        // As now we need to make the enemies go *towards* the player, not *outwards*
        transform.translation.x -= unit_direction.x * ENEMY_MOVE_SPEED * timer.delta_seconds();
        transform.translation.y += unit_direction.y * ENEMY_MOVE_SPEED * timer.delta_seconds();
    }
}

pub fn shoot(
    mut commands: Commands,
    x: f32,
    y: f32,
    player_query: Query<(&Transform, &Children), With<Player>>,
    weapon_query: Query<&Weapon>,
    asset_server: Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return;
    }
    let player = player.unwrap();

    let position = Vec2::new(player.0.translation.x, player.0.translation.y);
    let unit_direction = get_unit_direction_vector(position, Vec2::new(x, y));

    let angle = unit_direction.y.atan2(unit_direction.x) * -1.;

    let rotation = Quat::from_rotation_z(angle);

    let mut weapon_type = WeaponTypeEnum::default();

    for &child in player.1.iter() {
        if let Ok(weapon_children) = weapon_query.get(child) {
            weapon_type = weapon_children.0.clone();
        }
    }

    let damage = AMMO_DAMAGE;
    let direction = Vec3::new(unit_direction.x, unit_direction.y, 1.0);
    let pos = Vec3::new(
        player.0.translation.x + 20.0,
        player.0.translation.y,
        player.0.translation.z,
    );
    let scale = Vec3::ONE;
    let layer = PLAYER_LAYER;

    let ammo_bundle = AmmoBundle::new(
        texture_atlas_layout,
        sprites,
        &asset_server,
        scale,
        pos,
        weapon_type.clone(),
        direction,
        damage,
        rotation,
        layer.clone(),
    );

    commands.spawn(ammo_bundle);
}

pub fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform, &PlayerCamera)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform, _) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(ShootBullets { pos });
        }
    }
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed, &Player)>,
    time: Res<Time>,
    mut base_camera: Query<(&mut Transform, &BaseCamera), Without<Player>>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    if base_camera.get_single_mut().is_err() {
        return;
    }
    let (mut base_camera_transform, _) = base_camera.get_single_mut().unwrap();

    if player_query.get_single_mut().is_err() {
        return;
    }
    let (mut player_transform, player_speed, _) = player_query.get_single_mut().unwrap();

    // left move
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction_x -= 1.0;
    }
    // right move
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction_x += 1.0;
    }
    // top move
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction_y += 1.0;
    }
    // bottom move
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction_y -= 1.0;
    }

    let old_pos_x = player_transform.translation.x;
    let old_pos_y = player_transform.translation.y;

    let mut char_new_pos_x = old_pos_x + direction_x * player_speed.0 * time.delta_seconds();
    let mut char_new_pos_y = old_pos_y + direction_y * player_speed.0 * time.delta_seconds();

    let limit_x_left = (-WINDOW_RESOLUTION.x_px + PLAYER_X_MARGIN) / 2.0;
    let limit_x_right = (WINDOW_RESOLUTION.x_px - PLAYER_X_MARGIN) / 2.0;
    let limit_y_bottom = (-WINDOW_RESOLUTION.y_px + PLAYER_Y_MARGIN) / 2.0;
    let limit_y_top = (WINDOW_RESOLUTION.y_px - PLAYER_Y_MARGIN) / 2.0;

    if char_new_pos_x < limit_x_left {
        char_new_pos_x = limit_x_left;
    }
    if char_new_pos_x > limit_x_right {
        char_new_pos_x = limit_x_right;
    }
    if char_new_pos_y < limit_y_bottom {
        char_new_pos_y = limit_y_bottom;
    }
    if char_new_pos_y > limit_y_top {
        char_new_pos_y = limit_y_top;
    }

    // pan camera
    base_camera_transform.translation.x = char_new_pos_x;
    base_camera_transform.translation.y = char_new_pos_y;

    // translate player
    player_transform.translation.x = char_new_pos_x;
    player_transform.translation.y = char_new_pos_y;
}

pub fn handle_show_player_stats_ui(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    player_assets_ui_query: Query<Entity, With<PlayerStatsUI>>,
    mut player_query: Query<(&Speed, &Armor, &Children, &Health, &Mana, &Player)>,
    player_weapon_query: Query<(&Damage, &Weapon)>,
) {
    if player_query.get_single_mut().is_err() {
        return;
    }

    if keyboard_input.pressed(KeyCode::KeyK) {
        let number_of_spawned_stats_ui = player_assets_ui_query.iter().len();

        // only spawns a new ui if it does not already exist
        if number_of_spawned_stats_ui == 0 {
            let (player_speed, player_armor, player_children, player_health, player_mana, _) =
                player_query.get_single_mut().unwrap();
            for &child in player_children {
                if player_weapon_query.get(child).is_err() {
                    continue;
                }

                let player_weapon_unwrapped = player_weapon_query.get(child).unwrap();
                let player_weapon_damage = player_weapon_unwrapped.0;
                let player_weapon_type = player_weapon_unwrapped.1 .0.clone();
                let weapon_sprite =
                    get_weapon_sprite_based_on_weapon_type(player_weapon_type, &sprites);

                spawn_player_stats_ui(
                    &mut commands,
                    &asset_server,
                    player_health.0,
                    player_mana.0,
                    weapon_sprite.source,
                    player_weapon_damage.0,
                    player_armor.0,
                    player_speed.0,
                );

                break;
            }
        }
        return;
    }

    if player_assets_ui_query.get_single().is_err() {
        return;
    }
    let player_stats_ui = player_assets_ui_query.get_single().unwrap();
    commands.entity(player_stats_ui).despawn_recursive();
}

pub fn power_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut player_query: Query<(&mut Mana, &Player)>,
) {
    if player_query.get_single_mut().is_err() {
        return;
    }
    let (mut player_mana, _) = player_query.get_single_mut().unwrap();
    let mana_needed = 50.;
    if player_mana.0 < mana_needed {
        return;
    }

    spawn_power(&mut commands, texture_atlas_layout, &sprites, asset_server);
    player_mana.0 -= mana_needed;

    commands.trigger(PlayerManaChanged {
        mana: player_mana.0,
    });
}

// Won
pub fn handle_play_again_click(
    commands: Commands,
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &PlayAgainButton),
        Changed<Interaction>,
    >,
) {
    _handle_button_click(commands, interaction_query);
}

// Dead
pub fn handle_restart_click(
    commands: Commands,
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &RestartGameButton),
        Changed<Interaction>,
    >,
) {
    _handle_button_click(commands, interaction_query);
}

// Menu
pub fn handle_start_game_click(
    commands: Commands,
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &StartGameButton),
        Changed<Interaction>,
    >,
) {
    _handle_button_click(commands, interaction_query);
}

fn _handle_button_click<T: Component>(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &T), Changed<Interaction>>,
) {
    for (interaction, mut background_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.trigger(RestartGame);
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0., 255., 0.).into();
            }
            Interaction::None => {
                *background_color = Color::BLACK.into();
            }
        }
    }
}
