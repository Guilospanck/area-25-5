use crate::{
    enemy::Enemy,
    events::ShootBullets,
    player::Player,
    prelude::*,
    spawn_player_stats_ui, spawn_power,
    util::{get_random_chance, get_unit_direction_vector, get_weapon_sprite_based_on_weapon_type},
    AmmoBundle, Armor, BaseCamera, Damage, Health, Mana, PlayAgainButton, PlayerCamera,
    PlayerManaChanged, PlayerStatsUI, Power, RestartGame, RestartGameButton, Speed,
    SpritesResources, StartGameButton, Weapon, WindowResolutionResource,
};

pub fn change_enemy_direction(
    mut enemies: Query<&mut Enemy, (With<Enemy>, Without<Player>)>,
    player: Query<(&Transform, &Player)>,
) {
    let position = match player.get_single() {
        Ok(player_position) => Vec2::new(
            player_position.0.translation.x,
            player_position.0.translation.y,
        ),
        Err(_) => Vec2::splat(0.),
    };

    for (index, mut enemy) in enemies.iter_mut().enumerate() {
        // We don't want the mage to move towards player. It's going to be a
        // range caster.
        if enemy.class == EnemyClassEnum::Mage {
            continue;
        }

        let mut signal = 1.;
        if index % 2 != 0 {
            signal = -1.;
        }

        let chance = get_random_chance() * signal;
        enemy.direction_intention.translation = (position
            + Vec2::new(
                INITIAL_WINDOW_RESOLUTION.x_px * chance,
                INITIAL_WINDOW_RESOLUTION.y_px * chance,
            ))
        .extend(1.);
        enemy.is_random = !enemy.is_random;
    }
}

pub fn move_enemies_towards_player(
    // The reason the `Without` is needed here, even though it wouldn't be
    // as we are only querying for the Player on the second group,
    // is that rust mutability does not allow a variable to be mutable and
    // immutable at the same time. See https://bevyengine.org/learn/errors/#b0001
    // for more.
    mut enemies: Query<
        (&mut Transform, &Enemy),
        (With<Enemy>, Without<Player>, Without<BaseCamera>),
    >,
    time: Res<Time>,
    player: Query<(&Transform, &Player), (With<Player>, Without<BaseCamera>, Without<Enemy>)>,
    base_camera: Query<
        (&Transform, &BaseCamera),
        (With<BaseCamera>, Without<Player>, Without<Enemy>),
    >,
) {
    let mut position = match player.get_single() {
        Ok(player_position) => Vec2::new(
            player_position.0.translation.x,
            player_position.0.translation.y,
        ),
        Err(_) => Vec2::splat(0.),
    };

    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    position = Vec2::new(
        position.x + base_camera_transform.translation.x,
        position.y + base_camera_transform.translation.y,
    );

    let limit_x_left =
        (-BACKGROUND_TEXTURE_RESOLUTION.x_px * BACKGROUND_TEXTURE_SCALE + PLAYER_X_MARGIN) / 2.0;
    let limit_x_right =
        (BACKGROUND_TEXTURE_RESOLUTION.x_px * BACKGROUND_TEXTURE_SCALE - PLAYER_X_MARGIN) / 2.0;
    let limit_y_bottom =
        (-BACKGROUND_TEXTURE_RESOLUTION.y_px * BACKGROUND_TEXTURE_SCALE + PLAYER_Y_MARGIN) / 2.0;
    let limit_y_top =
        (BACKGROUND_TEXTURE_RESOLUTION.y_px * BACKGROUND_TEXTURE_SCALE - PLAYER_Y_MARGIN) / 2.0;

    for (mut transform, enemy) in enemies.iter_mut() {
        // We don't want the mage to move towards player. It's going to be a
        // range caster.
        if enemy.class == EnemyClassEnum::Mage {
            continue;
        }

        // Enemies have greater speed when charging the player.
        let mut speed = ENEMY_MOVE_SPEED * ENEMY_BOOST_SPEED_WHEN_CHARGING;

        // if the player is gonna walk randomly, then uses the `direction_intention`
        // instead of the player's position as the origin for the unit direction vector.
        //
        // It also walks in a slower way.
        if enemy.is_random {
            position = enemy.direction_intention.translation.truncate();
            speed = ENEMY_MOVE_SPEED / ENEMY_BOOST_SPEED_WHEN_CHARGING;
        }

        let unit_direction = get_unit_direction_vector(position, transform.translation.truncate());

        let old_pos_x = transform.translation.x;
        let old_pos_y = transform.translation.y;

        let mut char_new_pos_x = old_pos_x - unit_direction.x * speed * time.delta_seconds();
        let mut char_new_pos_y = old_pos_y + unit_direction.y * speed * time.delta_seconds();

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

        // See that these `-=` and `+=` are the opposite of what we use when spawning bullets
        // As now we need to make the enemies go *towards* the player, not *outwards*
        transform.translation.x = char_new_pos_x;
        transform.translation.y = char_new_pos_y;
    }
}

