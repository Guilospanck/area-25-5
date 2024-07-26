use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub damage: f32,
    pub pos: Vec2,
}

impl Enemy {
    fn random(rand: &mut ChaCha8Rng, damage: f32) -> Self {
        Enemy {
            health: ENEMY_HEALTH,
            damage,
            pos: Vec2::new(
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.x_px,
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.y_px,
            ),
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    enemy_by_level: &EnemyByLevel,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)));
    let color = Color::srgb(255., 255., 255.);
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    for _ in 1..=enemy_by_level.quantity {
        let enemy = Enemy::random(&mut rng, enemy_by_level.enemy.damage);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3::new(enemy.pos.x, enemy.pos.y, 1.),
                    scale: enemy_by_level.enemy.scale,
                    rotation: Quat::default(),
                },
                ..default()
            },
            enemy,
            GAME_LAYER,
        ));
    }
}
