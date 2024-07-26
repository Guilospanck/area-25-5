use crate::player::Alien;
use crate::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Component, Debug, Clone)]
pub struct Ammo {
    pub direction: Vec2,
    pub mesh: Mesh2dHandle,
    pub color: Color,
    pub damage: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub ammo: Ammo,
    pub pos: Vec3,
}

impl Weapon {
    fn random(rand: &mut ChaCha8Rng, ammo: Ammo) -> Self {
        Weapon {
            pos: Vec3::new(
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.x_px - 100.0),
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.y_px - 100.0),
                CHAR_Z_INDEX,
            ),
            ammo,
        }
    }
}

pub fn spawn_weapon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    weapon_by_level: &WeaponByLevel,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)));
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    let color = weapon_by_level.weapon.color;

    let ammo = Ammo {
        color,
        mesh: shape.clone(),
        damage: weapon_by_level.weapon.damage,
        direction: Vec2::splat(0.),
    };

    for _ in 1..=weapon_by_level.quantity {
        let weapon = Weapon::random(&mut rng, ammo.clone());
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3::new(weapon.pos.x, weapon.pos.y, 1.),
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::default(),
                },
                ..default()
            },
            weapon,
            GAME_LAYER,
        ));
    }
}

pub fn move_ammo(
    mut commands: Commands,
    mut ammos: Query<(Entity, &mut Transform, &mut Ammo), Without<Alien>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, ammo) in &mut ammos {
        let new_translation_x =
            transform.translation.x + ammo.direction.x * AMMO_MOVE_SPEED * timer.delta_seconds();
        let new_translation_y =
            transform.translation.y - ammo.direction.y * AMMO_MOVE_SPEED * timer.delta_seconds();

        let off_screen_x = !(-WINDOW_RESOLUTION.x_px / 2.0..=WINDOW_RESOLUTION.x_px / 2.0)
            .contains(&new_translation_x);
        let off_screen_y = !(-WINDOW_RESOLUTION.y_px / 2.0..=WINDOW_RESOLUTION.y_px / 2.0)
            .contains(&new_translation_y);

        if off_screen_x || off_screen_y {
            commands.entity(entity).despawn();
            return;
        }

        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;
    }
}
