use crate::{
    game_actions::shoot, game_over, player::Player, prelude::*, setup_sprite, spawn_enemy,
    spawn_item, spawn_player, spawn_weapon, ui::PlayerHealthBar, Ammo, CurrentWave, CurrentWaveUI,
    Damage, Enemy, EnemyWaves, GameOverOverlay, Health, Item, PlayerSpeedBar, PlayerState, Speed,
    SpritesResources, Weapon, WeaponWaves,
};

#[derive(Event)]
pub struct ShootBullets {
    pub pos: Vec2,
}

#[derive(Event)]
pub struct PlayerHealthChanged {
    pub health: f32,
}

#[derive(Event)]
pub struct PlayerSpeedChanged {
    pub speed: f32,
}

#[derive(Event)]
pub struct PlayerSpawned;

#[derive(Event)]
pub struct AllEnemiesDied;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct RestartGame;

pub fn on_mouse_click(
    trigger: Trigger<ShootBullets>,
    commands: Commands,
    player_query: Query<(&Transform, &Children), With<Player>>,
    weapon_query: Query<&Weapon>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let event = trigger.event();
    let Vec2 { x, y } = event.pos;

    shoot(
        commands,
        x,
        y,
        player_query,
        weapon_query,
        asset_server,
        &sprites,
        &mut texture_atlas_layout,
    );
}

pub fn on_player_health_changed(
    trigger: Trigger<PlayerHealthChanged>,
    mut player_health_bar: Query<&mut Text, With<PlayerHealthBar>>,
) {
    let event = trigger.event();
    let health = event.health;

    if let Ok(mut text) = player_health_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = health.to_string();
    }
}

pub fn on_player_speed_changed(
    trigger: Trigger<PlayerSpeedChanged>,
    mut player_speed_bar: Query<&mut Text, With<PlayerSpeedBar>>,
) {
    let event = trigger.event();
    let speed = event.speed;

    if let Ok(mut text) = player_speed_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = speed.to_string();
    }
}

pub fn on_player_spawned(
    _trigger: Trigger<PlayerSpawned>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    current_wave: Res<CurrentWave>,
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<PlayerState>,
) {
    let current_wave_enemy = enemy_waves
        .0
        .iter()
        .find(|enemy| enemy.level == current_wave.0 as usize);
    if current_wave_enemy.is_none() {
        println!("NO ENEMY MATCHING WAVE FOUND!!!");
        return;
    }

    *player_state = PlayerState::Alive;

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

    spawn_weapon(
        &mut commands,
        weapon_by_level,
        texture_atlas_layout,
        &sprites,
        asset_server,
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
        commands.trigger(GameOver);
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

    spawn_weapon(
        &mut commands,
        weapon_by_level,
        texture_atlas_layout,
        &sprites,
        asset_server,
    );

    // Update UI
    if let Ok(mut text) = current_wave_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Current wave: {}", current_wave.0);
    }
}

pub fn on_game_over(
    _trigger: Trigger<GameOver>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_state: ResMut<PlayerState>,
) {
    if *player_state == PlayerState::Dead {
        return;
    }

    *player_state = PlayerState::Dead;

    game_over(commands, asset_server);
}

//FIXME: not updating correctly the resources and ui values and it is taking
//at least two clicks to really do something
pub fn on_restart_click(
    _trigger: Trigger<RestartGame>,
    mut commands: Commands,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    // Game components/resources
    mut current_wave: ResMut<CurrentWave>,
    player_entity: Query<Entity, With<Player>>,
    enemies_entity: Query<Entity, With<Enemy>>,
    weapon_entity: Query<Entity, (With<Weapon>, Without<Player>)>,
    game_over_overlay: Query<(Entity, &GameOverOverlay)>,

    // UI
    mut current_wave_ui: Query<
        (&mut Text, &CurrentWaveUI),
        (Without<CurrentWaveUI>, Without<PlayerSpeedBar>),
    >,
) {
    if let Ok(go_overlay) = game_over_overlay.get_single() {
        commands.entity(go_overlay.0).despawn_recursive();
    }

    // Cleanup
    for entity in player_entity.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in weapon_entity.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemies_entity.iter() {
        commands.entity(entity).despawn_recursive();
    }

    current_wave.0 = 1u32;

    // Update UI
    if let Ok(mut text) = current_wave_ui.get_single_mut() {
        text.0.sections.first_mut().unwrap().value = format!("Current wave: {}", current_wave.0);
    }

    spawn_player(&mut commands, texture_atlas_layout, asset_server, sprites);

    commands.trigger(PlayerSpeedChanged {
        speed: PLAYER_MOVE_SPEED,
    });
    commands.trigger(PlayerHealthChanged {
        health: PLAYER_HEALTH,
    });
}
