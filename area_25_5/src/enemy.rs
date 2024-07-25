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
    fn random(rand: &mut ChaCha8Rng) -> Self {
        Enemy {
            health: 100.,
            damage: 20.,
            pos: Vec2::new(
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.x_px,
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.y_px,
            ),
        }
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)));
    let color = Color::srgb(255., 255., 255.);
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    for _ in 1..=50 {
        let enemy = Enemy::random(&mut rng);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3::new(enemy.pos.x, enemy.pos.y, 1.),
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::default(),
                },
                ..default()
            },
            enemy,
            GAME_LAYER,
        ));
    }
}
