use crate::animation::AnimationIndices;
use crate::animation::AnimationTimer;
use crate::prelude::*;
use crate::resources::SpritesResources;
use crate::stats::Damage;
use crate::stats::Direction;
use crate::util::get_ammo_sprite_based_on_weapon_type;
use crate::CleanupWhenPlayerDies;
use crate::Player;

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone))]
pub struct Ammo {
    pub weapon_type: WeaponTypeEnum,
    pub equipped_by: Entity,
}

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
        layer: RenderLayers,
        equipped_by: Entity,
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
            layer,
            equipped_by,
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
        layer: RenderLayers,
        equipped_by: Entity,
    ) -> Self {
        let ammo_sprite = get_ammo_sprite_based_on_weapon_type(weapon_type.clone(), sprites);
        let ammo_animation = ammo_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(ammo_sprite.layout);

        AmmoBundle {
            name: Name::new("Ammo"),
            marker: Ammo {
                weapon_type,
                equipped_by,
            },
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
            layer,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn move_player_ammo(
    mut commands: Commands,
    mut ammos_query: Query<(Entity, &mut Transform, &Direction, &Ammo), With<Ammo>>,
    player_query: Query<Entity, With<Player>>,
    timer: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (entity, mut transform, ammo_direction, ammo) in &mut ammos_query {
        // Do not move enemies ammos
        if ammo.equipped_by != player_entity {
            continue;
        }

        let new_translation_x =
            transform.translation.x + ammo_direction.0.x * AMMO_MOVE_SPEED * timer.delta_seconds();
        let new_translation_y =
            transform.translation.y - ammo_direction.0.y * AMMO_MOVE_SPEED * timer.delta_seconds();

        let off_screen_x = !(-BACKGROUND_TEXTURE_RESOLUTION.x_px
            ..=BACKGROUND_TEXTURE_RESOLUTION.x_px)
            .contains(&new_translation_x);
        let off_screen_y = !(-BACKGROUND_TEXTURE_RESOLUTION.y_px
            ..=BACKGROUND_TEXTURE_RESOLUTION.y_px)
            .contains(&new_translation_y);

        if off_screen_x || off_screen_y {
            commands.entity(entity).despawn();
            return;
        }

        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;
    }
}

pub fn move_enemy_ammo(
    mut commands: Commands,
    mut ammos_query: Query<(Entity, &mut Transform, &Direction, &Ammo), With<Ammo>>,
    player_query: Query<Entity, With<Player>>,
    timer: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (entity, mut transform, ammo_direction, ammo) in &mut ammos_query {
        // Do not move player ammos
        if ammo.equipped_by == player_entity {
            continue;
        }

        let new_translation_x =
            transform.translation.x + ammo_direction.0.x * AMMO_MOVE_SPEED * timer.delta_seconds();
        let new_translation_y =
            transform.translation.y - ammo_direction.0.y * AMMO_MOVE_SPEED * timer.delta_seconds();

        let off_screen_x = !(-BACKGROUND_TEXTURE_RESOLUTION.x_px
            ..=BACKGROUND_TEXTURE_RESOLUTION.x_px)
            .contains(&new_translation_x);
        let off_screen_y = !(-BACKGROUND_TEXTURE_RESOLUTION.y_px
            ..=BACKGROUND_TEXTURE_RESOLUTION.y_px)
            .contains(&new_translation_y);

        if off_screen_x || off_screen_y {
            commands.entity(entity).despawn();
            return;
        }

        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;
    }
}
