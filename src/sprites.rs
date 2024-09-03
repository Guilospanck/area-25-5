use crate::{
    animation::AnimationInfo, prelude::*, util::get_background_texture_based_on_game_level,
    CurrentGameLevel, SpritesResources,
};

#[derive(Clone, Debug, Default)]
pub struct RectangularDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct SpriteInfo<'a> {
    pub dimensions: RectangularDimensions,
    pub source: &'a str,
    pub animation: Option<AnimationInfo>,
    pub layout: TextureAtlasLayout,
}

impl Default for SpriteInfo<'_> {
    fn default() -> Self {
        Self {
            dimensions: RectangularDimensions::default(),
            source: "",
            animation: None,
            layout: TextureAtlasLayout::new_empty(UVec2::ZERO),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Sprites<'a> {
    // player
    pub player_char_walking: SpriteInfo<'a>,
    pub player_char_idle: SpriteInfo<'a>,
    // levels
    pub level_1_bg: SpriteInfo<'a>,
    pub level_2_bg: SpriteInfo<'a>,
    pub level_3_bg: SpriteInfo<'a>,
    // enemies
    pub orc_idle: SpriteInfo<'a>,
    pub mage_idle: SpriteInfo<'a>,
    // weapons
    pub bow: SpriteInfo<'a>,
    pub wand: SpriteInfo<'a>,
    // ammos
    pub arrow: SpriteInfo<'a>,
    pub magic_ball: SpriteInfo<'a>,
    // items
    pub speed_potion: SpriteInfo<'a>,
    pub lightning: SpriteInfo<'a>,
    pub shield: SpriteInfo<'a>,
    pub hp_pack: SpriteInfo<'a>,
    pub diamond: SpriteInfo<'a>,
    // ui
    pub profile: SpriteInfo<'a>,
    // powers
    pub mine_bomb: SpriteInfo<'a>,
    pub laser: SpriteInfo<'a>,
    pub circle_of_death: SpriteInfo<'a>,
}

#[derive(Component)]
pub struct TileBackground;

#[derive(Bundle)]
struct TileBundle {
    marker: TileBackground,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    layer: RenderLayers,
}

pub fn setup_sprite(
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
    current_game_level: Res<CurrentGameLevel>,
) {
    render_background_texture(
        &mut commands,
        &mut texture_atlas_layout,
        &asset_server,
        &sprites,
        current_game_level.0,
    );
}

fn setup_tile_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    x_offset: f32,
    y_offset: f32,
    tile_sprite: SpriteInfo<'static>,
    asset_server: &Res<AssetServer>,
) {
    let texture_atlas_layout = texture_atlas_layout.add(tile_sprite.layout);

    commands.spawn(TileBundle {
        marker: TileBackground,
        sprite: SpriteBundle {
            texture: asset_server.load(tile_sprite.source),
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(x_offset, y_offset, TILE_Z_INDEX),
                scale: Vec3::splat(BACKGROUND_TEXTURE_SCALE),
            },
            ..default()
        },
        atlas: TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0usize,
        },
        layer: BASE_LAYER,
    });
}

pub(crate) fn render_background_texture(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    sprites: &Res<SpritesResources>,
    game_level: u16,
) {
    let tile = get_background_texture_based_on_game_level(game_level, sprites);

    // number of tiles in a row
    let x_items = BACKGROUND_TEXTURE_RESOLUTION.x_px / tile.dimensions.width as f32;
    let x_items: u32 = x_items.ceil() as u32;

    // number of tiles in a column
    let y_items = BACKGROUND_TEXTURE_RESOLUTION.y_px / tile.dimensions.height as f32;
    let y_items: u32 = y_items.ceil() as u32;

    for _ in 0..y_items {
        for _ in 0..x_items {
            setup_tile_sprite(
                commands,
                texture_atlas_layout,
                0.,
                0.,
                tile.clone(),
                asset_server,
            );
        }
    }
}
