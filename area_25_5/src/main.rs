use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
    window::WindowResolution,
};

const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
const TILE_Z_INDEX: f32 = 0.;
const CHAR_Z_INDEX: f32 = 1.;
const ALIEN_MOVE_SPEED: f32 = 100.0;

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
        .add_plugins((
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
            Wireframe2dPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(FixedUpdate, (animate_sprite, move_char, handle_click))
        .add_systems(Update, move_bullets)
        .observe(on_bullets_shot)
        .run();
}

fn on_bullets_shot(
    trigger: Trigger<ShootBullets>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    alien: Query<(&Transform, &Alien)>,
) {
    let event = trigger.event();
    let Vec2 { x, y } = event.pos;

    spawn_bullet(commands, meshes, materials, x, y, alien);
}

fn move_bullets(mut bullets: Query<(&mut Transform, &mut Bullet)>, timer: Res<Time>) {
    for (mut transform, bullet) in &mut bullets {
        // move along some direction vector
        transform.translation.x += bullet.direction.x * ALIEN_MOVE_SPEED * timer.delta_seconds();
        transform.translation.y -= bullet.direction.y * ALIEN_MOVE_SPEED * timer.delta_seconds();

        // transform.rotate_z(bullet.angle);
    }
}

#[derive(Clone, Debug)]
struct RectangularDimensions {
    width: u32,
    height: u32,
}

#[derive(Clone, Debug)]
struct SpriteInfo {
    dimensions: RectangularDimensions,
    source: Handle<Image>,
    animation: Option<AnimationInfo>,
    layout: TextureAtlasLayout,
}

#[derive(Component, Debug, Clone)]
struct Sprites {
    alien_tile: SpriteInfo,
    gamestudio_tileset: SpriteInfo,
    alien_char_walking: SpriteInfo,
    alien_char_idle: SpriteInfo,
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component, Debug)]
struct Alien;

#[derive(Component, Debug)]
struct Bullet {
    direction: Vec2,
    angle: f32,
}

#[derive(Component)]
struct TileBackground;

#[derive(Component, Deref, DerefMut, Clone, Debug)]
struct AnimationTimer(Timer);

#[derive(Component, Clone, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Clone, Debug)]
struct AnimationInfo {
    indices: AnimationIndices,
    timer: AnimationTimer,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), InGameCamera, GAME_LAYER));
}

fn setup_sprite(
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    fn _setup_common(asset_server: Res<AssetServer>) -> Sprites {
        const ALIEN_PIXEL_SIZE: u32 = 32;
        const ALIEN_ANIMATION_TIMER: f32 = 0.1;
        // Alien tile
        const ALIEN_TILE_WIDTH: u32 = 95u32;
        const ALIEN_TILE_HEIGHT: u32 = 95u32;
        const ALIEN_TILE_OFFSET_X: u32 = 500u32;
        const ALIEN_TILE_OFFSET_Y: u32 = 623u32;

        // Sprites
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

    let sprites = _setup_common(asset_server);
    commands.spawn(sprites.clone());

    render_background_texture(&mut commands, &mut texture_atlas_layout, &sprites);
    setup_alien_sprite(&mut commands, &mut texture_atlas_layout, &sprites);
}

fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    alien: Query<(&Transform, &Alien)>,
) {
    let shape = Mesh2dHandle(meshes.add(Capsule2d::new(4., 8.0)));
    let color = Color::BLACK;

    let alien_position = alien.get_single().unwrap();
    let position = Vec2::new(
        alien_position.0.translation.x,
        alien_position.0.translation.y,
    );

    let direction_x = x - position.x;
    let direction_y = -(y - position.y);
    let direction = Vec2::new(direction_x, direction_y);
    let unit_direction = _get_unit_vector(direction);

    let angle = unit_direction.y.atan2(unit_direction.x) * -1.;

    let bullet = Bullet {
        direction: unit_direction,
        angle,
    };
    let rotation = Quat::from_rotation_z(angle);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(x, y, 1.),
                scale: Vec3::new(1., 1., 1.),
                rotation,
            },
            ..default()
        },
        bullet,
        GAME_LAYER,
    ));
}

fn _get_unit_vector(vec: Vec2) -> Vec2 {
    let modulo_x: f32 = vec.x.powi(2);
    let modulo_y: f32 = vec.y.powi(2);
    let modulo: f32 = modulo_x + modulo_y;
    let modulo: f32 = modulo.sqrt();

    let normalized_direction_x = vec.x / modulo;
    let normalized_direction_y = vec.y / modulo;

    Vec2::new(normalized_direction_x, normalized_direction_y)
}

#[derive(Event)]
struct ShootBullets {
    pos: Vec2,
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(ShootBullets { pos });
        }
    }
}

#[derive(Bundle)]
struct AlienBundle {
    marker: Alien,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    layer: RenderLayers,
}

impl AlienBundle {
    fn idle(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites,
    ) -> Self {
        Self::_util(texture_atlas_layout, sprites.alien_char_idle.clone())
    }

    fn walking(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites,
    ) -> Self {
        Self::_util(texture_atlas_layout, sprites.alien_char_walking.clone())
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,

        alien_sprite: SpriteInfo,
    ) -> Self {
        let alien_animation = alien_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(alien_sprite.layout);

        AlienBundle {
            marker: Alien,
            sprite: SpriteBundle {
                texture: alien_sprite.source.clone(),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: Vec3::new(
                        -WINDOW_RESOLUTION.x_px / 2. + 50.,
                        WINDOW_RESOLUTION.y_px / 2. - 80.,
                        CHAR_Z_INDEX,
                    ),
                    scale: Vec3::new(2., 2., 1.),
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: alien_animation.indices.first,
            },
            animation_indices: alien_animation.indices,
            animation_timer: alien_animation.timer,
            layer: GAME_LAYER,
        }
    }
}

fn setup_alien_sprite(
    commands: &mut Commands,
    texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Sprites,
) {
    commands.spawn(AlienBundle::idle(texture_atlas_layout, sprites));
}

#[derive(Bundle)]
struct TileBundle {
    marker: TileBackground,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    layer: RenderLayers,
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
                scale: Vec3::new(1.4, 1.4, 1.),
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
    let tile = sprites.gamestudio_tileset.clone();

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
fn animate_sprite(
    time: Res<Time>,
    // This will get only entities that have all of these components
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

fn move_char(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Query<&mut Transform, With<Alien>>,
    time: Res<Time>,
) {
    let mut direction_x = 0.;
    let mut direction_y = 0.;

    let mut char_transform = transform.single_mut();

    // left move
    if keyboard_input.pressed(KeyCode::KeyH) {
        direction_x -= 1.0;
    }

    // right move
    if keyboard_input.pressed(KeyCode::KeyL) {
        direction_x += 1.0;
    }
    // top move
    if keyboard_input.pressed(KeyCode::KeyK) {
        direction_y += 1.0;
    }

    // bottom move
    if keyboard_input.pressed(KeyCode::KeyJ) {
        direction_y -= 1.0;
    }

    let old_pos_x = char_transform.translation.x;
    let old_pos_y = char_transform.translation.y;

    let char_new_pos_x = old_pos_x + direction_x * ALIEN_MOVE_SPEED * time.delta_seconds();
    let char_new_pos_y = old_pos_y + direction_y * ALIEN_MOVE_SPEED * time.delta_seconds();

    char_transform.translation.x = char_new_pos_x;
    char_transform.translation.y = char_new_pos_y;
}
