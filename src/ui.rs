use bevy::{
    color::palettes::css::YELLOW,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{prelude::*, CurrentScore, ItemTypeEnum, PlayerProfileUISet};

// ############## UI ####################
#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarUI;

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
    pub counter: u32,
}

#[derive(Component)]
pub struct WeaponUI;

#[derive(Component)]
pub struct PlayerProfileUI;

#[derive(Component)]
pub struct PlayerStatsUI;

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
const HEALTH_BAR_UI_SCALE: f32 = 1.5;

pub(crate) fn spawn_health_bar(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    health: f32,
    max_health: f32,
    translation: Vec3,
    layer: RenderLayers,
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
        .spawn((parent, layer.clone(), HealthBar))
        .with_children(|parent| {
            parent.spawn((child, layer));
        })
        .id()
}

pub(crate) fn spawn_health_ui_bar(
    commands: &mut Commands,
    player_profile_ui_query: Query<(Entity, &Children, &PlayerProfileUI)>,
    player_health_ui_query: Query<(Entity, &HealthBarUI)>,
    health: f32,
    max_health: f32,
) {
    let player_profile_ui = player_profile_ui_query.get_single();
    if player_profile_ui.is_err() {
        return;
    }
    let player_profile_ui = player_profile_ui.unwrap();
    let player_profile_ui_entity = player_profile_ui.0;
    let player_profile_ui_children = player_profile_ui.1;

    // Despawn current player health ui bars
    for &child in player_profile_ui_children {
        if let Ok((player_health_ui_entity, _)) = player_health_ui_query.get(child) {
            commands.entity(player_health_ui_entity).despawn_recursive();
            break;
        }
    }

    const HEIGHT: f32 = 15.;

    let parent = NodeBundle {
        style: Style {
            width: Val::Px(MAX_HEALTH_BAR * HEALTH_BAR_UI_SCALE),
            height: Val::Px(HEIGHT),
            top: Val::Px(50.),
            left: Val::Px(5.),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba(255., 255., 255., 0.1)),
        ..default()
    };

    let proportional = MAX_HEALTH_BAR * health / max_health;
    let width: f32 = proportional * HEALTH_BAR_UI_SCALE;

    let child = NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(HEIGHT),
            position_type: PositionType::Relative,
            top: Val::Px(0.),
            ..default()
        },
        background_color: BackgroundColor(Color::srgb(0., 255., 0.)),
        ..default()
    };

    let id = commands
        .spawn((parent, OVERLAY_LAYER, HealthBarUI))
        .with_children(|parent| {
            parent.spawn((child, OVERLAY_LAYER));
        })
        .id();
    commands
        .entity(player_profile_ui_entity)
        .push_children(&[id]);
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
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn(());
        });
}

fn spawn_profile_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(250.),
                    height: Val::Px(100.),
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
                ..default()
            },
            OVERLAY_LAYER,
            PlayerProfileUI,
        ))
        .id();

    let child = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(70.0),
                    height: Val::Px(70.0),
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::ZERO,
                        top: Val::Px(10.),
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

    commands.entity(parent).push_children(&[child]);

    commands.trigger(PlayerProfileUISet);
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

pub fn spawn_player_stats_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,

    current_health: f32,

    current_weapon_sprite: &str,
    current_weapon_damage_value: f32,

    current_armor_value: f32,
    current_speed_value: f32,
) {
    let parent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    width: Val::Px(400.),
                    height: Val::Px(400.),
                    position_type: PositionType::Absolute,
                    top: Val::Px(120.),
                    left: Val::Px(10.),
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceAround,
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::ZERO,
                        top: Val::ZERO,
                        bottom: Val::ZERO,
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
                ..default()
            },
            OVERLAY_LAYER,
            PlayerStatsUI,
        ))
        .id();

    let root_node = (
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                column_gap: Val::Px(30.),
                ..default()
            },
            ..default()
        },
        OVERLAY_LAYER,
    );

    let icon_node = |sprite: &str| {
        (
            NodeBundle {
                style: Style {
                    width: Val::Px(70.0),
                    height: Val::Px(70.0),
                    ..default()
                },
                ..default()
            },
            UiImage::new(asset_server.load(sprite.to_owned())),
        )
    };

    let text_node = |key: &str, value: &str, commands: &mut Commands| {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(70.0),
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::NoWrap,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("{key}: {value}"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        ..default()
                    },
                ));
            })
            .id()
    };

    let player_text_node = text_node("Health", &format!("{current_health}"), commands);
    let player = commands
        .spawn(root_node.clone())
        .with_children(|parent| {
            parent.spawn(icon_node("textures/UI/profile.png"));
        })
        .add_child(player_text_node)
        .id();

    let weapon_text_node = text_node(
        "Damage",
        &format!("{current_weapon_damage_value}"),
        commands,
    );
    let weapon = commands
        .spawn(root_node.clone())
        .with_children(|parent| {
            parent.spawn(icon_node(current_weapon_sprite));
        })
        .add_child(weapon_text_node)
        .id();

    let armor_text_node = text_node("Armor", &format!("{current_armor_value}"), commands);
    let armor = commands
        .spawn(root_node.clone())
        .with_children(|parent| {
            parent.spawn(icon_node("textures/Items/shield.png"));
        })
        .add_child(armor_text_node)
        .id();

    let speed_text_node = text_node("Speed", &format!("{current_speed_value}"), commands);
    let speed = commands
        .spawn(root_node)
        .with_children(|parent| {
            parent.spawn(icon_node("textures/Items/lightning.png"));
        })
        .add_child(speed_text_node)
        .id();

    commands
        .entity(parent)
        .push_children(&[player, weapon, armor, speed]);
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
