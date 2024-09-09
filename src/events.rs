use chrono::Utc;

use bevy::{sprite::Mesh2dHandle, window::WindowResized};

use crate::{
    audio::hit_weapon_audio,
    cleanup_system, equip_player_with_power,
    game_actions::shoot_at_enemies,
    in_between_waves_pause_screen,
    player::Player,
    prelude::*,
    render_background_texture, reset_initial_state, setup_player, setup_ui, spawn_boss,
    spawn_enemy, spawn_health_bar, spawn_health_ui_bar, spawn_item, spawn_mana_ui_bar,
    spawn_power_ui, spawn_profile_ui, spawn_weapon, spawn_weapon_ui,
    ui::HealthBar,
    util::{
        get_boss_type_based_on_game_level, get_item_sprite_based_on_item_type,
        get_key_code_based_on_power_type, get_power_sprite_based_on_power_type, get_random_chance,
        get_weapon_sprite_based_on_weapon_type,
    },
    AmmoBundle, Armor, BaseCamera, Buff, BuffGroup, BuffsUI, CircleOfDeath, CleanupWhenPlayerDies,
    ContainerBuffsUI, CurrentBoss, CurrentGameLevel, CurrentGameLevelUI, CurrentScore, CurrentTime,
    CurrentTimeUI, CurrentWave, CurrentWaveUI, Damage, EnemiesLeftUI, Enemy, EnemyWaves,
    GameOverOverlay, GameState, GameWonOverlay, Health, HealthBarUI, Item, ItemTypeEnum, ItemWaves,
    Mana, ManaBarUI, MenuOverlay, PlayerProfileUI, PlayerProfileUIBarsRootNode, Power,
    PowerLevelUI, PowerLevels, PowerSpriteUI, PowerUI, PowerUIRootNode, ScoreUI, Speed,
    SpritesResources, TileBackground, Weapon, WeaponBundle, WeaponUI, WeaponWaves,
    WindowResolutionResource,
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
pub struct PlayerManaChanged {
    pub mana: f32,
}

#[derive(Event)]
pub struct PlayerSpawned {
    pub player_entity_id: Entity,
}

#[derive(Event)]
pub struct EnemyHealthChanged {
    pub health: f32,
    pub entity: Entity,
    pub max_health: f32,
}

#[derive(Event)]
pub struct AllEnemiesDied;

#[derive(Event)]
pub struct CurrentWaveChanged(pub u16);

#[derive(Event)]
pub struct SpawnEntitiesForNewWave;

#[derive(Event)]
pub struct UpdateTimeUI;

#[derive(Event)]
pub struct UpdateAliveEnemiesUI;

#[derive(Event)]
pub struct SetupNewTime;

#[derive(Event)]
pub struct PlayerProfileUISet;

#[derive(Event)]
pub struct BuffAdded {
    pub item_type: ItemTypeEnum,
}

#[derive(Event)]
pub struct BuffUIRemove {
    pub item_type: ItemTypeEnum,
}

#[derive(Event)]
pub struct BuffUIAdd {
    pub item_type: ItemTypeEnum,
}

#[derive(Event)]
pub struct WeaponFound {
    pub weapon_entity: Entity,
    pub weapon: Weapon,
    pub weapon_damage: Damage,
    pub player_entity: Entity,
    pub player_weapon_entity: Entity,
    pub player_ammo_entity: Entity,
}

#[derive(Event)]
pub struct PowerFound;

// Used for instances of Power (like CircleOfDeath)
// that need to be despawned once the entity (like the Annulus)
// is despawned
#[derive(Event)]
pub struct DespawnPower(pub PowerTypeEnum);

#[derive(Event)]
pub struct OnUpdatePowerUI {
    power_type: PowerTypeEnum,
    keycode: KeyCode,
}

#[derive(Event)]
pub struct MaybeSpawnHealthPack;

#[derive(Event)]
pub struct MaybeSpawnManaPack;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct RestartGame;

#[derive(Event)]
pub struct ScoreChanged {
    pub score: f32,
}

#[derive(Event)]
pub struct ChangeBackgroundTexture;

#[derive(Event)]
pub struct CurrentGameLevelChanged(pub u16);

pub fn on_mouse_click(
    trigger: Trigger<ShootBullets>,
    commands: Commands,
    player_query: Query<(Entity, &Transform, &Children), With<Player>>,
    weapon_query: Query<(&Weapon, &Damage)>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,
) {
    let event = trigger.event();
    let Vec2 { x, y } = event.pos;

    shoot_at_enemies(
        commands,
        x,
        y,
        player_query,
        weapon_query,
        asset_server,
        &sprites,
        &mut texture_atlas_layout,
        base_camera,
    );
}

fn modify_above_player_health(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    player_query: Query<(Entity, &Children), With<Player>>,
    player_health_bar_query: Query<Entity, With<HealthBar>>,

    health: f32,
) {
    let health_bar_translation = Vec3::new(2.0, 12.0, 0.0);

    let Ok((player_entity, player_children)) = player_query.get_single() else {
        return;
    };

    for &child in player_children.iter() {
        let Ok(player_health_bar_entity) = player_health_bar_query.get(child) else {
            continue;
        };

        commands
            .entity(player_health_bar_entity)
            .despawn_recursive();

        let health_bar = spawn_health_bar(
            commands,
            &mut meshes,
            &mut materials,
            health,
            PLAYER_HEALTH,
            health_bar_translation,
            PLAYER_LAYER,
        );

        commands
            .entity(player_entity)
            .remove_children(&[player_health_bar_entity]);
        commands.entity(player_entity).add_child(health_bar);

        break;
    }
}

fn modify_player_profile_ui_health(
    commands: &mut Commands,

    player_profile_ui_query: Query<(Entity, &Children, &PlayerProfileUI)>,
    mut player_bar_ui_root_node_query: Query<(Entity, &Children, &PlayerProfileUIBarsRootNode)>,
    player_health_ui_query: Query<(Entity, &HealthBarUI)>,

    health: f32,
) {
    let Ok((_, player_profile_children, _)) = player_profile_ui_query.get_single() else {
        return;
    };

    for &child in player_profile_children.iter() {
        let Ok((_, root_node_bar_children, _)) = player_bar_ui_root_node_query.get(child) else {
            continue;
        };

        for &root_node_child in root_node_bar_children.iter() {
            let Ok((health_bar_entity, _)) = player_health_ui_query.get(root_node_child) else {
                continue;
            };

            commands.entity(health_bar_entity).despawn_recursive();

            // Player profile health bar UI
            spawn_health_ui_bar(
                commands,
                &player_profile_ui_query,
                &mut player_bar_ui_root_node_query,
                &player_health_ui_query,
                health,
                PLAYER_HEALTH,
            );

            break;
        }
    }
}

pub fn on_player_health_changed(
    trigger: Trigger<PlayerHealthChanged>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,

    // Just above player bar
    player_query: Query<(Entity, &Children), With<Player>>,
    player_health_bar_query: Query<Entity, With<HealthBar>>,

    // Top-left UI
    player_profile_ui_query: Query<(Entity, &Children, &PlayerProfileUI)>,
    player_bar_ui_root_node_query: Query<(Entity, &Children, &PlayerProfileUIBarsRootNode)>,
    player_health_ui_query: Query<(Entity, &HealthBarUI)>,
) {
    let event = trigger.event();
    let health = event.health;

    modify_above_player_health(
        &mut commands,
        meshes,
        materials,
        player_query,
        player_health_bar_query,
        health,
    );

    modify_player_profile_ui_health(
        &mut commands,
        player_profile_ui_query,
        player_bar_ui_root_node_query,
        player_health_ui_query,
        health,
    );
}

pub fn on_player_mana_changed(
    trigger: Trigger<PlayerManaChanged>,
    mut commands: Commands,

    player_profile_ui_query: Query<(Entity, &Children, &PlayerProfileUI)>,
    // This is the container of the mana and health bars
    mut player_bar_ui_root_node_query: Query<(Entity, &Children, &PlayerProfileUIBarsRootNode)>,
    player_mana_ui_query: Query<(Entity, &ManaBarUI)>,
) {
    let Ok((_, player_profile_children, _)) = player_profile_ui_query.get_single() else {
        return;
    };

    let event = trigger.event();
    let mana = event.mana;

    for &child in player_profile_children.iter() {
        let Ok((_, root_node_bar_children, _)) = player_bar_ui_root_node_query.get(child) else {
            continue;
        };

        for &root_node_child in root_node_bar_children.iter() {
            let Ok((mana_bar_entity, _)) = player_mana_ui_query.get(root_node_child) else {
                continue;
            };

            commands.entity(mana_bar_entity).despawn_recursive();

            // Player profile mana bar UI
            spawn_mana_ui_bar(
                &mut commands,
                &player_profile_ui_query,
                &mut player_bar_ui_root_node_query,
                &player_mana_ui_query,
                mana,
                PLAYER_MANA,
            );

            break;
        }
    }
}

pub fn on_player_spawned(
    trigger: Trigger<PlayerSpawned>,
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
    let event = trigger.event();
    let player_entity_id = event.player_entity_id;

    if *player_state.get() != GameState::Alive {
        next_state.set(GameState::Alive);
    }

    let current_wave_enemy = enemy_waves
        .0
        .iter()
        .find(|enemy| enemy.wave == current_wave.0 as usize);
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
        .find(|weapon| weapon.wave == current_wave.0 as usize);
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
        player_entity_id,
        crate::util::EquippedTypeEnum::Player,
    );

    let current_wave_item = item_waves
        .0
        .iter()
        .find(|item| item.wave == current_wave.0 as usize);
    if current_wave_item.is_none() {
        println!("NO ITEM MATCHING WAVE FOUND!!!");
        return;
    }
    let item_by_wave = current_wave_item.unwrap();
    let current_game_level = 1;
    spawn_item(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
        item_by_wave.item.item_type.clone(),
        item_by_wave.quantity,
        current_game_level,
    );

    // UI stuff
    spawn_weapon_ui(&mut commands, &asset_server, DEFAULT_WEAPON_SPRITE_SOURCE);
    spawn_profile_ui(&mut commands, &asset_server);
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
    let max_health = event.max_health;

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
                        max_health,
                        health_bar_translation,
                        BASE_LAYER,
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

    mut current_boss: ResMut<CurrentBoss>,
    mut next_state: ResMut<NextState<GameState>>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    current_wave: Res<CurrentWave>,
    current_time: Res<CurrentTime>,
    current_game_level: Res<CurrentGameLevel>,
    player_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
) {
    // TODO: change this to be inside the event handler
    // Add multiplier to score based on the time left
    let mut seconds = current_time.seconds;
    seconds += current_time.minutes * 60;
    let score = SCORE_MULTIPLIER * seconds as f32;
    commands.trigger(ScoreChanged { score });

    // Update current wave
    let new_wave = current_wave.0 + 1;

    if new_wave as usize <= NUMBER_OF_WAVES {
        // update current wave
        commands.trigger(CurrentWaveChanged(new_wave));

        // Setup new time
        commands.trigger(SetupNewTime);
        return;
    }

    // If current boss exists, it means that we just defeated it.
    if current_boss.0.is_some() {
        // check level
        let new_level = current_game_level.0 + 1;

        // We spawned all possible levels and waves
        if new_level as usize > NUMBER_OF_LEVELS {
            if *player_state.get() != GameState::Won {
                next_state.set(GameState::Won);
            }
            return;
        }

        // increase level
        commands.trigger(CurrentGameLevelChanged(new_level));

        // reset current boss
        current_boss.0 = None;

        // update texture background
        commands.trigger(ChangeBackgroundTexture);

        // reset current wave
        let new_wave = 1;
        commands.trigger(CurrentWaveChanged(new_wave));

        // Setup new time
        commands.trigger(SetupNewTime);

        return;
    }

    let boss = get_boss_type_based_on_game_level(current_game_level.0);
    let health_bar_translation = Vec3::new(2.0, 15.0, 0.0);
    let quantity = 1;

    spawn_boss(
        &mut commands,
        &asset_server,
        &sprites,
        &mut texture_atlas_layout,
        &mut meshes,
        &mut materials,
        boss.health,
        boss.base_damage,
        boss.scale,
        health_bar_translation,
        quantity,
        boss.class,
    );

    // update current boss
    current_boss.0 = Some(current_game_level.0);

    // Update alive enemies UI
    commands.trigger(UpdateAliveEnemiesUI);
}

