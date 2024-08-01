use crate::animation::AnimationIndices;
use crate::animation::AnimationTimer;
use crate::prelude::*;
use crate::resources::SpritesResources;
use crate::stats::Damage;
use crate::stats::Direction;
use crate::util::get_ammo_sprite_based_on_weapon_type;
use crate::util::get_random_vec3;
use crate::CleanupWhenPlayerDies;

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Ammo(pub WeaponTypeEnum);

#[derive(Bundle, Clone)]
pub(crate) struct AmmoBundle {
    pub(crate) marker: Ammo,
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

impl AmmoBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        weapon_type: WeaponTypeEnum,
        direction: Vec3,
        damage: f32,
        rotation: Quat,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            pos,
            weapon_type,
            direction,
            damage,
            rotation,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        weapon_type: WeaponTypeEnum,
        direction: Vec3,
        damage: f32,
        rotation: Quat,
    ) -> Self {
        let ammo_sprite = get_ammo_sprite_based_on_weapon_type(weapon_type.clone(), sprites);
        let ammo_animation = ammo_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(ammo_sprite.layout);

        AmmoBundle {
            name: Name::new("Ammo"),
            marker: Ammo(weapon_type),
            direction: Direction(direction),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(ammo_sprite.source),
                transform: Transform {
                    rotation,
                    translation: pos,
                    scale,
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: ammo_animation.indices.first,
            },
            animation_indices: ammo_animation.indices,
            animation_timer: ammo_animation.timer,
            layer: GAME_LAYER,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn spawn_ammo(
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
    let rotation = Quat::default();

    for idx in 1..=weapon_by_level.quantity {
        let random_spawning_pos = get_random_vec3(idx as u64, Some(WEAPON_RANDOM_SEED));

        let bundle = AmmoBundle::new(
            &mut texture_atlas_layout,
            sprites,
            &asset_server,
            scale,
            random_spawning_pos,
            weapon_type.clone(),
            direction,
            damage,
            rotation,
        );

        commands.spawn(bundle);
    }
}

pub fn move_ammo(
    mut commands: Commands,
    mut ammos_query: Query<(Entity, &mut Transform, &Direction), With<Ammo>>,
    // weapon_query: Query<&Direction, (With<Weapon>, With<Player>)>,
    timer: Res<Time>,
) {
    for (entity, mut transform, ammo_direction) in &mut ammos_query {
        let new_translation_x =
            transform.translation.x + ammo_direction.0.x * AMMO_MOVE_SPEED * timer.delta_seconds();
        let new_translation_y =
            transform.translation.y - ammo_direction.0.y * AMMO_MOVE_SPEED * timer.delta_seconds();

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