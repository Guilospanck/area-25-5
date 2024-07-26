use crate::{
    animation::AnimationInfo, player::AlienBundle, prelude::*, AlienSpawned, Enemy, EnemyBundle,
    SpritesResources,
};

#[derive(Clone, Debug)]
pub(crate) struct RectangularDimensions {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct SpriteInfo<'a> {
    pub(crate) dimensions: RectangularDimensions,
    pub(crate) source: &'a str,
    pub(crate) animation: Option<AnimationInfo>,
    pub(crate) layout: TextureAtlasLayout,
}

#[derive(Component, Debug, Clone)]
pub(crate) struct Sprites<'a> {
    pub(crate) alien_tile: SpriteInfo<'a>,
    pub(crate) gamestudio_tileset: SpriteInfo<'a>,
    pub(crate) alien_char_walking: SpriteInfo<'a>,
    pub(crate) alien_char_idle: SpriteInfo<'a>,
    pub(crate) alien_custom_bg: SpriteInfo<'a>,
    pub(crate) enemy_char_idle: SpriteInfo<'a>,
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
    meshes: ResMut<Assets<Mesh>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(sprites.0.clone());

    render_background_texture(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites.0,
        &asset_server,
    );
    setup_alien_sprite(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites,
        meshes,
        &asset_server,
    );
    setup_enemy_sprite(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
    );
}

fn setup_alien_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: &Res<AssetServer>,
) {
    let alien = AlienBundle::idle(texture_atlas_layout, meshes, &sprites.0, asset_server);
    commands.spawn(alien);
    commands.trigger(AlienSpawned);
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
            texture: asset_server.load(tile_sprite.source.clone()),
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
    sprites: &Sprites<'static>,
    asset_server: &Res<AssetServer>,
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

fn setup_enemy_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: &Res<AssetServer>,
) {
    let enemy = Enemy {
        pos: Vec3::splat(1.),
        health: ENEMY_HEALTH,
        damage: ENEMY_DAMAGE,
    };
    let enemy = EnemyBundle::idle(texture_atlas_layout, &sprites.0, asset_server, enemy);
    commands.spawn(enemy);
}
