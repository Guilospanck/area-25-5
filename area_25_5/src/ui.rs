use bevy::{color::palettes::css::YELLOW, sprite::Anchor};

use crate::prelude::*;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerSpeedBar;

#[derive(Component)]
pub struct CurrentWaveUI;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct GameOverOverlay;

fn health_points_bar(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
                    format!("{}", PLAYER_HEALTH),
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
                10.0,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        PlayerHealthBar,
        GAME_LAYER,
    ));
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
                10.0,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        PlayerSpeedBar,
        GAME_LAYER,
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
                10.0,
            )),
            text_anchor: Anchor::TopCenter,
            ..default()
        },
        CurrentWaveUI,
        GAME_LAYER,
    ));
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    health_points_bar(&mut commands, &asset_server);
    speed_bar(&mut commands, &asset_server);
    current_wave(&mut commands, &asset_server);
}

pub fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };

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
        background_color: Color::srgb(255., 0., 0.).into(),
        ..default()
    };

    let button = (
        ButtonBundle {
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
        GAME_LAYER,
        StartButton,
    );

    commands
        .spawn((node_bundle, GAME_LAYER, GameOverOverlay))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "MAIN MENU",
                    TextStyle {
                        font: text_style.clone().font,
                        font_size: text_style.font_size,
                        color: text_style.color,
                    },
                )
                .with_text_justify(JustifyText::Center),
                GAME_LAYER,
            ));

            parent.spawn(button).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Start game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ),
                    GAME_LAYER,
                ));
            });
        });
}

pub fn game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };

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
        // /// This component is automatically managed by the UI layout system.
        // /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
        // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        background_color: Color::srgb(255., 0., 0.).into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
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
        GAME_LAYER,
        StartButton,
    );

    commands
        .spawn((node_bundle, GAME_LAYER, GameOverOverlay))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: text_style.clone().font,
                        font_size: text_style.font_size,
                        color: text_style.color,
                    },
                )
                .with_text_justify(JustifyText::Center),
                GAME_LAYER,
            ));

            parent.spawn(button).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ),
                    GAME_LAYER,
                ));
            });
        });
}