pub fn shoot_at_enemies(
    mut commands: Commands,
    x: f32,
    y: f32,
    player_query: Query<(Entity, &Transform, &Children), With<Player>>,
    weapon_query: Query<&Weapon>,
    asset_server: Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok((player_entity, player_transform, player_children)) = player_query.get_single() else {
        return;
    };

    let position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );
    let unit_direction = get_unit_direction_vector(position, Vec2::new(x, y));

    let angle = unit_direction.y.atan2(unit_direction.x) * -1.;

    let rotation = Quat::from_rotation_z(angle);

    let mut weapon_type = WeaponTypeEnum::default();

    for &child in player_children.iter() {
        if let Ok(weapon) = weapon_query.get(child) {
            weapon_type = weapon.weapon_type.clone();
        }
    }

    let damage = AMMO_DAMAGE;
    let direction = Vec3::new(unit_direction.x, unit_direction.y, 1.0);
    let pos = Vec3::new(
        player_transform.translation.x + 20.0,
        player_transform.translation.y,
        player_transform.translation.z,
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
        player_entity,
    );

    commands.spawn(ammo_bundle);
}

pub fn shoot_at_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,

    player_query: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform, &Children), With<Enemy>>,
    weapon_query: Query<&Weapon>,
    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    let player_position = Vec2::new(
        player_transform.translation.x + base_camera_transform.translation.x,
        player_transform.translation.y + base_camera_transform.translation.y,
    );

    for (enemy_entity, enemy_transform, enemy_children) in enemies.iter() {
        for &child in enemy_children.iter() {
            if let Ok(weapon) = weapon_query.get(child) {
                let enemy_position =
                    Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);

                let unit_direction = get_unit_direction_vector(enemy_position, player_position);
                let angle = unit_direction.y.atan2(unit_direction.x) * -1.;
                let rotation = Quat::from_rotation_z(angle);

                let weapon_type = weapon.weapon_type.clone();
                let damage = AMMO_DAMAGE;
                let direction = Vec3::new(unit_direction.x, unit_direction.y, 1.0);
                let pos = Vec3::new(
                    enemy_transform.translation.x + 8.0,
                    enemy_transform.translation.y,
                    enemy_transform.translation.z,
                );
                let scale = Vec3::ONE;
                let layer = BASE_LAYER;

                let ammo_bundle = AmmoBundle::new(
                    &mut texture_atlas_layout,
                    &sprites,
                    &asset_server,
                    scale,
                    pos,
                    weapon_type.clone(),
                    direction,
                    damage,
                    rotation,
                    layer.clone(),
                    enemy_entity,
                );

                commands.spawn(ammo_bundle);
            }
        }
    }
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
    window_resolution: Res<WindowResolutionResource>,
) {
    let Ok((mut base_camera_transform, _)) = base_camera.get_single_mut() else {
        return;
    };

    let Ok((mut player_transform, player_speed, _)) = player_query.get_single_mut() else {
        return;
    };

    let mut direction_x = 0.;
    let mut direction_y = 0.;

    // top move
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction_y += 1.0;
    }
    // left move
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction_x -= 1.0;
    }
    // bottom move
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction_y -= 1.0;
    }
    // right move
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction_x += 1.0;
    }

    // -------------------- PLAYER ------------------------
    let old_pos_x = player_transform.translation.x;
    let old_pos_y = player_transform.translation.y;

    let mut char_new_pos_x = old_pos_x + direction_x * player_speed.0 * time.delta_seconds();
    let mut char_new_pos_y = old_pos_y + direction_y * player_speed.0 * time.delta_seconds();

    let limit_x_left = (-window_resolution.x_px + PLAYER_X_MARGIN) / 2.0;
    let limit_x_right = (window_resolution.x_px - PLAYER_X_MARGIN) / 2.0;
    let limit_y_bottom = (-window_resolution.y_px + PLAYER_Y_MARGIN) / 2.0;
    let limit_y_top = (window_resolution.y_px - PLAYER_Y_MARGIN) / 2.0;

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

    // translate player
    player_transform.translation.x = char_new_pos_x;
    player_transform.translation.y = char_new_pos_y;

    // ------------------- CAMERA -------------------------
    let old_camera_pos_x = base_camera_transform.translation.x;
    let old_camera_pos_y = base_camera_transform.translation.y;

    let mut base_camera_new_pos_x =
        old_camera_pos_x + direction_x * player_speed.0 * time.delta_seconds();
    let mut base_camera_new_pos_y =
        old_camera_pos_y + direction_y * player_speed.0 * time.delta_seconds();

    let limit_x_left = (-BACKGROUND_TEXTURE_RESOLUTION.x_px) / 2.0;
    let limit_x_right = (BACKGROUND_TEXTURE_RESOLUTION.x_px) / 2.0;
    let limit_y_bottom = (-BACKGROUND_TEXTURE_RESOLUTION.y_px) / 2.0;
    let limit_y_top = (BACKGROUND_TEXTURE_RESOLUTION.y_px) / 2.0;

    if base_camera_new_pos_x < limit_x_left {
        base_camera_new_pos_x = limit_x_left;
    }
    if base_camera_new_pos_x > limit_x_right {
        base_camera_new_pos_x = limit_x_right;
    }
    if base_camera_new_pos_y < limit_y_bottom {
        base_camera_new_pos_y = limit_y_bottom;
    }
    if base_camera_new_pos_y > limit_y_top {
        base_camera_new_pos_y = limit_y_top;
    }

    // pan camera
    base_camera_transform.translation.x = base_camera_new_pos_x;
    base_camera_transform.translation.y = base_camera_new_pos_y;
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
                let player_weapon_type = player_weapon_unwrapped.1.weapon_type.clone();
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,

    mut player_query: Query<(Entity, &mut Mana, &Children, &Transform)>,
    power_query: Query<(&Damage, &Power)>,
    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,

    enemies: Query<(Entity, &mut Health, &Damage), With<Enemy>>,
) {
    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    let Ok((_, mut player_mana, player_children, player_transform)) = player_query.get_single_mut()
    else {
        return;
    };

    let mut current_player_powers = vec![];
    for &child in player_children {
        if let Ok(player_powers) = power_query.get(child) {
            current_player_powers.push(player_powers);
        }
    }

    if current_player_powers.is_empty() {
        return;
    }

    let get_player_power_based_on_the_keycode = |key_code: KeyCode| -> Option<(&Damage, &Power)> {
        for player_power in current_player_powers {
            let power = player_power.1;

            if power.trigger_key == key_code {
                return Some(player_power);
            }
        }

        None
    };

    let spawn_power_based_on_keypress = |key: KeyCode| -> Option<f32> {
        let (power_damage, power) = get_player_power_based_on_the_keycode(key)?;

        if player_mana.0 < power.mana_needed {
            return None;
        }

        let player_translation = player_transform.translation + base_camera_transform.translation;

        spawn_power(
            &mut commands,
            texture_atlas_layout,
            &sprites,
            asset_server,
            meshes,
            materials,
            power.clone(),
            power_damage.clone(),
            player_translation,
            enemies,
        );
        Some(power.mana_needed)
    };

    let optional_mana = if keyboard_input.any_just_pressed([KeyCode::KeyH].into_iter()) {
        spawn_power_based_on_keypress(KeyCode::KeyH)
    } else if keyboard_input.any_just_pressed([KeyCode::KeyJ].into_iter()) {
        spawn_power_based_on_keypress(KeyCode::KeyJ)
    } else if keyboard_input.any_just_pressed([KeyCode::KeyL].into_iter()) {
        spawn_power_based_on_keypress(KeyCode::KeyL)
    } else {
        return;
    };

    let Some(mana_needed) = optional_mana else {
        return;
    };

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
                *background_color = Color::srgba(26., 50., 27., 0.3).into();
            }
            Interaction::None => {
                *background_color = Color::BLACK.into();
            }
        }
    }
}
