use crate::{
    prelude::*,
    spawn_health_bar,
    util::{get_enemy_sprite_based_on_enemy_class, get_random_vec3},
    AmmoBundle, AnimationIndices, AnimationTimer, CleanupWhenPlayerDies, Damage, Health, SpritesResources, WeaponBundle,
};

#[derive(Component, Clone)]
pub struct Enemy {
    pub is_random: bool,
    pub direction_intention: Transform,
    pub class: EnemyClassEnum,
}

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
        asset_server: &Res<AssetServer>,
        sprites: &Res<SpritesResources>,
        pos: Vec3,
        health: f32,
        damage: f32,
        scale: Vec3,
        class: EnemyClassEnum,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            asset_server,
            sprites,
            pos,
            health,
            damage,
            scale,
            class,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        asset_server: &Res<AssetServer>,
        sprites: &Res<SpritesResources>,
        pos: Vec3,
        health: f32,
        damage: f32,
        scale: Vec3,
        class: EnemyClassEnum,
    ) -> Self {
        let enemy_sprite = get_enemy_sprite_based_on_enemy_class(class.clone(), sprites);
        let enemy_animation = enemy_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(enemy_sprite.layout);

        EnemyBundle {
            marker: Enemy {
                is_random: false,
                direction_intention: Transform::default(),
                class,
            },
            name: Name::new("Enemy"),
            health: Health(health),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(enemy_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale,
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_animation.indices.first,
            },
            animation_indices: enemy_animation.indices,
            animation_timer: enemy_animation.timer,
            layer: BASE_LAYER,
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
    let scale = enemy_by_level.enemy.scale;
    let health_bar_translation = Vec3::new(2.0, 15.0, 0.0);
    let enemy_class = enemy_by_level.enemy.class.clone();
    let quantity = enemy_by_level.quantity;

    match enemy_class {
        EnemyClassEnum::Orc => spawn_orc_enemy(
            commands,
            asset_server,
            sprites,
            texture_atlas_layout,
            meshes,
            materials,
            health,
            damage,
            scale,
            health_bar_translation,
            quantity,
        ),
        EnemyClassEnum::Mage => spawn_mage_enemy(
            commands,
            asset_server,
            sprites,
            texture_atlas_layout,
            meshes,
            materials,
            health,
            damage,
            scale,
            health_bar_translation,
            quantity,
        ),
    }
}

fn spawn_orc_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    health: f32,
    damage: f32,
    scale: Vec3,
    health_bar_translation: Vec3,
    quantity: u32,
) {
    for idx in 1..=quantity as usize {
        let random_spawning_pos = get_random_vec3(idx as u64, None);

        let bundle = EnemyBundle::idle(
            texture_atlas_layout,
            asset_server,
            sprites,
            random_spawning_pos,
            health,
            damage,
            scale,
            EnemyClassEnum::Orc,
        );

        let health_bar = spawn_health_bar(
            commands,
            meshes,
            materials,
            health,
            health,
            health_bar_translation,
            BASE_LAYER,
        );
        commands.spawn(bundle).push_children(&[health_bar]);
    }
}

fn spawn_mage_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    health: f32,
    damage: f32,
    scale: Vec3,
    health_bar_translation: Vec3,
    quantity: u32,
) {
    let weapon_damage = ENEMY_AMMO_DAMAGE;
    let weapon_direction = Vec3::ZERO;
    let weapon_pos = Vec3::new(8.0, 0.0, CHAR_Z_INDEX);
    let weapon_scale = Vec3::splat(0.5);
    let weapon_type = WeaponTypeEnum::Wand;
    let layer = BASE_LAYER;

    let ammo_scale = Vec3::ONE;
    let ammo_rotation = Quat::default();

    let health_bar_entity = spawn_health_bar(
        commands,
        meshes,
        materials,
        health,
        health,
        health_bar_translation,
        layer.clone(),
    );

    for idx in 1..=quantity as usize {
        let random_spawning_pos = get_random_vec3(idx as u64, None);

        let bundle = EnemyBundle::idle(
            texture_atlas_layout,
            asset_server,
            sprites,
            random_spawning_pos,
            health,
            damage,
            scale,
            EnemyClassEnum::Mage,
        );

        let enemy_mage_entity = commands.spawn(bundle).id();

        let weapon_bundle = WeaponBundle::new(
            texture_atlas_layout,
            sprites,
            asset_server,
            weapon_scale,
            weapon_pos,
            weapon_direction,
            weapon_damage,
            weapon_type.clone(),
            layer.clone(),
            enemy_mage_entity,
        );

        let ammo_bundle = AmmoBundle::new(
            texture_atlas_layout,
            sprites,
            asset_server,
            ammo_scale,
            weapon_pos,
            weapon_type.clone(),
            weapon_direction,
            weapon_damage,
            ammo_rotation,
            layer.clone(),
            enemy_mage_entity,
        );

        commands
            .entity(enemy_mage_entity)
            .with_children(|parent| {
                parent.spawn(weapon_bundle.clone()).with_children(|parent| {
                    parent.spawn(ammo_bundle.clone());
                });
            })
            .push_children(&[health_bar_entity]);
    }
}
