use bevy::{prelude::*, render::view::RenderLayers, window::WindowResolution};

const GAME_LAYER: RenderLayers = RenderLayers::layer(0);
const TILE_Z_INDEX: f32 = 0.;
const CHAR_Z_INDEX: f32 = 1.;

const ALIEN_PIXEL_SIZE: u32 = 32;

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
        .add_systems(Startup, (setup_resources, setup_camera, setup_sprite))
        .add_systems(FixedUpdate, animate_sprite)
        .run();
}

#[derive(Clone, Copy)]
struct RectangularDimensions {
    width: u32,
    height: u32,
}

#[derive(Component, Clone)]
struct SpriteInfo {
    dimensions: RectangularDimensions,
    offset_x: u32,
    offset_y: u32,
    source: Handle<Image>,
}

#[derive(Resource)]
struct Sprites {
    black_tile: SpriteInfo,
    alien_tile: SpriteInfo,
    alien_char_walking: SpriteInfo,
    alien_char_idle: SpriteInfo,
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct Alien;

#[derive(Component)]
struct TileBackground;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Sprites
    commands.insert_resource(Sprites {
        black_tile: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 97u32,
                height: 63u32,
            },
            offset_y: 129u32,
            offset_x: 96u32,
            source: asset_server.load("textures/Tiles/RunnerTileSet.png"),
        },
        alien_tile: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 95u32,
                height: 95u32,
            },
            offset_y: 623u32,
            offset_x: 500u32,
            source: asset_server.load("textures/Tiles/alien.png"),
        },
        alien_char_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32u32,
                height: 32u32,
            },
            offset_y: 0u32,
            offset_x: 0u32,
            source: asset_server.load("textures/Alien/Alien_idle.png"),
        },
        alien_char_walking: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32u32,
                height: 32u32,
            },
            offset_y: 0u32,
            offset_x: 0u32,
            source: asset_server.load("textures/Alien/Alien_run.png"),
        },
    });
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), InGameCamera, GAME_LAYER));
}

fn setup_sprite(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<Sprites>,
) {
    render_tiles_to_bottom(&mut commands, &mut texture_atlas_layouts, &sprites);
    setup_alien_sprite(&mut commands, &mut texture_atlas_layouts, 4, 1, &sprites);
    // setup_alien_run_sprite(&mut commands, &mut texture_atlas_layouts, 6, 1, &sprites);
}

#[derive(Bundle)]
struct AlienBundle {
    marker: Alien,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    // animation_idx: ads,
    // animation_timer: ads,
    layer: RenderLayers,
}

fn setup_alien_sprite(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    columns: u32,
    rows: u32,
    sprites: &Res<Sprites>,
) {
    // Grid starts at top-left
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

    commands.spawn(AlienBundle {
        marker: Alien,
        sprite: SpriteBundle {
            texture: sprites.alien_char_idle.source.clone(),
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(199., 0., CHAR_Z_INDEX),
                scale: Vec3::new(4., 4., 1.),
            },
            ..default()
        },
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        // animation_indices,
        // AnimationTimer(Timer::from_seconds(ANIMATION_TIMER, TimerMode::Repeating)),
        layer: GAME_LAYER,
    });
}

#[derive(Bundle)]
struct TileBundle {
    marker: TileBackground,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    // animation_idx: ads,
    // animation_timer: ads,
    layer: RenderLayers,
}

fn setup_tile_sprite(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    x_offset: f32,
    y_offset: f32,
    tile: SpriteInfo,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(tile.dimensions.width, tile.dimensions.height),
        1,
        1,
        None,
        Some(UVec2::new(tile.offset_x, tile.offset_y)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(TileBundle {
        marker: TileBackground,
        sprite: SpriteBundle {
            texture: tile.source,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(x_offset, y_offset, TILE_Z_INDEX),
                scale: Vec3::new(1., 1., 1.),
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

fn render_tiles_to_bottom(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<Sprites>,
) {
    let tile = sprites.alien_tile.clone();
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
            texture_atlas_layouts,
            x_offset,
            y_offset,
            tile.clone(),
        );
    }
}

fn render_tiles_on_whole_screen(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<Sprites>,
) {
    let tile = sprites.alien_tile.clone();
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
                texture_atlas_layouts,
                x_offset,
                y_offset,
                tile.clone(),
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

// fn move_char(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<AlienIdle>>,
//     time: Res<Time>,
//     mut animate_query: Query<(
//         &mut AnimationIndices,
//         &mut TextureAtlas,
//         &mut AnimationTimer,
//     )>,
// ) {
//     let mut direction_x = 0.;
//     let mut direction_y = 0.;
//
//     let mut char_transform = query.single_mut();
//
//     let (mut animate, mut atlas, mut timer) = animate_query.single_mut();
//
//     // left
//     if keyboard_input.just_pressed(KeyCode::KeyH) {
//         *timer = AnimationTimer(Timer::from_seconds(
//             ALIEN_ANIMATION_TIMER,
//             TimerMode::Repeating,
//         ));
//         atlas.index = PLAYER_FACING_LEFT_WALKING.0;
//         animate.first = PLAYER_FACING_LEFT_WALKING.0;
//         animate.last = PLAYER_FACING_LEFT_WALKING.1;
//     }
//     if keyboard_input.pressed(KeyCode::KeyH) {
//         direction_x -= 1.0;
//     }
//     if keyboard_input.just_released(KeyCode::KeyH) {
//         *timer = AnimationTimer(Timer::from_seconds(
//             PLAYER_ANIMATION_STAND_STILL_TIMER,
//             TimerMode::Repeating,
//         ));
//         atlas.index = PLAYER_FACING_LEFT_STAND_STILL.0;
//         animate.first = PLAYER_FACING_LEFT_STAND_STILL.0;
//         animate.last = PLAYER_FACING_LEFT_STAND_STILL.1;
//     }
//
//     // right
//     if keyboard_input.just_pressed(KeyCode::KeyL) {
//         *timer = AnimationTimer(Timer::from_seconds(
//             ALIEN_ANIMATION_TIMER,
//             TimerMode::Repeating,
//         ));
//         atlas.index = PLAYER_FACING_RIGHT_WALKING.0;
//         animate.first = PLAYER_FACING_RIGHT_WALKING.0;
//         animate.last = PLAYER_FACING_RIGHT_WALKING.1;
//     }
//     if keyboard_input.pressed(KeyCode::KeyL) {
//         direction_x += 1.0;
//     }
//     if keyboard_input.just_released(KeyCode::KeyL) {
//         *timer = AnimationTimer(Timer::from_seconds(
//             PLAYER_ANIMATION_STAND_STILL_TIMER,
//             TimerMode::Repeating,
//         ));
//         atlas.index = PLAYER_FACING_RIGHT_STAND_STILL.0;
//         animate.first = PLAYER_FACING_RIGHT_STAND_STILL.0;
//         animate.last = PLAYER_FACING_RIGHT_STAND_STILL.1;
//     }
//
//     let old_pos_x = char_transform.translation.x;
//     let old_pos_y = char_transform.translation.y;
//
//     let char_new_pos_x = old_pos_x + direction_x * ALIEN_SPEED * time.delta_seconds();
//     let char_new_pos_y = old_pos_y + direction_y * ALIEN_SPEED * time.delta_seconds();
//
//     char_transform.translation.x = char_new_pos_x;
//     char_transform.translation.y = char_new_pos_y;
// }
