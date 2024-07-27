use crate::{
    enemy::Enemy, events::ShootBullets, player::Player, prelude::*,
    util::get_unit_direction_vector, weapon::Ammo,
};
use std::f32::consts::PI;

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
    player: Query<(&Transform, &Player)>,
    asset_server: Res<AssetServer>,
) {
    let player_query = player.get_single().unwrap();
    let position = Vec2::new(player_query.0.translation.x, player_query.0.translation.y);
    let player = player_query.1;
    let unit_direction = get_unit_direction_vector(position, Vec2::new(x, y));

    let angle = unit_direction.y.atan2(unit_direction.x) * -1.;

    let rotation = Quat::from_rotation_z(angle + PI / 2.);

    let player_ammo = player.weapon.ammo.clone();
    let ammo = Ammo {
        source: player_ammo.source,
        direction: unit_direction,
        damage: AMMO_DAMAGE,
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(player.weapon.ammo.source.clone()),
            transform: Transform {
                rotation,
                translation: player.weapon.pos,
                scale: Vec3::new(1., 1., 1.),
            },
            ..default()
        },
        ammo,
        GAME_LAYER,
    ));
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
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    time: Res<Time>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    if player_query.get_single_mut().is_err() {
        return;
    }
    let (mut char_transform, player) = player_query.get_single_mut().unwrap();

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

    let old_pos_x = char_transform.translation.x;
    let old_pos_y = char_transform.translation.y;

    let char_new_pos_x = old_pos_x + direction_x * player.speed * time.delta_seconds();
    let char_new_pos_y = old_pos_y + direction_y * player.speed * time.delta_seconds();

    let off_screen_x = !((-WINDOW_RESOLUTION.x_px + 20.) / 2.0
        ..=(WINDOW_RESOLUTION.x_px - 20.) / 2.0)
        .contains(&char_new_pos_x);
    let off_screen_y = !((-WINDOW_RESOLUTION.y_px + 80.) / 2.0
        ..=(WINDOW_RESOLUTION.y_px - 80.) / 2.0)
        .contains(&char_new_pos_y);

    if off_screen_x || off_screen_y {
        return;
    }

    char_transform.translation.x = char_new_pos_x;
    char_transform.translation.y = char_new_pos_y;
}
