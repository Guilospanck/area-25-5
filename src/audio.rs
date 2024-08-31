use crate::{prelude::*, PlayerHitAudioTimeout};

pub fn hit_enemy_audio(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    return;
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/breakout_collision.ogg"),
        ..default()
    });
}

pub fn hit_item_audio(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    return;
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/breakout_collision.ogg"),
        ..default()
    });
}

pub fn hit_weapon_audio(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    return;
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/breakout_collision.ogg"),
        ..default()
    });
}

pub fn player_hit_audio(
    asset_server: &Res<AssetServer>,
    time: &Res<Time>,
    commands: &mut Commands,
    audio_timeout: &mut ResMut<PlayerHitAudioTimeout>,
) {
    return;
    audio_timeout.0.tick(time.delta());

    if audio_timeout.0.finished() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/breakout_collision.ogg"),
            ..default()
        });
    }
}
