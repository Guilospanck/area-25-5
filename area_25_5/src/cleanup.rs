use crate::{prelude::*, CurrentWave, CurrentWaveUI, PlayerHealthChanged, PlayerSpeedChanged};

#[derive(Component, Clone)]
pub struct CleanupWhenPlayerDies;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}

pub fn reset_initial_state(
    mut commands: Commands,
    mut current_wave_ui: Query<(&mut Text, &CurrentWaveUI)>,
    mut current_wave: ResMut<CurrentWave>,
) {
    // Update UI
    current_wave.0 = 1u32;
    if let Ok(mut text) = current_wave_ui.get_single_mut() {
        text.0.sections.first_mut().unwrap().value = format!("Current wave: {}", current_wave.0);
    }
    commands.trigger(PlayerSpeedChanged {
        speed: PLAYER_MOVE_SPEED,
    });
    commands.trigger(PlayerHealthChanged {
        health: PLAYER_HEALTH,
    });
}
