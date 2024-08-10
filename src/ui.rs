use bevy::{
    color::palettes::css::YELLOW,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    prelude::*, util::get_item_sprite_based_on_item_type, Buff, CurrentScore, ItemTypeEnum, Player,
    SpritesResources,
};

// ############## UI ####################
#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct PlayerSpeedBar;

#[derive(Component)]
pub struct PlayerArmorBar;

#[derive(Component)]
pub struct CurrentWaveUI;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct CurrentTimeUI;

#[derive(Component)]
pub struct ContainerBuffsUI;

#[derive(Component)]
pub struct BuffsUI {
    pub item_type: ItemTypeEnum,
}

#[derive(Component)]
pub struct WeaponUI;

// ############## BUTTONS ####################
#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct RestartGameButton;

// ############## SCREENS ####################
#[derive(Component)]
pub struct MenuOverlay;

#[derive(Component)]
pub struct GameOverOverlay;

#[derive(Component)]
pub struct GameWonOverlay;

const MAX_HEALTH_BAR: f32 = 100.0;
const HEALTH_BAR_SCALE: f32 = 0.2;

pub(crate) fn spawn_health_bar(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    health: f32,
    max_health: f32,
    translation: Vec3,
) -> Entity {
    let parent_shape =
        Mesh2dHandle(meshes.add(Rectangle::new(MAX_HEALTH_BAR * HEALTH_BAR_SCALE, 2.5)));
    let parent = MaterialMesh2dBundle {
        mesh: parent_shape,
        material: materials.add(Color::srgba(255., 255., 255., 0.1)),
        transform: Transform::from_xyz(translation.x, translation.y, translation.z),
        ..default()
    };

    let proportional = MAX_HEALTH_BAR * health / max_health;
    let width: f32 = proportional * HEALTH_BAR_SCALE;
    let child_shape = Mesh2dHandle(meshes.add(Rectangle::new(width, 2.5)));
    let child = MaterialMesh2dBundle {
        mesh: child_shape,
        material: materials.add(Color::srgb(0., 255., 0.)),
        transform: Transform::from_xyz(
            -(MAX_HEALTH_BAR * HEALTH_BAR_SCALE / 2. - width / 2.),
            0.0,
            UI_Z_INDEX,
        ),
        ..default()
    };

    commands
        .spawn((parent, PLAYER_LAYER, HealthBar))
        .with_children(|parent| {
            parent.spawn((child, PLAYER_LAYER));
        })
        .id()
}

fn speed_bar(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("{}", PLAYER_MOVE_SPEED),
                    TextStyle {
                        color: Color::Srgba(YELLOW),
                        ..text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                WINDOW_RESOLUTION.x_px / 2. - 30.,
                WINDOW_RESOLUTION.y_px / 2. - 30.,
                UI_Z_INDEX,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        PlayerSpeedBar,
        OVERLAY_LAYER,
    ));
}

fn armor_bar(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("{}", PLAYER_ARMOR),
                    TextStyle {
                        color: Color::Srgba(YELLOW),
                        ..text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                -200.,
                WINDOW_RESOLUTION.y_px / 2. - 30.,
                UI_Z_INDEX,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        PlayerArmorBar,
        OVERLAY_LAYER,
    ));
}

fn current_wave(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Current wave: 1",
                    TextStyle {
                        color: Color::Srgba(YELLOW),
                        ..text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                -WINDOW_RESOLUTION.x_px / 2. + 100.,
                WINDOW_RESOLUTION.y_px / 2. - 30.,
                UI_Z_INDEX,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        CurrentWaveUI,
        OVERLAY_LAYER,
    ));
}

fn spawn_score_points_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "0",
                    TextStyle {
                        color: Color::Srgba(YELLOW),
                        ..text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                WINDOW_RESOLUTION.y_px / 2. - 30.,
                UI_Z_INDEX,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        ScoreUI,
        OVERLAY_LAYER,
    ));
}

fn spawn_current_timer_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "01:00",
                    TextStyle {
                        color: Color::Srgba(YELLOW),
                        ..text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                200.0,
                WINDOW_RESOLUTION.y_px / 2. - 30.,
                UI_Z_INDEX,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        CurrentTimeUI,
        OVERLAY_LAYER,
    ));
}

pub(crate) fn spawn_container_buffs_ui(commands: &mut Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::RowReverse,
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                column_gap: Val::Px(2.),
                right: Val::Px(105.),
                top: Val::Px(5.),
                ..default()
            },
            ..default()
        },
        OVERLAY_LAYER,
        ContainerBuffsUI,
    ));
}

fn spawn_profile_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            OVERLAY_LAYER,
        ))
        .id();

    let child = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    margin: UiRect {
                        left: Val::ZERO,
                        right: Val::Px(5.),
                        top: Val::Px(5.),
                        bottom: Val::ZERO,
                    },
                    ..default()
                },
                ..default()
            },
            UiImage::new(asset_server.load("textures/UI/profile.png")),
            OVERLAY_LAYER,
        ))
        .id();

    commands.entity(parent).add_child(child);
}