pub fn spawn_entities_for_new_wave(
    _trigger: Trigger<SpawnEntitiesForNewWave>,
    mut commands: Commands,

    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    current_wave: Res<CurrentWave>,
    current_game_level: Res<CurrentGameLevel>,
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    item_waves: Res<ItemWaves>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    weapons: Query<(Entity, Option<&Parent>, &Damage), With<Weapon>>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Get player's current weapon damage
    let mut optional_players_current_damage: Option<&Damage> = None;
    for weapon in weapons.iter() {
        if weapon.1.is_none() {
            commands.entity(weapon.0).despawn();
        } else {
            optional_players_current_damage = Some(weapon.2);
        }
    }
    let Some(player_current_damage) = optional_players_current_damage else {
        println!("Could not find player's current weapon with its damage");
        return;
    };

    // Spawn more different enemies
    let current_wave_enemy = enemy_waves
        .0
        .iter()
        .find(|enemy| enemy.wave == current_wave.0 as usize)
        .cloned();
    if current_wave_enemy.is_none() {
        println!("NO ENEMY MATCHING WAVE FOUND!!!");
        return;
    }
    let mut enemy_by_level = current_wave_enemy.unwrap();

    // increase base damage of all wave enemies based on current level
    let base_damage_multiplier =
        ENEMY_BASE_DAMAGE_MULTIPLIER_BASED_ON_LEVEL * current_game_level.0 as f32 + 1.0;
    enemy_by_level.enemy.base_damage *= base_damage_multiplier;

    spawn_enemy(
        &mut commands,
        &asset_server,
        &sprites,
        &mut texture_atlas_layout,
        &enemy_by_level,
        &mut meshes,
        &mut materials,
    );

    // Update alive enemies UI
    commands.trigger(UpdateAliveEnemiesUI);

    // Spawn more different weapons
    let current_wave_weapon = weapon_waves
        .0
        .iter()
        .find(|weapon| weapon.wave == current_wave.0 as usize)
        .cloned();
    if current_wave_weapon.is_none() {
        println!("NO WEAPON MATCHING WAVE FOUND!!!");
        return;
    }
    let mut weapon_by_level = current_wave_weapon.unwrap();

    // increase base damage of all wave weapons based on current level
    let base_damage_multiplier =
        WEAPON_BASE_DAMAGE_MULTIPLIER_BASED_ON_LEVEL * current_game_level.0 as f32 + 1.0;

    weapon_by_level.weapon.base_damage *= base_damage_multiplier;
    // Cap weapon_base_damage to minimum of the current weapon damage
    // carried by the player + the base_damage_multiplier
    if weapon_by_level.weapon.base_damage < player_current_damage.0 {
        weapon_by_level.weapon.base_damage = player_current_damage.0 * base_damage_multiplier;
    }

    spawn_weapon(
        &mut commands,
        &weapon_by_level,
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
        player_entity,
        crate::util::EquippedTypeEnum::Player,
    );

    let current_wave_item = item_waves
        .0
        .iter()
        .find(|item| item.wave == current_wave.0 as usize);
    if current_wave_item.is_none() {
        println!("NO ITEM MATCHING WAVE FOUND!!!");
        return;
    }
    let item_by_wave = current_wave_item.unwrap();
    spawn_item(
        &mut commands,
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
        item_by_wave.item.item_type.clone(),
        item_by_wave.quantity,
        current_game_level.0,
    );
}

