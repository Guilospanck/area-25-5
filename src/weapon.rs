use crate::animation::AnimationIndices;
use crate::animation::AnimationTimer;
use crate::prelude::*;
use crate::resources::SpritesResources;
use crate::stats::Damage;
use crate::stats::Direction;
use crate::util::get_random_vec3;
use crate::util::get_weapon_sprite_based_on_weapon_type;
use crate::CleanupWhenPlayerDies;
use rand::Rng;

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Weapon(pub WeaponTypeEnum);

#[derive(Bundle, Clone)]
pub(crate) struct WeaponBundle {
    pub(crate) marker: Weapon,
    pub(crate) direction: Direction,
    pub(crate) damage: Damage,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
}

impl WeaponBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        direction: Vec3,
        damage: f32,
        weapon_type: WeaponTypeEnum,
        layer: RenderLayers,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            pos,
            direction,
            damage,
            weapon_type,
            layer,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        direction: Vec3,
        damage: f32,
        weapon_type: WeaponTypeEnum,
        layer: RenderLayers,
    ) -> Self {
        let weapon_sprite = get_weapon_sprite_based_on_weapon_type(weapon_type.clone(), sprites);
        let weapon_animation = weapon_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(weapon_sprite.layout);

        WeaponBundle {
            name: Name::new("Weapon"),
            marker: Weapon(weapon_type),
            direction: Direction(direction),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(weapon_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale,
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: weapon_animation.indices.first,
            },
            animation_indices: weapon_animation.indices,
            animation_timer: weapon_animation.timer,
            layer,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn spawn_weapon(
    commands: &mut Commands,
    weapon_by_level: &WeaponByLevel,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: &Res<AssetServer>,
) {
    let weapon_type = &weapon_by_level.weapon.weapon_type;
    let damage = weapon_by_level.weapon.damage;
    let scale = Vec3::ONE;
    let direction = Vec3::ZERO;
    let layer = BASE_LAYER;

    for idx in 1..=weapon_by_level.quantity {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let random_spawning_pos = get_random_vec3(idx as u64, Some(n1 as u64 * WEAPON_RANDOM_SEED));

        // The base layer in which weapon is being rendered on is being scaled
        // by BASE_CAMERA_PROJECTION_SCALE, therefore we must change the weapon
        // position to be able to render weapons on the whole background "map"
        let new_pos = random_spawning_pos / BASE_CAMERA_PROJECTION_SCALE;

        let bundle = WeaponBundle::new(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            new_pos,
            direction,
            damage,
            weapon_type.clone(),
            layer.clone(),
        );

        commands.spawn(bundle);
    }
}
