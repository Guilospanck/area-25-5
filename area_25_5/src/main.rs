use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    reflect::List,
    render::view::RenderLayers,
};

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

// Player related
const PLAYER_SPEED: f32 = 100.;
const PLAYER_TEXTURE_CHAR_WIDTH: u32 = 32;
const PLAYER_TEXTURE_CHAR_HEIGHT: u32 = 46;
const PLAYER_TEXTURE_COLUMNS: u32 = 6;
const PLAYER_TEXTURE_ROWS: u32 = 8;
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

/*
Foreground (800x600):
Top-left: (-400, 300)
Top-right: (400, 300)
Bottom-left: (-400, -300)
Bottom-right: (400, -300)
Black-empty-rect: (-400, 300) (400, 197)

First obs: 138, 62   =>
    rect: (-400, 197) (-262, 135)
    center: (-331, 166)

-400, 238  =>  -262, 300


Problems:
- player collision should think about his feet;

TODO:
- add other obstacles for the fg and bg
- limit movement of the player when obstacle (but not too much,
he needs to be able to get out of that place :D)

*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .insert_resource(IsOnObstacle(false))
        .add_systems(Startup, (setup_camera, setup_ui, setup_sprite))
        .add_systems(
            FixedUpdate,
            (move_char, animate_sprite, check_for_collisions),
        )
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

#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
struct IsOnObstacle(bool);

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

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/apartment_background.png"),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        Background,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/apartment_foreground.png"),
            transform: Transform::from_xyz(0., 0., 3.),
            ..default()
        },
        Foreground,
        PIXEL_PERFECT_LAYERS,
        Collider,
    ));

    // Grid starts at top-left
    let texture_handle = asset_server.load("textures/player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_TEXTURE_CHAR_WIDTH, PLAYER_TEXTURE_CHAR_HEIGHT),
        PLAYER_TEXTURE_COLUMNS,
        PLAYER_TEXTURE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        first: PLAYER_FACING_FORWARD_STAND_STILL.0,
        last: PLAYER_FACING_FORWARD_STAND_STILL.1,
    };

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(0., 0., 4.),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_STAND_STILL_TIMER,
            TimerMode::Repeating,
        )),
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

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(asset_server.load("textures/profile.png")),
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(60.0),
                        right: Val::Px(50.0),
                        width: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Larry",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 10.0,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(138.0),
                    height: Val::Px(62.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..default()
                },
                background_color: Color::srgb(0.4, 0.4, 1.).into(),
                ..default()
            });
        });
}

fn move_char(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut animate_query: Query<(
        &mut AnimationIndices,
        &mut TextureAtlas,
        &mut AnimationTimer,
    )>,
    obstacle: Res<IsOnObstacle>,
) {
    if **obstacle {
        return;
    }

    let mut direction_x = 0.;
    let mut direction_y = 0.;
    let mut char_transform = query.single_mut();
    let (mut animate, mut atlas, mut timer) = animate_query.single_mut();

    // left
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_WALKING_TIMER,
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
            PLAYER_ANIMATION_WALKING_TIMER,
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

    // down
    if keyboard_input.just_pressed(KeyCode::KeyJ) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_WALKING_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_FORWARD_WALKING.0;
        animate.first = PLAYER_FACING_FORWARD_WALKING.0;
        animate.last = PLAYER_FACING_FORWARD_WALKING.1;
    }
    if keyboard_input.pressed(KeyCode::KeyJ) {
        direction_y -= 1.0;
    }
    if keyboard_input.just_released(KeyCode::KeyJ) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_STAND_STILL_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_FORWARD_STAND_STILL.0;
        animate.first = PLAYER_FACING_FORWARD_STAND_STILL.0;
        animate.last = PLAYER_FACING_FORWARD_STAND_STILL.1;
    }

    if keyboard_input.just_pressed(KeyCode::KeyK) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_WALKING_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_BACK_WALKING.0;
        animate.first = PLAYER_FACING_BACK_WALKING.0;
        animate.last = PLAYER_FACING_BACK_WALKING.1;
    }
    if keyboard_input.pressed(KeyCode::KeyK) {
        direction_y += 1.0;
    }
    if keyboard_input.just_released(KeyCode::KeyK) {
        *timer = AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_STAND_STILL_TIMER,
            TimerMode::Repeating,
        ));
        atlas.index = PLAYER_FACING_BACK_STAND_STILL.0;
        animate.first = PLAYER_FACING_BACK_STAND_STILL.0;
        animate.last = PLAYER_FACING_BACK_STAND_STILL.1;
    }

    char_transform.translation.x += direction_x * PLAYER_SPEED * time.delta_seconds();
    char_transform.translation.y += direction_y * PLAYER_SPEED * time.delta_seconds();
}

fn check_for_collisions(
    mut player_query: Query<&Transform, With<Player>>,
    mut obstacle: ResMut<IsOnObstacle>,
) {
    let player_transform = player_query.single_mut();

    let foreground_objects: Vec<Aabb2d> =
        vec![Aabb2d::new(Vec2::new(-331., 166.), Vec2::new(69., 31.))];

    let collided = check_player_collision(
        Aabb2d::new(
            player_transform.translation.truncate(),
            player_transform.scale.truncate() / 2.,
        ),
        foreground_objects,
    );

    **obstacle = collided;
}

fn check_player_collision(player: Aabb2d, obstacles: Vec<Aabb2d>) -> bool {
    obstacles.iter().any(|obstacle| {
        let obs = obstacle.downcast_ref::<Aabb2d>().unwrap();
        player.intersects(obs)
    })
}
