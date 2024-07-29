use crate::{
    enemy::Enemy, events::ShootBullets, player::Player, prelude::*,
    util::get_unit_direction_vector, AmmoBundle, GameOver, RestartButton, RestartGame, Speed,
    SpritesResources, Weapon,
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
        commands.trigger(GameOver);
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
    );

    commands.spawn(ammo_bundle);
}

pub fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
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

pub fn move_char(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed, &Player)>,
    time: Res<Time>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    if player_query.get_single_mut().is_err() {
        return;
    }
    let (mut player_transform, player_speed, _) = player_query.get_single_mut().unwrap();

    // left move
    if keyboard_input.pressed(KeyCode::KeyH) {
        direction_x -= 1.0;
    }

    // right move
    if keyboard_input.pressed(KeyCode::KeyL) {
        direction_x += 1.0;
    }
    // top move
    if keyboard_input.pressed(KeyCode::KeyK) {
        direction_y += 1.0;
    }

    // bottom move
    if keyboard_input.pressed(KeyCode::KeyJ) {
        direction_y -= 1.0;
    }

    let old_pos_x = player_transform.translation.x;
    let old_pos_y = player_transform.translation.y;

    let char_new_pos_x = old_pos_x + direction_x * player_speed.0 * time.delta_seconds();
    let char_new_pos_y = old_pos_y + direction_y * player_speed.0 * time.delta_seconds();

    let off_screen_x = !((-WINDOW_RESOLUTION.x_px + 20.) / 2.0
        ..=(WINDOW_RESOLUTION.x_px - 20.) / 2.0)
        .contains(&char_new_pos_x);
    let off_screen_y = !((-WINDOW_RESOLUTION.y_px + 80.) / 2.0
        ..=(WINDOW_RESOLUTION.y_px - 80.) / 2.0)
        .contains(&char_new_pos_y);

    if off_screen_x || off_screen_y {
        return;
    }

    player_transform.translation.x = char_new_pos_x;
    player_transform.translation.y = char_new_pos_y;
}

pub fn check_restart_click(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &RestartButton),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut background_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => commands.trigger(RestartGame),
            Interaction::Hovered => {
                *background_color = Color::srgb(0., 255., 0.).into();
            }
            Interaction::None => {
                *background_color = Color::BLACK.into();
            }
        }
    }
}
