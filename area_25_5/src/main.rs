use bevy::{prelude::*, render::view::RenderLayers};

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(FixedUpdate, animate_sprite)
        .run();
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct AlienIdle;

#[derive(Component)]
struct AlienRun;

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
    // Grid starts at top-left
    let texture_handle = asset_server.load("textures/Alien/Alien_idle.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(ALIEN_PIXEL_SIZE, ALIEN_PIXEL_SIZE),
        4,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(4., 4., 0.),
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

    // Grid starts at top-left
    let texture_handle = asset_server.load("textures/Alien/Alien_run.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(ALIEN_PIXEL_SIZE, ALIEN_PIXEL_SIZE),
        6,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                rotation: Quat::default(),
                translation: Vec3::new(64., 0., 1.),
                scale: Vec3::new(4., 4., 0.),
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
