use crate::{
    animation::AnimationInfo, player::PlayerBundle, prelude::*, AmmoBundle, PlayerSpawned,
    SpritesResources, WeaponBundle,
};

#[derive(Clone, Debug)]
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

#[derive(Component, Debug, Clone)]
pub struct Sprites<'a> {
    pub player_tile: SpriteInfo<'a>,
    pub gamestudio_tileset: SpriteInfo<'a>,
    pub player_char_walking: SpriteInfo<'a>,
    pub player_char_idle: SpriteInfo<'a>,
    pub player_custom_bg: SpriteInfo<'a>,
    pub enemy_char_idle: SpriteInfo<'a>,
    pub bow: SpriteInfo<'a>,
    pub arrow: SpriteInfo<'a>,
    pub wand: SpriteInfo<'a>,
    pub magic_ball: SpriteInfo<'a>,
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
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    render_background_texture(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites.0,
        &asset_server,
    );
    setup_player_sprite(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites.0,
        &asset_server,
        &sprites,
    );
}

fn setup_player_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Sprites<'static>,
    asset_server: &Res<AssetServer>,
    sprites_resources: &Res<SpritesResources>,
) {
    let player = PlayerBundle::idle(texture_atlas_layout, sprites, asset_server);

    let damage = AMMO_DAMAGE;
    let direction = Vec3::ZERO;
    let pos = Vec3::new(8.0, 0.0, CHAR_Z_INDEX);
    let weapon_scale = Vec3::new(0.5, 0.5, 1.);
    let weapon_type = WeaponTypeEnum::Bow;

    let weapon_bundle = WeaponBundle::new(
        texture_atlas_layout,
        sprites_resources,
        asset_server,
        weapon_scale,
        pos,
        direction,
        damage,
        weapon_type,
    );

    let scale = Vec3::ONE;
    let weapon_type = WeaponTypeEnum::default();
    let rotation = Quat::default();

    let ammo_bundle = AmmoBundle::new(
        texture_atlas_layout,
        sprites_resources,
        asset_server,
        scale,
        pos,
        weapon_type,
        direction,
        damage,
        rotation,
    );

    let ammo_entity_id = commands.spawn(ammo_bundle).id();
    let weapon_entity_id = commands.spawn(weapon_bundle).id();
    let player_entity_id = commands.spawn(player).id();

    commands
        .entity(weapon_entity_id)
        .push_children(&[ammo_entity_id]);
    commands
        .entity(player_entity_id)
        .push_children(&[weapon_entity_id]);

    commands.trigger(PlayerSpawned);
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
    let tile = sprites.player_custom_bg.clone();

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
