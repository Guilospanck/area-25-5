use crate::{
    prelude::*, CurrentScore, CurrentTime, CurrentTimeChanged, CurrentWave, CurrentWaveChanged,
    PlayerHealthChanged, PlayerSpeedChanged, ScoreChanged,
};

#[derive(Component, Clone)]
pub struct CleanupWhenPlayerDies;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}

pub fn reset_initial_state(
    mut commands: Commands,
    mut current_wave: ResMut<CurrentWave>,
    mut current_time: ResMut<CurrentTime>,
    mut current_score: ResMut<CurrentScore>,
) {
    // Update UI
    current_wave.0 = 1u16;
    current_time.minutes = 0u16;
    current_time.seconds = 30u16;
    current_score.0 = 0.0;

    commands.trigger(CurrentWaveChanged);
    commands.trigger(CurrentTimeChanged);
    commands.trigger(PlayerSpeedChanged {
        speed: PLAYER_MOVE_SPEED,
    });
    commands.trigger(PlayerHealthChanged {
        health: PLAYER_HEALTH,
    });
    commands.trigger(ScoreChanged { score: 0.0 });
}
