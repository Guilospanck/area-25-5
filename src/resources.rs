use crate::{
    prelude::*, AnimationIndices, AnimationInfo, AnimationTimer, RectangularDimensions, SpriteInfo,
    Sprites,
};

#[derive(Resource)]
pub struct CurrentWave(pub u16);

#[derive(Resource)]
pub struct CurrentScore(pub f32);

#[derive(Resource, Clone)]
pub struct TimePassed {
    pub minutes: u16,
    pub seconds: u16,
}

#[derive(Resource)]
pub struct EnemyWaves(pub [EnemyByLevel; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct WeaponWaves(pub [WeaponByLevel<'static>; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct ItemWaves(pub [ItemByLevel<'static>; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct SpritesResources(pub Sprites<'static>);

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Alive,
    Dead,
    Won,
}

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(CurrentWave(1));
    commands.insert_resource(EnemyWaves(ENEMIES_PER_WAVE));
    commands.insert_resource(WeaponWaves(WEAPONS_PER_WAVE));
    commands.insert_resource(ItemWaves(ITEMS_PER_WAVE));
    commands.insert_resource(SpritesResources(get_sprites()));
    commands.insert_resource(CurrentScore(0.));

    commands.insert_resource(TimePassed {
        minutes: 1,
        seconds: 0,
    });
}

pub fn get_sprites() -> Sprites<'static> {
    const PLAYER_PIXEL_SIZE: u32 = 32;
    const PLAYER_ANIMATION_TIMER: f32 = 0.1;
    // Player tile
    const PLAYER_TILE_WIDTH: u32 = 95u32;
    const PLAYER_TILE_HEIGHT: u32 = 95u32;
    const PLAYER_TILE_OFFSET_X: u32 = 500u32;
    const PLAYER_TILE_OFFSET_Y: u32 = 623u32;

    Sprites {
        player_tile: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_TILE_WIDTH,
                height: PLAYER_TILE_HEIGHT,
            },
            source: "textures/Tiles/player.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_TILE_WIDTH, PLAYER_TILE_HEIGHT),
                1,
                1,
                None,
                Some(UVec2::new(PLAYER_TILE_OFFSET_X, PLAYER_TILE_OFFSET_Y)),
            ),
        },
        gamestudio_tileset: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1361,
                height: 763,
            },
            source: "textures/Tiles/tileset.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1361, 763), 1, 1, None, None),
        },
        player_custom_bg: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1920,
                height: 1080,
            },
            source: "textures/Background/Alien1.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 1, 1, None, None),
        },
        player_char_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Alien/Alien_idle.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 3 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                4,
                1,
                None,
                None,
            ),
        },
        player_char_walking: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Alien/Alien_run.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 5 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                6,
                1,
                None,
                None,
            ),
        },
        enemy_char_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Enemy/Idle-Sheet.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 3 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                4,
                1,
                None,
                None,
            ),
        },
        bow: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Bow.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        arrow: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Arrow.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        wand: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Wand.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        magic_ball: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/MagicBall.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        speed_potion: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Effects/speed_potion.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        lightning: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Effects/lightning.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
    }
}
