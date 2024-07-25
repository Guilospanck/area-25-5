use bevy::log::tracing_subscriber::fmt::format;

use crate::{
    game_actions::shoot, player::Alien, prelude::*, spawn_enemy, spawn_item, ui::AlienHealthBar,
    AlienSpeedBar, CurrentWave, CurrentWaveUI, Waves,
};

#[derive(Event)]
pub struct ShootBullets {
    pub pos: Vec2,
}

#[derive(Event)]
pub struct AlienHealthChanged {
    pub health: f32,
}

#[derive(Event)]
pub struct AlienSpeedChanged {
    pub speed: f32,
}

#[derive(Event)]
pub struct AlienSpawned;

#[derive(Event)]
pub struct AllEnemiesDied;

pub fn on_mouse_click(
    trigger: Trigger<ShootBullets>,
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    alien: Query<(&Transform, &Alien)>,
) {
    let event = trigger.event();
    let Vec2 { x, y } = event.pos;

    shoot(commands, materials, x, y, alien);
}

pub fn on_alien_health_changed(
    trigger: Trigger<AlienHealthChanged>,
    mut alien_health_bar: Query<&mut Text, With<AlienHealthBar>>,
) {
    let event = trigger.event();
    let health = event.health;

    if let Ok(mut text) = alien_health_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = health.to_string();
    }
}

pub fn on_alien_speed_changed(
    trigger: Trigger<AlienSpeedChanged>,
    mut alien_speed_bar: Query<&mut Text, With<AlienSpeedBar>>,
) {
    let event = trigger.event();
    let speed = event.speed;

    if let Ok(mut text) = alien_speed_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = speed.to_string();
    }
}

pub fn on_alien_spawned(
    _trigger: Trigger<AlienSpawned>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_wave: Res<CurrentWave>,
    waves: Res<Waves>,
) {
    let number_of_enemies_to_be_spawned: u32 = waves.info[current_wave.0 as usize - 1];
    spawn_enemy(
        &mut commands,
        &mut meshes,
        &mut materials,
        number_of_enemies_to_be_spawned,
    );
    spawn_item(
        commands,
        meshes,
        materials,
        crate::ItemStatsType::Speed,
        ITEM_SPEED_VALUE,
    );
}

pub fn on_all_enemies_died(
    _trigger: Trigger<AllEnemiesDied>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_wave: ResMut<CurrentWave>,
    waves: Res<Waves>,
    mut current_wave_ui: Query<&mut Text, With<CurrentWaveUI>>,
) {
    let new_wave = current_wave.0 + 1;
    if new_wave as usize > NUMBER_OF_WAVES {
        return;
    }

    current_wave.0 = new_wave;
    let number_of_enemies_to_be_spawned: u32 = waves.info[current_wave.0 as usize - 1];
    spawn_enemy(
        &mut commands,
        &mut meshes,
        &mut materials,
        number_of_enemies_to_be_spawned,
    );

    // Update UI
    if let Ok(mut text) = current_wave_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Current wave: {}", current_wave.0);
    }
}
