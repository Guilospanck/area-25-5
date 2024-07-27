use crate::{
    prelude::*, util::get_random_vec3, AnimationIndices, AnimationTimer, Damage, Health,
    SpriteInfo, Sprites, SpritesResources,
};

#[derive(Component, Clone)]
pub struct Enemy;

#[derive(Bundle, Clone)]
pub(crate) struct EnemyBundle {
    pub(crate) marker: Enemy,
    pub(crate) health: Health,
    pub(crate) damage: Damage,
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
        pos: Vec3,
        health: f32,
        damage: f32,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.enemy_char_idle.clone(),
            asset_server,
            pos,
            health,
            damage,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        enemy_sprite: SpriteInfo<'static>,
        asset_server: &Res<AssetServer>,
        pos: Vec3,
        health: f32,
        damage: f32,
    ) -> Self {
        let enemy_animation = enemy_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(enemy_sprite.layout);

        EnemyBundle {
            marker: Enemy,
            health: Health(health),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(enemy_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale: Vec3::ONE,
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

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    enemy_by_level: &EnemyByLevel,
) {
    let health = enemy_by_level.enemy.health;
    let damage = enemy_by_level.enemy.damage;

    for idx in 1..=enemy_by_level.quantity {
        let random_spawning_pos = get_random_vec3(idx as u64);

        let bundle = EnemyBundle::idle(
            texture_atlas_layout,
            &sprites.0,
            asset_server,
            random_spawning_pos,
            health,
            damage,
        );

        commands.spawn(bundle);
    }
}
