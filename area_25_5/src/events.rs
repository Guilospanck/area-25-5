use crate::{
    game_actions::shoot, player::Alien, prelude::*, spawn_enemy, spawn_item, spawn_weapon,
    ui::AlienHealthBar, AlienSpeedBar, CurrentWave, CurrentWaveUI, EnemyWaves, SpritesResources,
    WeaponWaves,
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
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let current_wave_enemy = enemy_waves
        .0
        .iter()
        .find(|enemy| enemy.level == current_wave.0 as usize);
    if current_wave_enemy.is_none() {
        println!("NO ENEMY MATCHING WAVE FOUND!!!");
        return;
    }
    let enemy_by_level = current_wave_enemy.unwrap();
    spawn_enemy(
        &mut commands,
        &asset_server,
        &sprites,
        &mut texture_atlas_layout,
        enemy_by_level,
    );

    let current_wave_weapon = weapon_waves
        .0
        .iter()
        .find(|weapon| weapon.level == current_wave.0 as usize);
    if current_wave_weapon.is_none() {
        println!("NO WEAPON MATCHING WAVE FOUND!!!");
        return;
    }
    let weapon_by_level = current_wave_weapon.unwrap();
    spawn_weapon(&mut commands, &mut meshes, &mut materials, weapon_by_level);

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
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    mut current_wave_ui: Query<&mut Text, With<CurrentWaveUI>>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // Update and cap current wave
    let new_wave = current_wave.0 + 1;
    if new_wave as usize > NUMBER_OF_WAVES {
        return;
    }
    current_wave.0 = new_wave;

    // Spawn more different enemies
    let current_wave_enemy = enemy_waves
        .0
        .iter()
        .find(|enemy| enemy.level == current_wave.0 as usize);
    if current_wave_enemy.is_none() {
        println!("NO ENEMY MATCHING WAVE FOUND!!!");
        return;
    }
    let enemy_by_level = current_wave_enemy.unwrap();
    spawn_enemy(
        &mut commands,
        &asset_server,
        &sprites,
        &mut texture_atlas_layout,
        enemy_by_level,
    );

    // Spawn more different weapons
    let current_wave_weapon = weapon_waves
        .0
        .iter()
        .find(|weapon| weapon.level == current_wave.0 as usize);
    if current_wave_weapon.is_none() {
        println!("NO WEAPON MATCHING WAVE FOUND!!!");
        return;
    }
    let weapon_by_level = current_wave_weapon.unwrap();
    spawn_weapon(&mut commands, &mut meshes, &mut materials, weapon_by_level);

    // Update UI
    if let Ok(mut text) = current_wave_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Current wave: {}", current_wave.0);
    }
}
