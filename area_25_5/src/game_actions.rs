use crate::{
    enemy::Enemy, events::ShootBullets, player::Alien, prelude::*, util::get_unit_direction_vector,
    weapon::Ammo,
};
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

pub fn move_enemies_towards_alien(
    // The reason the `Without` is needed here, even though it wouldn't be
    // as we are only querying for the Alien on the second group,
    // is that rust mutability does not allow a variable to be mutable and
    // immutable at the same time. See https://bevyengine.org/learn/errors/#b0001
    // for more.
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Alien>)>,
    timer: Res<Time>,
    alien: Query<(&Transform, &Alien)>,
) {
    for mut transform in enemies.iter_mut() {
        let position = match alien.get_single() {
            Ok(alien_position) => Vec2::new(
                alien_position.0.translation.x,
                alien_position.0.translation.y,
            ),
            Err(_) => Vec2::splat(0.),
        };

        let unit_direction = get_unit_direction_vector(position, transform.translation.truncate());
        // See that these `-=` and `+=` are the opposite of what we use when spawning bullets
        // As now we need to make the enemies go *towards* the alien, not *outwards*
        transform.translation.x -= unit_direction.x * ENEMY_MOVE_SPEED * timer.delta_seconds();
        transform.translation.y += unit_direction.y * ENEMY_MOVE_SPEED * timer.delta_seconds();
    }
}

pub fn shoot(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    alien: Query<(&Transform, &Alien)>,
) {
    let alien_query = alien.get_single().unwrap();
    let position = Vec2::new(alien_query.0.translation.x, alien_query.0.translation.y);
    let alien = alien_query.1;
    let unit_direction = get_unit_direction_vector(position, Vec2::new(x, y));

    let angle = unit_direction.y.atan2(unit_direction.x) * -1.;

    let rotation = Quat::from_rotation_z(angle + PI / 2.);

    let alien_ammo = alien.weapon.ammo.clone();
    let ammo = Ammo {
        color: alien_ammo.color,
        mesh: alien_ammo.mesh,
        direction: unit_direction,
        damage: AMMO_DAMAGE,
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: ammo.mesh.clone(),
            material: materials.add(ammo.color),
            transform: Transform {
                translation: Vec3::new(position.x + 10., position.y, 1.),
                scale: Vec3::new(1., 1., 1.),
                rotation,
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
    mut transform: Query<&mut Transform, With<Alien>>,
    time: Res<Time>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    if transform.get_single_mut().is_err() {
        return;
    }
    let mut char_transform = transform.get_single_mut().unwrap();

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

    let char_new_pos_x = old_pos_x + direction_x * ALIEN_MOVE_SPEED * time.delta_seconds();
    let char_new_pos_y = old_pos_y + direction_y * ALIEN_MOVE_SPEED * time.delta_seconds();

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