pub(crate) fn spawn_weapon_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprite_source: &str,
) {
    let parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OVERLAY_LAYER,
            WeaponUI,
        ))
        .id();

    let child = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(5.)),
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
                ..default()
            },
            UiImage::new(asset_server.load(sprite_source.to_owned())),
            OVERLAY_LAYER,
        ))
        .id();

    commands.entity(parent).add_child(child);
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    speed_bar(&mut commands, &asset_server);
    armor_bar(&mut commands, &asset_server);
    current_wave(&mut commands, &asset_server);
    spawn_score_points_ui(&mut commands, &asset_server);
    spawn_current_timer_ui(&mut commands, &asset_server);
    spawn_profile_ui(&mut commands, &asset_server);
    spawn_container_buffs_ui(&mut commands);
    spawn_weapon_ui(&mut commands, &asset_server, DEFAULT_WEAPON_SPRITE_SOURCE);
}

pub fn menu_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = "MAIN MENU";
    let button_title = "Start game";
    let font_size = 100.;

    let one = commands
        .spawn(_build_custom_text_bundle(
            &asset_server,
            title,
            font_size,
            Color::WHITE,
        ))
        .id();

    let two = commands
        .spawn(_build_custom_button(StartGameButton))
        .with_children(|parent| {
            parent.spawn(_build_custom_text_bundle(
                &asset_server,
                button_title,
                40.,
                Color::srgb(0.9, 0.9, 0.9),
            ));
        })
        .id();

    _default_screen(commands, MenuOverlay, vec![one, two]);
}

pub fn game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_score: Res<CurrentScore>,
) {
    let title = "GAME OVER";
    let button_title = "Restart game";
    let font_size = 100.;

    let one = commands
        .spawn(_build_custom_text_bundle(
            &asset_server,
            title,
            font_size,
            Color::WHITE,
        ))
        .id();

    let two = commands
        .spawn(_build_custom_text_bundle(
            &asset_server,
            &format!("Final score: {:.6}", &current_score.0.to_string()),
            40.,
            Color::WHITE,
        ))
        .id();

    let three = commands
        .spawn(_build_custom_button(RestartGameButton))
        .with_children(|parent| {
            parent.spawn(_build_custom_text_bundle(
                &asset_server,
                button_title,
                40.,
                Color::srgb(0.9, 0.9, 0.9),
            ));
        })
        .id();

    _default_screen(commands, GameOverOverlay, vec![one, two, three]);
}

pub fn game_won_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_score: Res<CurrentScore>,
) {
    let title = "YOU WON";
    let button_title = "Play again";
    let font_size = 100.;

    let one = commands
        .spawn(_build_custom_text_bundle(
            &asset_server,
            title,
            font_size,
            Color::WHITE,
        ))
        .id();

    let two = commands
        .spawn(_build_custom_text_bundle(
            &asset_server,
            &format!("Final score: {:.6}", &current_score.0.to_string()),
            40.,
            Color::WHITE,
        ))
        .id();

    let three = commands
        .spawn(_build_custom_button(PlayAgainButton))
        .with_children(|parent| {
            parent.spawn(_build_custom_text_bundle(
                &asset_server,
                button_title,
                40.,
                Color::srgb(0.9, 0.9, 0.9),
            ));
        })
        .id();

    _default_screen(commands, GameWonOverlay, vec![one, two, three]);
}

fn _default_screen<T: Component>(
    mut commands: Commands,
    root_node_component: T,
    children_entities: Vec<Entity>,
) {
    let node_bundle = NodeBundle {
        style: Style {
            width: Val::Px(WINDOW_RESOLUTION.x_px),
            height: Val::Px(WINDOW_RESOLUTION.y_px),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        ..default()
    };

    commands
        .spawn((node_bundle, MENU_UI_LAYER, root_node_component))
        .push_children(&children_entities);
}

#[derive(Bundle)]
struct CustomTextBundle {
    bundle: TextBundle,
    layer: RenderLayers,
}

fn _build_custom_text_bundle(
    asset_server: &Res<AssetServer>,
    title: &str,
    font_size: f32,
    color: Color,
) -> CustomTextBundle {
    let text_style = _build_text_style(asset_server, font_size, color);

    CustomTextBundle {
        bundle: TextBundle::from_section(
            title,
            TextStyle {
                font: text_style.clone().font,
                font_size: text_style.font_size,
                color: text_style.color,
            },
        )
        .with_text_justify(JustifyText::Center),
        layer: MENU_UI_LAYER,
    }
}

#[derive(Bundle)]
struct CustomButtonBundle<T: Component> {
    bundle: ButtonBundle,
    layer: RenderLayers,
    component: T,
}

fn _build_custom_button<T: Component>(button: T) -> CustomButtonBundle<T> {
    CustomButtonBundle {
        bundle: ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(1.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            background_color: Color::BLACK.into(),
            ..default()
        },
        layer: MENU_UI_LAYER,
        component: button,
    }
}

fn _build_text_style(asset_server: &Res<AssetServer>, font_size: f32, color: Color) -> TextStyle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    TextStyle {
        font: font.clone(),
        font_size,
        color,
    }
}
