use crate::{
    game_actions::shoot, player::Player, prelude::*, spawn_enemy, spawn_health_bar, spawn_item,
    spawn_weapon, ui::HealthBar, CurrentScore, CurrentWave, CurrentWaveUI, Enemy, EnemyWaves,
    GameState, ItemWaves, PlayerSpeedBar, ScoreUI, SpritesResources, Weapon, WeaponWaves,
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
pub struct EnemyHealthChanged {
    pub health: f32,
    pub entity: Entity,
}

#[derive(Event)]
pub struct AllEnemiesDied;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct RestartGame;

#[derive(Event)]
pub struct ScoreChanged {
    pub score: f32,
}

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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<(Entity, &Children), With<Player>>,
    player_health_bar_query: Query<Entity, With<HealthBar>>,
) {
    let event = trigger.event();
    let health = event.health;

    let player = player_query.get_single();
    if player.is_err() {
        return;
    }
    let player = player.unwrap();
    let player_entity = player.0;
    let health_bar_translation = Vec3::new(2.0, 12.0, 0.0);

    for &child in player.1.iter() {
        if let Ok(health_bar_entity) = player_health_bar_query.get(child) {
            commands.entity(health_bar_entity).despawn_recursive();
            let health_bar = spawn_health_bar(
                &mut commands,
                &mut meshes,
                &mut materials,
                health,
                PLAYER_HEALTH,
                health_bar_translation,
            );
            commands
                .entity(player_entity)
                .remove_children(&[health_bar_entity]);
            commands.entity(player_entity).add_child(health_bar);
            break;
        }
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
    current_wave: Res<CurrentWave>,
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    item_waves: Res<ItemWaves>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    player_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if *player_state.get() != GameState::Alive {
        next_state.set(GameState::Alive);
    }

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
        &mut meshes,
        &mut materials,
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
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
    );

    let current_wave_item = item_waves
        .0
        .iter()
        .find(|item| item.level == current_wave.0 as usize);
    if current_wave_item.is_none() {
        println!("NO ITEM MATCHING WAVE FOUND!!!");
        return;
    }
    let item_by_level = current_wave_item.unwrap();
    spawn_item(
        &mut commands,
        item_by_level,
        texture_atlas_layout,
        &sprites,
        asset_server,
    );
}

pub fn on_enemy_health_changed(
    trigger: Trigger<EnemyHealthChanged>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<(Entity, &Children), With<Enemy>>,
    enemy_health_bar_query: Query<Entity, With<HealthBar>>,
) {
    let event = trigger.event();
    let health = event.health;
    let enemy_entity = event.entity;

    let health_bar_translation = Vec3::new(2.0, 15.0, 0.0);
    for (entity, children) in enemy_query.iter() {
        if entity == enemy_entity {
            for &child in children.iter() {
                if let Ok(health_bar_entity) = enemy_health_bar_query.get(child) {
                    commands.entity(health_bar_entity).despawn_recursive();
                    let health_bar = spawn_health_bar(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        health,
                        ENEMY_HEALTH,
                        health_bar_translation,
                    );
                    commands
                        .entity(enemy_entity)
                        .remove_children(&[health_bar_entity]);
                    commands.entity(enemy_entity).add_child(health_bar);
                    break;
                }
            }
            break;
        }
    }
}

pub fn on_all_enemies_died(
    _trigger: Trigger<AllEnemiesDied>,
    mut commands: Commands,
    mut current_wave: ResMut<CurrentWave>,
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    item_waves: Res<ItemWaves>,
    mut current_wave_ui: Query<&mut Text, With<CurrentWaveUI>>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    player_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Update and cap current wave
    let new_wave = current_wave.0 + 1;
    if new_wave as usize > NUMBER_OF_WAVES {
        if *player_state.get() != GameState::Won {
            next_state.set(GameState::Won);
        }
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
        &mut meshes,
        &mut materials,
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
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
    );

    let current_wave_item = item_waves
        .0
        .iter()
        .find(|item| item.level == current_wave.0 as usize);
    if current_wave_item.is_none() {
        println!("NO ITEM MATCHING WAVE FOUND!!!");
        return;
    }
    let item_by_level = current_wave_item.unwrap();
    spawn_item(
        &mut commands,
        item_by_level,
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
    player_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *player_state.get() == GameState::Dead {
        return;
    }
    next_state.set(GameState::Dead);
}

pub fn on_restart_click(
    _trigger: Trigger<RestartGame>,
    player_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *player_state.get() == GameState::Alive {
        return;
    }
    next_state.set(GameState::Alive);
}

pub fn on_score_changed(
    trigger: Trigger<ScoreChanged>,
    mut current_score: ResMut<CurrentScore>,
    mut score_ui: Query<&mut Text, With<ScoreUI>>,
) {
    let event = trigger.event();
    let score = event.score;

    current_score.0 += score;

    if let Ok(mut text) = score_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = current_score.0.to_string();
    }
}