pub fn on_current_wave_changed(
    trigger: Trigger<CurrentWaveChanged>,
    mut commands: Commands,

    mut current_wave: ResMut<CurrentWave>,

    mut current_wave_ui: Query<
        (&mut Text, &CurrentWaveUI),
        (Without<CurrentTimeUI>, Without<CurrentGameLevelUI>),
    >,
    items: Query<(Entity, &Item), With<Item>>,

    mut next_state: ResMut<NextState<GameState>>,
) {
    // Despawn items and weapons that were spawned on the map
    for (item_entity, item) in items.iter() {
        match item.item_type {
            ItemTypeEnum::Health(_) => continue,
            _ => commands.entity(item_entity).despawn(),
        }
    }

    // update Current wave
    let event = trigger.event();
    let new_wave = event.0;
    current_wave.0 = new_wave;

    // Update current wave UI
    if let Ok((mut text, _)) = current_wave_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Wave #{}", current_wave.0);
    }

    // Only show the in-between pause screen if it is from the second
    // wave onwards, as for the first wave we have the `starting game`
    // and the `next level` parts
    if new_wave != 1 {
        // spawn the in-between wave pause screen
        next_state.set(GameState::InBetweenWaves);
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
    if *player_state.get() == GameState::Start {
        return;
    }

    next_state.set(GameState::Start);
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
        text.sections.first_mut().unwrap().value = format!("{:.6}", current_score.0.to_string());
    }
}

