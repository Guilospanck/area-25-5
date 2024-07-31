use bevy::{color::palettes::css::YELLOW, sprite::Anchor};

use crate::{prelude::*};

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerSpeedBar;

#[derive(Component)]
pub struct CurrentWaveUI;

#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct RestartGameButton;

#[derive(Component)]
pub struct MenuOverlay;

#[derive(Component)]
pub struct GameOverOverlay;

#[derive(Component)]
pub struct GameWonOverlay;

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

pub fn menu_screen(commands: Commands, asset_server: Res<AssetServer>) {
    let title = "MAIN MENU";
    let button_title = "Start game";
    _default_screen(
        commands,
        asset_server,
        title,
        button_title,
        StartGameButton,
        MenuOverlay,
    );
}

pub fn game_over_screen(commands: Commands, asset_server: Res<AssetServer>) {
    let title = "GAME OVER";
    let button_title = "Restart game";
    _default_screen(
        commands,
        asset_server,
        title,
        button_title,
        RestartGameButton,
        GameOverOverlay,
    );
}

pub fn game_won_screen(commands: Commands, asset_server: Res<AssetServer>) {
    let title = "YOU WON";
    let button_title = "Play again";
    _default_screen(
        commands,
        asset_server,
        title,
        button_title,
        PlayAgainButton,
        GameWonOverlay,
    );
}

fn _default_screen<T: Component, R: Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    title: &str,
    button_title: &str,
    button_component: T,
    root_node_component: R,
) {
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
        button_component,
    );

    commands
        .spawn((node_bundle, GAME_LAYER, root_node_component))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    title,
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
                        button_title,
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
