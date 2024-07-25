use bevy::{color::palettes::css::YELLOW, sprite::Anchor};

use crate::prelude::*;

#[derive(Component)]
pub struct AlienHealthBar;

fn health_points_bar(commands: &mut Commands, asset_server: Res<AssetServer>) {
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
                    "100000000",
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

pub fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    health_points_bar(commands, asset_server);
}