pub fn tick_timer(mut commands: Commands, mut current_time: ResMut<CurrentTime>) {
    // Update current time
    let mut minutes = current_time.minutes;
    let mut seconds = current_time.seconds;

    if seconds == 0 && minutes == 0 {
        commands.trigger(GameOver);
        return;
    }

    if seconds == 0 && minutes > 0 {
        minutes -= 1;
        seconds = 59;
    } else if seconds > 0 {
        seconds = seconds.saturating_sub(1);
    }

    *current_time = CurrentTime { minutes, seconds };
    commands.trigger(UpdateTimeUI);
}

pub fn remove_outdated_buffs(
    mut commands: Commands,
    mut player: Query<(&mut Speed, &mut Armor, &Children), With<Player>>,
    player_buff_group_query: Query<(Entity, &BuffGroup)>,
) {
    let Ok((_, mut player_armor, player_children)) = player.get_single_mut() else {
        return;
    };

    let should_be_despawned = |buff_group: BuffGroup,
                               player_armor: &mut Armor,
                               commands: &mut Commands,
                               buff_ui_despawned: Option<ItemTypeEnum>|
     -> bool {
        match &buff_group.item {
            crate::ItemTypeEnum::Speed(_)
            | crate::ItemTypeEnum::Armor(_)
            | crate::ItemTypeEnum::Health(_)
            | crate::ItemTypeEnum::Mana(_) => false,
            crate::ItemTypeEnum::Shield(shield) => {
                if shield.duration_seconds.is_none() {
                    return false;
                }

                let start_time = buff_group.start_time;
                let end_time = Utc::now().time();
                let diff = end_time - start_time;

                let has_passed =
                    diff.num_seconds() > shield.duration_seconds.unwrap().try_into().unwrap();

                if has_passed {
                    // update player armor
                    // TODO: check for shield type (magical vs physical)
                    if shield.defensive > 0. {
                        player_armor.0 -= shield.defensive * NUMBER_OF_BUFF_ITEMS as f32;
                    }
                    if buff_ui_despawned.is_none() {
                        commands.trigger(BuffUIRemove {
                            item_type: buff_group.item.clone(),
                        });
                    }
                }

                has_passed
            }
        }
    };

    let mut buff_group_ui_despawned = None;
    for &child in player_children {
        let Ok((player_buff_group_entity, player_buff_group)) = player_buff_group_query.get(child)
        else {
            continue;
        };

        if should_be_despawned(
            player_buff_group.clone(),
            &mut player_armor,
            &mut commands,
            buff_group_ui_despawned.clone(),
        ) {
            if buff_group_ui_despawned.is_none() {
                buff_group_ui_despawned = Some(player_buff_group.item.clone());
            }

            commands
                .entity(player_buff_group_entity)
                .despawn_recursive();
        }
    }
}

pub fn refill_health(mut commands: Commands, mut player: Query<&mut Health, With<Player>>) {
    let Ok(mut player_health) = player.get_single_mut() else {
        return;
    };

    if player_health.0 < PLAYER_HEALTH {
        player_health.0 += 1.0;
    }

    commands.trigger(PlayerHealthChanged {
        health: player_health.0,
    });
}

pub fn refill_mana(mut commands: Commands, mut player: Query<&mut Mana, With<Player>>) {
    let Ok(mut player_mana) = player.get_single_mut() else {
        return;
    };

    if player_mana.0 < PLAYER_MANA {
        player_mana.0 += 1.0;
    }

    commands.trigger(PlayerManaChanged {
        mana: player_mana.0,
    });
}

