use bevy::{
    asset::AssetPath, ecs::observer::TriggerTargets, prelude::*, render::view::RenderLayers,
    window::WindowResolution,
};

const GAME_LAYER: RenderLayers = RenderLayers::layer(0);
const TILE_Z_INDEX: f32 = 0.;
const CHAR_Z_INDEX: f32 = 1.;

const ANIMATION_TIMER: f32 = 0.1;
const ALIEN_PIXEL_SIZE: u32 = 32;
const ALIEN_SPEED: f32 = 100.;
const ALIEN_ANIMATION_TIMER: f32 = 0.1;

// Player not moving
const PLAYER_ANIMATION_STAND_STILL_TIMER: f32 = 0.1;
const PLAYER_FACING_FORWARD_STAND_STILL: (usize, usize) = (0, 5);
const PLAYER_FACING_LEFT_STAND_STILL: (usize, usize) = (6, 11);
const PLAYER_FACING_BACK_STAND_STILL: (usize, usize) = (12, 17);
const PLAYER_FACING_RIGHT_STAND_STILL: (usize, usize) = (18, 23);
// Player Walking
const PLAYER_ANIMATION_WALKING_TIMER: f32 = 0.1;
const PLAYER_FACING_FORWARD_WALKING: (usize, usize) = (24, 29);
const PLAYER_FACING_LEFT_WALKING: (usize, usize) = (30, 35);
const PLAYER_FACING_BACK_WALKING: (usize, usize) = (36, 41);
const PLAYER_FACING_RIGHT_WALKING: (usize, usize) = (42, 46);

struct CustomWindowResolution {
    x_px: f32,
    y_px: f32,
}

const WINDOW_RESOLUTION: CustomWindowResolution = CustomWindowResolution {
    x_px: 1920.0,
    y_px: 1080.0,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_RESOLUTION.x_px,
                            WINDOW_RESOLUTION.y_px,
                        )
                        .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(FixedUpdate, (animate_sprite, move_char))
        .run();
}

#[derive(Clone, Copy)]
struct RectangularDimensions {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
struct TileInfo<'a> {
    dimensions: RectangularDimensions,
    offset_x: u32,
    offset_y: u32,
    source: &'a str,
}

// Whole spritesheet (RunnerTileSet.png): 432x192
const BLACK_TILE_DIMENSIONS: TileInfo = TileInfo {
    dimensions: RectangularDimensions {
        width: 97u32,
        height: 63u32,
    },
    offset_y: 129u32,
    offset_x: 96u32,
    source: "textures/Tiles/RunnerTileSet.png",
};

// 95x95 tile
// Whole spritesheet (tiles/alien.png): 1280x731
const ALIEN_TERRAIN: TileInfo = TileInfo {
    dimensions: RectangularDimensions {
        width: 95u32,
        height: 95u32,
    },
    offset_y: 623u32,
    offset_x: 500u32,
    source: "textures/Tiles/alien.png",
};

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct AlienIdle;

#[derive(Component)]
struct AlienWalking;

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
    commands.spawn((Camera2dBundle::default(), InGameCamera, GAME_LAYER));
}

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    render_tiles_to_bottom(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        ALIEN_TERRAIN,
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
                translation: Vec3::new(199., 0., CHAR_Z_INDEX),
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
        GAME_LAYER,
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
                translation: Vec3::new(0., 0., CHAR_Z_INDEX),
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
        AlienWalking,
        GAME_LAYER,
    ));
}

fn setup_tile_sprite(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tile: TileInfo,
    x_offset: f32,
    y_offset: f32,
) {
    let texture_handle: Handle<Image> = asset_server.load(tile.source.to_string());
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
            texture: texture_handle.clone(),
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(x_offset, y_offset, TILE_Z_INDEX),
                scale: Vec3::new(1., 1., 1.),
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0usize,
        },
        TileBackground,
        GAME_LAYER,
    ));
}

