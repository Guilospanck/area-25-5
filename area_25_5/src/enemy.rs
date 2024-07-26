use crate::{
    prelude::*, Ammo, AnimationIndices, AnimationTimer, SpriteInfo, Sprites, SpritesResources,
    Weapon,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Bundle, Clone)]
pub(crate) struct EnemyBundle {
    pub(crate) marker: Enemy,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
}

impl EnemyBundle {
    pub(crate) fn idle(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
        enemy: Enemy,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.enemy_char_idle.clone(),
            asset_server,
            enemy,
        )
    }

    pub(crate) fn walking(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
        enemy: Enemy,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.alien_char_walking.clone(),
            asset_server,
            enemy,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        enemy_sprite: SpriteInfo<'static>,
        asset_server: &Res<AssetServer>,
        enemy: Enemy,
    ) -> Self {
        let enemy_animation = enemy_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(enemy_sprite.layout);

        EnemyBundle {
            marker: enemy.clone(),
            sprite: SpriteBundle {
                texture: asset_server.load(enemy_sprite.source.clone()),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: Vec3::new(enemy.pos.x, enemy.pos.y, 1.0),
                    scale: Vec3::new(1., 1., 1.),
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_animation.indices.first,
            },
            animation_indices: enemy_animation.indices,
            animation_timer: enemy_animation.timer,
            layer: GAME_LAYER,
        }
    }
}

#[derive(Component, Clone)]
pub struct Enemy {
    pub health: f32,
    pub damage: f32,
    pub pos: Vec3,
}

impl Enemy {
    fn random(rand: &mut ChaCha8Rng, damage: f32) -> Self {
        Enemy {
            health: ENEMY_HEALTH,
            damage,
            pos: Vec3::new(
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.x_px,
                (rand.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.y_px,
                CHAR_Z_INDEX,
            ),
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    enemy_by_level: &EnemyByLevel,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);

    for _ in 1..=enemy_by_level.quantity {
        let enemy = Enemy::random(&mut rng, enemy_by_level.enemy.damage);
        let bundle = EnemyBundle::idle(texture_atlas_layout, &sprites.0, asset_server, enemy);

        commands.spawn(bundle);
    }
}