pub fn expand_circle_of_death(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut circle_of_death: Query<
        (Entity, &mut Mesh2dHandle, &mut CircleOfDeath),
        With<CircleOfDeath>,
    >,
) {
    for (circle_entity, mut mesh2d_handle, mut circle) in circle_of_death.iter_mut() {
        let new_outer_radius = circle.outer_circle_radius * 0.2 + circle.outer_circle_radius;
        let new_inner_radius = new_outer_radius - 10.0;

        if new_inner_radius > BACKGROUND_TEXTURE_SCALE * BACKGROUND_TEXTURE_RESOLUTION.x_px {
            commands.entity(circle_entity).despawn();
            commands.trigger(DespawnPower(PowerTypeEnum::CircleOfDeath));
            continue;
        }

        let new_mesh = meshes.add(Annulus::new(new_inner_radius, new_outer_radius));

        circle.inner_circle_radius = new_inner_radius;
        circle.outer_circle_radius = new_outer_radius;

        *mesh2d_handle = Mesh2dHandle(new_mesh);
    }
}

pub fn despawn_powers(
    trigger: Trigger<DespawnPower>,

    commands: Commands,
    player_query: Query<(&Children, &Player)>,
    player_powers_query: Query<(Entity, &Power)>,

    powers_query: Query<(Entity, &Power), With<Power>>,
) {
    let event = trigger.event();
    let power_type = event.0.clone();

    if let PowerTypeEnum::CircleOfDeath = power_type {
        despawn_circle_of_death_power(commands, player_query, player_powers_query, powers_query)
    };
}

fn despawn_circle_of_death_power(
    mut commands: Commands,
    player_query: Query<(&Children, &Player)>,
    player_powers_query: Query<(Entity, &Power)>,

    powers_query: Query<(Entity, &Power), With<Power>>,
) {
    let Ok((player_children, _)) = player_query.get_single() else {
        return;
    };

    let mut current_player_powers_entity: Vec<Entity> = vec![];
    for &child in player_children {
        if let Ok(player_powers) = player_powers_query.get(child) {
            current_player_powers_entity.push(player_powers.0);
        }
    }

    let circle_of_death_key_code = get_key_code_based_on_power_type(PowerTypeEnum::CircleOfDeath);

    for (power_entity, power) in powers_query.iter() {
        // if current power is from player, do not despawn it
        if current_player_powers_entity.contains(&power_entity) {
            continue;
        }

        if power.trigger_key == circle_of_death_key_code {
            commands.entity(power_entity).despawn();
        }
    }
}

const NUMBER_OF_POSITIONS: usize = 360; // 2pi

pub fn animate_player_buffs(
    mut player_query: Query<&Children, With<Player>>,
    player_buff_group_query: Query<(&Children, &BuffGroup)>,
    mut player_buff_query: Query<(&mut Transform, &Buff)>,
    time: Res<Time>,
) {
    let elapsed_seconds = time.elapsed_seconds();
    let degrees = elapsed_seconds % NUMBER_OF_POSITIONS as f32;

    let Ok(player_children) = player_query.get_single_mut() else {
        return;
    };

    for &child in player_children.iter() {
        let Ok((player_buff_group_children, _)) = player_buff_group_query.get(child) else {
            continue;
        };

        for (idx, &player_buff_group_child) in player_buff_group_children.iter().enumerate() {
            if player_buff_query.get_mut(player_buff_group_child).is_err() {
                continue;
            }

            let (mut player_buff_transform, player_buff) =
                player_buff_query.get_mut(player_buff_group_child).unwrap();

            match player_buff.item {
                ItemTypeEnum::Shield(_) => {
                    let radians = DEGREES_TO_RADIANS
                        * degrees
                        * (idx + 1) as f32
                        * (NUMBER_OF_POSITIONS / NUMBER_OF_BUFF_ITEMS) as f32;

                    let (mut y, mut x) = f32::sin_cos(radians);
                    y *= RADIUS_FROM_PLAYER;
                    x *= RADIUS_FROM_PLAYER;

                    player_buff_transform.translation = Vec3::new(x, y, 1.);
                }
                _ => continue,
            }
        }
    }
}

pub fn setup_new_time(
    _trigger: Trigger<SetupNewTime>,
    mut commands: Commands,
    mut current_time: ResMut<CurrentTime>,
    current_wave: Res<CurrentWave>,
) {
    // Update current time
    let mut seconds: u16 = current_wave.0 * 30;
    let mod_seconds = seconds % 60;
    let minutes: u16 = seconds / 60;
    if mod_seconds == 0 {
        seconds = 0;
    } else {
        seconds = mod_seconds;
    }
    *current_time = CurrentTime { minutes, seconds };

    commands.trigger(UpdateTimeUI);
}

pub fn update_time_ui(
    _trigger: Trigger<UpdateTimeUI>,
    current_time: Res<CurrentTime>,
    mut current_time_ui: Query<(&mut Text, &CurrentTimeUI), Without<CurrentWaveUI>>,
) {
    // Update UI
    if let Ok((mut text, _)) = current_time_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value =
            format!("{:02}:{:02}", current_time.minutes, current_time.seconds);
    }
}

