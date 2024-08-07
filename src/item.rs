use crate::{
    prelude::*,
    util::{get_item_sprite_based_on_item_type, get_random_vec3},
    AnimationIndices, AnimationTimer, CleanupWhenPlayerDies, ItemTypeEnum, SpritesResources,
};
use rand::Rng;

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Item {
    pub item_type: ItemTypeEnum,
}

#[derive(Bundle, Clone)]
pub(crate) struct ItemBundle {
    pub(crate) marker: Item,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
}

impl ItemBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        item_type: ItemTypeEnum,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            pos,
            item_type,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        item_type: ItemTypeEnum,
    ) -> Self {
        let item_sprite = get_item_sprite_based_on_item_type(item_type.clone(), sprites);
        let item_animation = item_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(item_sprite.layout);

        // The base layer in which item is being rendered on is being scaled
        // by BASE_CAMERA_PROJECTION_SCALE, therefore we must change the item
        // position to be able to render items on the whole background "map"
        let new_pos = pos / BASE_CAMERA_PROJECTION_SCALE;

        ItemBundle {
            name: Name::new("Item"),
            marker: Item { item_type },
            sprite: SpriteBundle {
                texture: asset_server.load(item_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: new_pos,
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
            layer: BASE_LAYER,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    item_by_level: &ItemByLevel,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    let quantity = &item_by_level.quantity;
    let item_type = &item_by_level.item.item;
    let scale = Vec3::splat(2.);

    for idx in 1..=*quantity {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let random_spawning_pos = get_random_vec3(idx as u64, Some(n1 as u64 * ITEM_RANDOM_SEED));

        let bundle = ItemBundle::new(
            &mut texture_atlas_layout,
            sprites,
            &asset_server,
            scale,
            random_spawning_pos,
            item_type.clone(),
        );

        commands.spawn(bundle);
    }
}
