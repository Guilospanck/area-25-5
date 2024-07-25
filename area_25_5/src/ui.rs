use bevy::{color::palettes::css::YELLOW, sprite::Anchor};

use crate::prelude::*;

#[derive(Component)]
pub struct AlienHealthBar;

#[derive(Component)]
pub struct AlienSpeedBar;

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
                    format!("{}", ALIEN_HEALTH),
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
        AlienHealthBar,
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
                    format!("{}", ALIEN_MOVE_SPEED),
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
        AlienSpeedBar,
        GAME_LAYER,
    ));
}
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    health_points_bar(&mut commands, &asset_server);
    speed_bar(&mut commands, &asset_server);
}
