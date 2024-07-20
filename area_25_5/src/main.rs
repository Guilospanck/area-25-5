use bevy::{asset::AssetPath, prelude::*, render::view::RenderLayers};

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(FixedUpdate, animate_sprite)
        .run();
}

struct RectangularDimensions {
    width: u32,
    height: u32,
}

struct TileInfo {
    dimensions: RectangularDimensions,
    offset_x: u32,
    offset_y: u32,
}

// Whole spritesheet: 432x192
const BLACK_TILE_DIMENSIONS: TileInfo = TileInfo {
    dimensions: RectangularDimensions {
        width: 97u32,
        height: 63u32,
    },
    offset_y: 129u32,
    offset_x: 96u32,
};

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct AlienIdle;

#[derive(Component)]
struct AlienRun;

#[derive(Component)]
struct TileBackground;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));
}

const ANIMATION_TIMER: f32 = 0.1;
const ALIEN_PIXEL_SIZE: u32 = 32;

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    setup_tile_sprite(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        BLACK_TILE_DIMENSIONS,
    );
    setup_alien_idle_sprite(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        "textures/Alien/Alien_idle.png".into(),
        4,
        1,
    );
    setup_alien_run_sprite(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        "textures/Alien/Alien_run.png".into(),
        6,
        1,
    );
}

fn setup_alien_idle_sprite(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_path: AssetPath,
    columns: u32,
    rows: u32,
) {
    let (texture_handle, texture_atlas_layout, animation_indices) =
        _get_texture_atlas_and_animation_indices(
            asset_server,
            texture_atlas_layouts,
            asset_path,
            columns,
            rows,
        );

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(199., 0., 1.),
                scale: Vec3::new(4., 4., 1.),
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(ANIMATION_TIMER, TimerMode::Repeating)),
        AlienIdle,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn setup_alien_run_sprite(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_path: AssetPath,
    columns: u32,
    rows: u32,
) {
    let (texture_handle, texture_atlas_layout, animation_indices) =
        _get_texture_atlas_and_animation_indices(
            asset_server,
            texture_atlas_layouts,
            asset_path,
            columns,
            rows,
        );

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(4., 0., 1.),
                scale: Vec3::new(4., 4., 1.),
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(ANIMATION_TIMER, TimerMode::Repeating)),
        AlienRun,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn _get_texture_atlas_and_animation_indices(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_path: AssetPath,
    columns: u32,
    rows: u32,
) -> (Handle<Image>, Handle<TextureAtlasLayout>, AnimationIndices) {
    // Grid starts at top-left
    let texture_handle: Handle<Image> = asset_server.load(asset_path);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(ALIEN_PIXEL_SIZE, ALIEN_PIXEL_SIZE),
        columns,
        rows,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let last = columns - 1u32;
    let last: usize = last.try_into().unwrap();
    let animation_indices = AnimationIndices { first: 0, last };

    (texture_handle, texture_atlas_layout, animation_indices)
}

fn setup_tile_sprite(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tile: TileInfo,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/Tiles/RunnerTileSet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(tile.dimensions.width, tile.dimensions.height),
        1,
        1,
        None,
        Some(UVec2::new(tile.offset_x, tile.offset_y)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0usize,
        },
        TileBackground,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
