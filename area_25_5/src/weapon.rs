use crate::animation::AnimationIndices;
use crate::animation::AnimationTimer;
use crate::prelude::*;
use crate::resources::SpritesResources;
use crate::stats::Damage;
use crate::stats::Direction;
use crate::util::get_random_vec3;
use crate::util::get_weapon_sprite_based_on_weapon_type;
use crate::CleanupWhenPlayerDies;

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
            layer: GAME_LAYER,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn spawn_weapon(
    commands: &mut Commands,
    weapon_by_level: &WeaponByLevel,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    let weapon_type = &weapon_by_level.weapon.weapon_type;
    let damage = weapon_by_level.weapon.damage;
    let scale = Vec3::ONE;
    let direction = Vec3::ZERO;

    for idx in 1..=weapon_by_level.quantity {
        let random_spawning_pos = get_random_vec3(idx as u64);

        let bundle = WeaponBundle::new(
            &mut texture_atlas_layout,
            sprites,
            &asset_server,
            scale,
            random_spawning_pos,
            direction,
            damage,
            weapon_type.clone(),
        );

        commands.spawn(bundle);
    }
}
