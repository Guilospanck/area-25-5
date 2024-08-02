use crate::{
    prelude::*, spawn_health_bar, util::get_random_vec3, AnimationIndices, AnimationTimer,
    CleanupWhenPlayerDies, Damage, Health, SpriteInfo, Sprites, SpritesResources,
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
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
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
            name: Name::new("Enemy"),
            health: Health(health),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(enemy_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale: Vec3::splat(3.0),
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_animation.indices.first,
            },
            animation_indices: enemy_animation.indices,
            animation_timer: enemy_animation.timer,
            layer: PLAYER_LAYER,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    enemy_by_level: &EnemyByLevel,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let health = enemy_by_level.enemy.health;
    let damage = enemy_by_level.enemy.damage;

    let health_bar_translation = Vec3::new(2.0, 15.0, 0.0);

    for idx in 1..=enemy_by_level.quantity {
        let random_spawning_pos = get_random_vec3(idx as u64, None);

        let bundle = EnemyBundle::idle(
            texture_atlas_layout,
            &sprites.0,
            asset_server,
            random_spawning_pos,
            health,
            damage,
        );

        let health_bar = spawn_health_bar(
            commands,
            meshes,
            materials,
            health,
            health,
            health_bar_translation,
        );
        commands.spawn(bundle).push_children(&[health_bar]);
    }
}