pub fn update_current_alive_enemies_ui(
    _trigger: Trigger<UpdateAliveEnemiesUI>,
    mut current_alive_enemies_ui: Query<(&mut Text, &EnemiesLeftUI)>,
    current_alive_enemies: Query<&Enemy, With<Enemy>>,
) {
    let alive_enemies = current_alive_enemies.iter().len();

    // Update UI
    if let Ok((mut text, _)) = current_alive_enemies_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Alive enemies: {}", alive_enemies);
    }
}

pub fn on_buff_added(
    trigger: Trigger<BuffAdded>,
    mut commands: Commands,
    mut container_buff_ui: Query<(&Children, &ContainerBuffsUI)>,
    mut buff_ui_query: Query<(&mut BuffsUI, &Children)>,
    mut buff_ui_text: Query<&mut Text>,
) {
    let Ok((container_buff_children, _)) = container_buff_ui.get_single_mut() else {
        return;
    };

    let event = trigger.event();
    let item_type = event.item_type.clone();

    let mut buff_counter = 0;

    for &child in container_buff_children {
        let Ok((mut buff_ui, buff_ui_children)) = buff_ui_query.get_mut(child) else {
            continue;
        };
        let buff_type = buff_ui.item_type.clone();
        let current_buffer_counter = buff_ui.counter;

        match (&buff_type, &item_type) {
            (ItemTypeEnum::Speed(_), ItemTypeEnum::Speed(_))
            | (ItemTypeEnum::Armor(_), ItemTypeEnum::Armor(_))
            | (ItemTypeEnum::Shield(_), ItemTypeEnum::Shield(_)) => {
                buff_counter += 1;
            }
            _ => continue,
        };

        if buff_counter != 0 {
            buff_counter = current_buffer_counter + 1;
            buff_ui.counter = buff_counter;

            for &buff_ui_child in buff_ui_children {
                let Ok(mut buff_ui_counter_text) = buff_ui_text.get_mut(buff_ui_child) else {
                    continue;
                };
                buff_ui_counter_text.sections.first_mut().unwrap().value =
                    format!("x{}", buff_counter);
            }
            break;
        }
    }

    // INFO: we only want to add new buffs to the UI if they do not exist
    // there already. If they do, we only increase their counter.
    if buff_counter != 0 {
        return;
    }

    commands.trigger(BuffUIAdd { item_type });
}

pub fn on_weapon_found(
    trigger: Trigger<WeaponFound>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
    weapon_ui: Query<(Entity, &WeaponUI)>,
) {
    let event = trigger.event();
    let WeaponFound {
        weapon_entity,
        weapon,
        weapon_damage,
        player_entity,
        player_weapon_entity,
        player_ammo_entity,
    } = event;

    let direction = Vec3::ZERO;
    let pos = Vec3::new(8.0, 0.0, CHAR_Z_INDEX);
    let weapon_scale = Vec3::new(0.5, 0.5, 1.);
    let ammo_scale = Vec3::ONE;
    let rotation = Quat::default();

    let weapon_type = weapon.weapon_type.clone();
    let damage = weapon_damage.0;
    let layer = PLAYER_LAYER;

    let scale = ammo_scale;
    let ammo_bundle = AmmoBundle::new(
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
        scale,
        pos,
        weapon_type.clone(),
        direction,
        damage,
        rotation,
        layer.clone(),
        *player_entity,
        crate::util::EquippedTypeEnum::Player,
    );

    let scale = weapon_scale;
    let weapon_bundle = WeaponBundle::new(
        &mut texture_atlas_layout,
        &sprites,
        &asset_server,
        scale,
        pos,
        direction,
        damage,
        weapon_type.clone(),
        layer.clone(),
        *player_entity,
        crate::util::EquippedTypeEnum::Player,
    );

    // despawn current player's weapon
    // (otherwise it will only remove the link
    // to the parent entity and will look like it
    // was spawned on the center of the screen)
    commands
        .entity(*player_entity)
        .remove_children(&[*player_weapon_entity]);
    commands.entity(*player_weapon_entity).clear_children();
    commands.entity(*player_weapon_entity).despawn();
    commands.entity(*player_ammo_entity).despawn();

    // Add new weapon and ammo to player's entity
    commands.entity(*player_entity).with_children(|parent| {
        parent.spawn(weapon_bundle).with_children(|parent| {
            parent.spawn(ammo_bundle);
        });
    });

    // play audio when colliding weapon
    hit_weapon_audio(&asset_server, &mut commands);

    // remove collided weapon
    commands.entity(*weapon_entity).despawn();

    // update UI
    commands
        .entity(weapon_ui.get_single().unwrap().0)
        .despawn_recursive();
    let sprite_source = get_weapon_sprite_based_on_weapon_type(weapon_type, &sprites).source;
    spawn_weapon_ui(&mut commands, &asset_server, sprite_source);
}

