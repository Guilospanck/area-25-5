use crate::{
    animation::{AnimationIndices, AnimationInfo, AnimationTimer},
    player::AlienBundle,
    prelude::*,
};

#[derive(Clone, Debug)]
pub(crate) struct RectangularDimensions {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct SpriteInfo {
    pub(crate) dimensions: RectangularDimensions,
    pub(crate) source: Handle<Image>,
    pub(crate) animation: Option<AnimationInfo>,
    pub(crate) layout: TextureAtlasLayout,
}

#[derive(Component, Debug, Clone)]
pub(crate) struct Sprites {
    pub(crate) alien_tile: SpriteInfo,
    pub(crate) gamestudio_tileset: SpriteInfo,
    pub(crate) alien_char_walking: SpriteInfo,
    pub(crate) alien_char_idle: SpriteInfo,
    pub(crate) alien_custom_bg: SpriteInfo,
}

#[derive(Component)]
struct TileBackground;

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
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let sprites = get_sprites(&asset_server);
    commands.spawn(sprites.clone());

    render_background_texture(&mut commands, &mut texture_atlas_layout, &sprites);
    setup_alien_sprite(&mut commands, &mut texture_atlas_layout, &sprites, meshes);
}

fn setup_alien_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Sprites,
    meshes: ResMut<Assets<Mesh>>,
) {
    let alien = AlienBundle::idle(texture_atlas_layout, meshes, sprites);
    commands.spawn(alien);
}

fn setup_tile_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    x_offset: f32,
    y_offset: f32,
    tile_sprite: SpriteInfo,
) {
    let texture_atlas_layout = texture_atlas_layout.add(tile_sprite.layout);

    commands.spawn(TileBundle {
        marker: TileBackground,
        sprite: SpriteBundle {
            texture: tile_sprite.source,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(x_offset, y_offset, TILE_Z_INDEX),
                scale: Vec3::splat(1.),
            },
            ..default()
        },
        atlas: TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0usize,
        },
        layer: GAME_LAYER,
    });
}

fn render_background_texture(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Sprites,
) {
    let tile = sprites.alien_custom_bg.clone();

    // number of tiles in a row
    let x_items = WINDOW_RESOLUTION.x_px / tile.dimensions.width as f32;
    let x_items: u32 = x_items.ceil() as u32;

    // number of tiles in a column
    let y_items = WINDOW_RESOLUTION.y_px / tile.dimensions.height as f32;
    let y_items: u32 = y_items.ceil() as u32;

    for _ in 0..y_items {
        for _ in 0..x_items {
            setup_tile_sprite(commands, texture_atlas_layout, 0., 0., tile.clone());
        }
    }
}

fn get_sprites(asset_server: &Res<AssetServer>) -> Sprites {
    const ALIEN_PIXEL_SIZE: u32 = 32;
    const ALIEN_ANIMATION_TIMER: f32 = 0.1;
    // Alien tile
    const ALIEN_TILE_WIDTH: u32 = 95u32;
    const ALIEN_TILE_HEIGHT: u32 = 95u32;
    const ALIEN_TILE_OFFSET_X: u32 = 500u32;
    const ALIEN_TILE_OFFSET_Y: u32 = 623u32;

    Sprites {
        alien_tile: SpriteInfo {
            dimensions: RectangularDimensions {
                width: ALIEN_TILE_WIDTH,
                height: ALIEN_TILE_HEIGHT,
            },
            source: asset_server.load("textures/Tiles/alien.png"),
            animation: None,
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(ALIEN_TILE_WIDTH, ALIEN_TILE_HEIGHT),
                1,
                1,
                None,
                Some(UVec2::new(ALIEN_TILE_OFFSET_X, ALIEN_TILE_OFFSET_Y)),
            ),
        },
        gamestudio_tileset: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1361,
                height: 763,
            },
            source: asset_server.load("textures/Tiles/tileset.png"),
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1361, 763), 1, 1, None, None),
        },
        alien_custom_bg: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1920,
                height: 1080,
            },
            source: asset_server.load("textures/Background/Alien1.png"),
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 1, 1, None, None),
        },
        alien_char_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: ALIEN_PIXEL_SIZE,
                height: ALIEN_PIXEL_SIZE,
            },
            source: asset_server.load("textures/Alien/Alien_idle.png"),
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 3 },
                timer: AnimationTimer(Timer::from_seconds(
                    ALIEN_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(ALIEN_PIXEL_SIZE, ALIEN_PIXEL_SIZE),
                4,
                1,
                None,
                None,
            ),
        },
        alien_char_walking: SpriteInfo {
            dimensions: RectangularDimensions {
                width: ALIEN_PIXEL_SIZE,
                height: ALIEN_PIXEL_SIZE,
            },
            source: asset_server.load("textures/Alien/Alien_run.png"),
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 5 },
                timer: AnimationTimer(Timer::from_seconds(
                    ALIEN_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(ALIEN_PIXEL_SIZE, ALIEN_PIXEL_SIZE),
                6,
                1,
                None,
                None,
            ),
        },
    }
}
