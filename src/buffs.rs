use chrono::{NaiveTime, Utc};

use crate::{
    prelude::*, util::get_item_sprite_based_on_item_type, AnimationIndices, AnimationTimer, Armor,
    CleanupWhenPlayerDies, Health, Mana, Speed, SpritesResources,
};

/*
* These are things that the player will get throughout the game
* and they might be temporary or not.
* */

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub enum ShieldType {
    #[default]
    Physical,
    Magical,
}

#[cfg_attr(
    not(feature = "web"),
    derive(Reflect, Component, Default, Debug, Clone)
)]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Default, Debug, Clone))]
pub struct Shield {
    pub offensive: f32,
    pub defensive: f32,
    pub shield_type: ShieldType,
    pub duration_seconds: Option<u64>,
}

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone))]
pub enum ItemTypeEnum {
    Speed(Speed),
    Armor(Armor),
    Shield(Shield),
    Health(Health),
    Mana(Mana),
}

impl Default for ItemTypeEnum {
    fn default() -> Self {
        Self::Speed(Speed::default())
    }
}

#[derive(Component, Debug, Clone)]
pub struct Buff {
    pub start_time: NaiveTime,
    pub item: ItemTypeEnum,
}

#[derive(Component, Debug, Clone)]
pub struct BuffGroup {
    pub start_time: NaiveTime,
    pub item: ItemTypeEnum,
}

#[derive(Bundle, Clone)]
pub(crate) struct BuffGroupBundle {
    pub(crate) sprite: SpriteBundle,
    pub(crate) marker: BuffGroup,
    pub(crate) layer: RenderLayers,
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
}

#[derive(Bundle, Clone)]
pub(crate) struct BuffBundle {
    pub(crate) marker: Buff,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
}

impl BuffGroupBundle {
    pub(crate) fn new(item_type: ItemTypeEnum, layer: RenderLayers) -> Self {
        Self::_util(item_type, layer)
    }

    fn _util(item_type: ItemTypeEnum, layer: RenderLayers) -> Self {
        let start_time = Utc::now().time();
        let buff_group = BuffGroup {
            item: item_type,
            start_time,
        };

        BuffGroupBundle {
            name: Name::new("BuffGroup"),
            marker: buff_group,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::splat(0.),
                    ..default()
                },
                ..default()
            },
            layer,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

impl BuffBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        item_type: ItemTypeEnum,
        layer: RenderLayers,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            pos,
            item_type,
            layer,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        item_type: ItemTypeEnum,
        layer: RenderLayers,
    ) -> Self {
        let item_sprite = get_item_sprite_based_on_item_type(item_type.clone(), sprites);
        let item_animation = item_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(item_sprite.layout);

        let start_time = Utc::now().time();
        let buff = Buff {
            item: item_type,
            start_time,
        };

        BuffBundle {
            name: Name::new("Buff"),
            marker: buff,
            sprite: SpriteBundle {
                texture: asset_server.load(item_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale,
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: item_animation.indices.first,
            },
            animation_indices: item_animation.indices,
            animation_timer: item_animation.timer,
            layer,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}