pub fn on_buff_remove_ui(
    trigger: Trigger<BuffUIRemove>,
    mut commands: Commands,

    mut container_buff_ui: Query<(&Children, &ContainerBuffsUI)>,
    mut buff_ui_query: Query<(Entity, &mut BuffsUI, &Children)>,
    mut buff_ui_text: Query<&mut Text>,
) {
    let event = trigger.event();
    let item_type = event.item_type.clone();

    let Ok((children, _)) = container_buff_ui.get_single_mut() else {
        return;
    };

    for &child in children {
        let Ok((buff_ui_entity, mut buff_ui, buff_ui_children)) = buff_ui_query.get_mut(child)
        else {
            continue;
        };

        let current_buff_counter = buff_ui.counter;

        match (&item_type, &buff_ui.item_type) {
            (ItemTypeEnum::Speed(_), ItemTypeEnum::Speed(_))
            | (ItemTypeEnum::Armor(_), ItemTypeEnum::Armor(_)) => continue,
            (ItemTypeEnum::Shield(_), ItemTypeEnum::Shield(_)) => {
                if current_buff_counter == 1 {
                    commands.entity(buff_ui_entity).despawn_recursive();
                } else {
                    // remove one counter from UI and from buff
                    buff_ui.counter -= 1;

                    for &buff_ui_child in buff_ui_children {
                        let Ok(mut buff_ui_counter_text) = buff_ui_text.get_mut(buff_ui_child)
                        else {
                            continue;
                        };

                        buff_ui_counter_text.sections.first_mut().unwrap().value =
                            format!("x{}", current_buff_counter - 1);
                    }
                }

                break;
            }
            _ => continue,
        };
    }
}

pub fn on_buff_add_ui(
    trigger: Trigger<BuffUIAdd>,
    mut commands: Commands,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    mut container_buff_ui: Query<(Entity, &ContainerBuffsUI)>,
) {
    let Ok((parent, _)) = container_buff_ui.get_single_mut() else {
        return;
    };

    let event = trigger.event();
    let item_type = event.item_type.clone();

    let buff_counter = 1;

    let child_node = |sprite: &str, buff_type: ItemTypeEnum| {
        (
            NodeBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(5.)),
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
                ..default()
            },
            UiImage::new(asset_server.load(sprite.to_owned())),
            OVERLAY_LAYER,
            BuffsUI {
                item_type: buff_type,
                counter: buff_counter,
            },
            CleanupWhenPlayerDies,
        )
    };

    let item_sprite = match &item_type {
        ItemTypeEnum::Speed(speed) => {
            get_item_sprite_based_on_item_type(ItemTypeEnum::Speed(speed.clone()).clone(), &sprites)
        }
        ItemTypeEnum::Armor(armor) => {
            get_item_sprite_based_on_item_type(ItemTypeEnum::Armor(armor.clone()).clone(), &sprites)
        }
        ItemTypeEnum::Shield(shield) => get_item_sprite_based_on_item_type(
            ItemTypeEnum::Shield(shield.clone()).clone(),
            &sprites,
        ),
        _ => return,
    };

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 10.0,
        ..default()
    };

    let buff_counter_ui = (
        TextBundle {
            text: Text::from_section(format!("x{}", buff_counter), text_style),
            style: Style {
                position_type: PositionType::Relative,
                // TODO: get rid of magic numbers
                top: Val::Px(16.),
                left: Val::Px(18.),
                ..default()
            },
            ..default()
        },
        OVERLAY_LAYER,
    );

    let id = commands
        .spawn(child_node(item_sprite.source, item_type))
        .with_children(|parent| {
            parent.spawn(buff_counter_ui);
        })
        .id();
    commands.entity(parent).push_children(&[id]);
}

pub fn on_player_profile_ui_set(
    _trigger: Trigger<PlayerProfileUISet>,
    mut commands: Commands,
    player_profile_ui_query: Query<(Entity, &Children, &PlayerProfileUI)>,
    mut player_bar_ui_root_node_query: Query<(Entity, &Children, &PlayerProfileUIBarsRootNode)>,
    player_health_ui_query: Query<(Entity, &HealthBarUI)>,
    player_mana_ui_query: Query<(Entity, &ManaBarUI)>,
) {
    spawn_health_ui_bar(
        &mut commands,
        &player_profile_ui_query,
        &mut player_bar_ui_root_node_query,
        &player_health_ui_query,
        PLAYER_HEALTH,
        PLAYER_HEALTH,
    );

    spawn_mana_ui_bar(
        &mut commands,
        &player_profile_ui_query,
        &mut player_bar_ui_root_node_query,
        &player_mana_ui_query,
        PLAYER_MANA,
        PLAYER_MANA,
    );
}

pub fn on_power_found(
    _trigger: Trigger<PowerFound>,
    mut commands: Commands,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    current_game_level: Res<CurrentGameLevel>,
    power_levels: Res<PowerLevels>,

    player_query: Query<(Entity, &Children, &Player)>,
    player_powers_query: Query<(Entity, &Power)>,
) {
    let Ok((player_entity, player_children, _)) = player_query.get_single() else {
        return;
    };

    let mod_power_level_index = ((current_game_level.0 - 1) as usize % NUMBER_OF_POWERS) + 1;
    let current_level_power = power_levels
        .0
        .iter()
        .find(|power| power.level == mod_power_level_index);
    if current_level_power.is_none() {
        println!("NO POWER MATCHING level FOUND!!!");
        return;
    }
    let power_by_level = current_level_power.unwrap();

    let power_type = power_by_level.power.power_type.clone();
    let keycode = get_key_code_based_on_power_type(power_type.clone());

    // Remove power from player if it has the same keycode - therefore
    // it is the same type.
    // The reason is because we want to replace it, not to add a `duplicate`
    // one.
    let mut power_increase = None;
    for &child in player_children {
        if let Ok(player_powers) = player_powers_query.get(child) {
            if player_powers.1.trigger_key == keycode {
                power_increase = Some(1.2f32);
                commands
                    .entity(player_entity)
                    .remove_children(&[player_powers.0]);
                break;
            }
        }
    }

    equip_player_with_power(
        &mut commands,
        texture_atlas_layout,
        &sprites,
        &asset_server,
        power_by_level,
        player_entity,
        power_increase,
    );

    commands.trigger(OnUpdatePowerUI {
        power_type,
        keycode,
    });
}