fn render_tiles_to_bottom(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tile: TileInfo,
) {
    let origin = Vec2::new(-WINDOW_RESOLUTION.x_px / 2., WINDOW_RESOLUTION.y_px / 2.);

    // number of tiles in a row
    let x_items = WINDOW_RESOLUTION.x_px / tile.dimensions.width as f32;
    let x_items: u32 = x_items.ceil() as u32;

    // number of tiles in a column
    let y_items = WINDOW_RESOLUTION.y_px / tile.dimensions.height as f32;
    let y_items: u32 = y_items.ceil() as u32;

    let y_offset: f32 =
        origin.y - (y_items * tile.dimensions.height - (tile.dimensions.height)) as f32;
    let mut x_offset: f32 = origin.x + (tile.dimensions.width / 2) as f32;

    for j in 0..x_items {
        if j != 0 {
            x_offset += tile.dimensions.width as f32;
        }

        setup_tile_sprite(
            commands,
            asset_server,
            texture_atlas_layouts,
            tile,
            x_offset,
            y_offset,
        );
    }
}

fn render_tiles_on_whole_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tile: TileInfo,
) {
    let origin = Vec2::new(-WINDOW_RESOLUTION.x_px / 2., WINDOW_RESOLUTION.y_px / 2.);

    // number of tiles in a row
    let x_items = WINDOW_RESOLUTION.x_px / tile.dimensions.width as f32;
    let x_items: u32 = x_items.ceil() as u32;

    // number of tiles in a column
    let y_items = WINDOW_RESOLUTION.y_px / tile.dimensions.height as f32;
    let y_items: u32 = y_items.ceil() as u32;

    let mut y_offset: f32 = origin.y - (tile.dimensions.height / 2) as f32;

    for i in 0..y_items {
        let mut x_offset: f32 = origin.x + (tile.dimensions.width / 2) as f32;
        if i != 0 {
            y_offset -= tile.dimensions.height as f32;
        }

        for j in 0..x_items {
            if j != 0 {
                x_offset += tile.dimensions.width as f32;
            }

            setup_tile_sprite(
                commands,
                asset_server,
                texture_atlas_layouts,
                tile,
                x_offset,
                y_offset,
            );
        }
    }
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

fn move_char(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<AlienIdle>>,
    time: Res<Time>,
    mut animate_query: Query<(
        &mut AnimationIndices,
        &mut TextureAtlas,
        &mut AnimationTimer,
    )>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    let mut char_transform = query.single_mut();
    println!("sjdfjas");

    let (mut animate, mut atlas, mut timer) = animate_query.single_mut();

    // left
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        *timer = AnimationTimer(Timer::from_seconds(
            ALIEN_ANIMATION_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_LEFT_WALKING.0;
        animate.first = PLAYER_FACING_LEFT_WALKING.0;
        animate.last = PLAYER_FACING_LEFT_WALKING.1;
    }
    if keyboard_input.pressed(KeyCode::KeyH) {
        direction_x -= 1.0;
    }
    if keyboard_input.just_released(KeyCode::KeyH) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_STAND_STILL_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_LEFT_STAND_STILL.0;
        animate.first = PLAYER_FACING_LEFT_STAND_STILL.0;
        animate.last = PLAYER_FACING_LEFT_STAND_STILL.1;
    }

    // right
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        *timer = AnimationTimer(Timer::from_seconds(
            ALIEN_ANIMATION_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_RIGHT_WALKING.0;
        animate.first = PLAYER_FACING_RIGHT_WALKING.0;
        animate.last = PLAYER_FACING_RIGHT_WALKING.1;
    }
    if keyboard_input.pressed(KeyCode::KeyL) {
        direction_x += 1.0;
    }
    if keyboard_input.just_released(KeyCode::KeyL) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_STAND_STILL_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_RIGHT_STAND_STILL.0;
        animate.first = PLAYER_FACING_RIGHT_STAND_STILL.0;
        animate.last = PLAYER_FACING_RIGHT_STAND_STILL.1;
    }

    let old_pos_x = char_transform.translation.x;
    let old_pos_y = char_transform.translation.y;

    let char_new_pos_x = old_pos_x + direction_x * ALIEN_SPEED * time.delta_seconds();
    let char_new_pos_y = old_pos_y + direction_y * ALIEN_SPEED * time.delta_seconds();

    char_transform.translation.x = char_new_pos_x;
    char_transform.translation.y = char_new_pos_y;
}
