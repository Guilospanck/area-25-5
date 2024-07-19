use bevy::{prelude::*, render::view::RenderLayers, window::WindowResized};

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// In-game resolution width.
const RES_WIDTH: u32 = 640;

/// In-game resolution height.
const RES_HEIGHT: u32 = 640;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(Update, fit_canvas)
        .run();
}

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Foreground;

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/apartment_background.png"),
            transform: Transform::from_xyz(-40., 20., 2.),
            ..default()
        },
        Background,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/apartment_foreground.png"),
            transform: Transform::from_xyz(-40., 20., 3.),
            ..default()
        },
        Foreground,
        PIXEL_PERFECT_LAYERS,
    ));

    // Grid starts at top-left
    let texture_handle = asset_server.load("textures/player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(44), 6, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(32.0, 44.0),
                }),
                ..default()
            },
            texture: texture_handle,
            transform: Transform::from_xyz(-40., -20., 4.),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0usize,
        },
        Player,
        PIXEL_PERFECT_LAYERS,
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<InGameCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
