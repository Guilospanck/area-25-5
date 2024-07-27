use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug)]
pub enum ItemStatsType {
    Speed,
    Armor,
}

#[derive(Component)]
pub struct Item {
    pub pos: Vec2,
    pub stats: ItemStatsType,
    pub value: f32,
}

impl Item {
    fn random(rand: &mut ChaCha8Rng, stats: ItemStatsType, value: f32) -> Self {
        Item {
            pos: Vec2::new(
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.x_px - 100.0),
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.y_px - 100.0),
            ),
            stats,
            value,
        }
    }
}

pub fn spawn_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    stats: ItemStatsType,
    value: f32,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)));
    let color = Color::srgb(0., 255., 0.);
    let mut rng = ChaCha8Rng::seed_from_u64(13878367467713);

    for _ in 1..=10 {
        let item = Item::random(&mut rng, stats.clone(), value);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3::new(item.pos.x, item.pos.y, 1.),
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::default(),
                },
                ..default()
            },
            item,
            GAME_LAYER,
        ));
    }
}