pub fn update_power_ui(
    trigger: Trigger<OnUpdatePowerUI>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,

    power_ui_root_node: Query<(Entity, &PowerUIRootNode)>,
    mut power_ui_query: Query<(&mut PowerUI, &Children), With<PowerUI>>,
    power_sprite_ui_query: Query<&Children, With<PowerSpriteUI>>,
    mut power_level_ui_query: Query<&mut Text, With<PowerLevelUI>>,
) {
    let event = trigger.event();
    let power_type = event.power_type.clone();
    let keycode = event.keycode;

    let sprite_source = get_power_sprite_based_on_power_type(power_type.clone(), &sprites).source;

    let Ok((power_ui_root_node_entity, _)) = power_ui_root_node.get_single() else {
        return;
    };

    let mut found = None;
    for mut power_ui in power_ui_query.iter_mut() {
        if power_ui.0.power_type == power_type {
            // Update level of existing power
            power_ui.0.power_level += 1;

            found = Some(power_ui);
            break;
        }
    }

    // Update level of existing power on the UI
    if found.is_some() {
        let (power_ui, power_children) = found.unwrap();

        for &child in power_children {
            if power_sprite_ui_query.get(child).is_err() {
                continue;
            }

            let power_sprite_ui_children = power_sprite_ui_query.get(child).unwrap();

            for &sprite_child in power_sprite_ui_children {
                if let Ok(mut power_level_ui_text) = power_level_ui_query.get_mut(sprite_child) {
                    power_level_ui_text.sections.first_mut().unwrap().value =
                        format!("{}", power_ui.power_level);
                }
            }
        }

        return;
    }

    // Only spawn new powers
    let child_id = spawn_power_ui(
        &mut commands,
        &asset_server,
        sprite_source,
        power_type,
        keycode,
    );
    commands
        .entity(power_ui_root_node_entity)
        .add_child(child_id);
}

pub fn on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut window_resolution: ResMut<WindowResolutionResource>,
) {
    for e in resize_reader.read() {
        window_resolution.x_px = e.width;
        window_resolution.y_px = e.height;
    }
}

pub fn maybe_spawn_health_points_pack(
    _trigger: Trigger<MaybeSpawnHealthPack>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    current_game_level: Res<CurrentGameLevel>,
    asset_server: Res<AssetServer>,
) {
    let chance = get_random_chance();
    let quantity = 1;
    let level = current_game_level.0;

    if chance < CHANCE_TO_SPAWN_HEALTH_POINTS_PACK {
        spawn_item(
            &mut commands,
            &mut texture_atlas_layout,
            &sprites,
            &asset_server,
            ItemTypeEnum::Health(crate::Health(10.)),
            quantity,
            level,
        );
    }
}

pub fn maybe_spawn_mana_points_pack(
    _trigger: Trigger<MaybeSpawnManaPack>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    current_game_level: Res<CurrentGameLevel>,
    asset_server: Res<AssetServer>,
) {
    let chance = get_random_chance();
    let quantity = 1;
    let level = current_game_level.0;

    if chance < CHANCE_TO_SPAWN_MANA_POINTS_PACK {
        spawn_item(
            &mut commands,
            &mut texture_atlas_layout,
            &sprites,
            &asset_server,
            ItemTypeEnum::Mana(crate::Mana(10.)),
            quantity,
            level,
        );
    }
}

pub fn change_background_texture(
    _trigger: Trigger<ChangeBackgroundTexture>,
    mut commands: Commands,
    current_game_level: Res<CurrentGameLevel>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    background_query: Query<Entity, With<TileBackground>>,
) {
    background_query.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });

    render_background_texture(
        &mut commands,
        &mut texture_atlas_layout,
        &asset_server,
        &sprites,
        current_game_level.0,
    );
}

pub fn on_current_game_level_changed(
    trigger: Trigger<CurrentGameLevelChanged>,
    mut commands: Commands,
    mut current_game_level: ResMut<CurrentGameLevel>,
    mut current_game_level_ui: Query<
        (&mut Text, &CurrentGameLevelUI),
        (Without<CurrentTimeUI>, Without<CurrentWaveUI>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let event = trigger.event();
    let new_level = event.0;

    // Update Current Game Level
    current_game_level.0 = new_level;

    // update current game level UI
    if let Ok((mut text, _)) = current_game_level_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Level #{}", new_level);
    }

    // spawn the in-between levels pause screen
    next_state.set(GameState::InBetweenLevels);

    // Add new power to the player
    commands.trigger(PowerFound);
}
