use std::time::Duration;

use bevy::log::tracing_subscriber::fmt::format;

use crate::{
    audio::hit_weapon_audio,
    game_actions::shoot,
    player::Player,
    prelude::*,
    spawn_enemy, spawn_health_bar, spawn_item, spawn_weapon, spawn_weapon_ui,
    ui::HealthBar,
    util::{get_item_sprite_based_on_item_type, get_weapon_sprite_based_on_weapon_type},
    AmmoBundle, Armor, Buff, BuffGroup, BuffsUI, ContainerBuffsUI, CurrentScore, CurrentTime,
    CurrentTimeUI, CurrentWave, CurrentWaveUI, Damage, Enemy, EnemyWaves, GameState, Item,
    ItemTypeEnum, ItemWaves, PlayerArmorBar, PlayerSpeedBar, ScoreUI, Speed, SpriteInfo,
    SpritesResources, Weapon, WeaponBundle, WeaponUI, WeaponWaves,
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
pub struct PlayerArmorChanged {
    pub armor: f32,
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
pub struct CurrentWaveChanged;

#[derive(Event)]
pub struct CurrentTimeChanged;

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
    pub player_weapon_entity: Option<Entity>,
    pub player_ammo_entity: Option<Entity>,
}

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

pub fn on_player_armor_changed(
    trigger: Trigger<PlayerArmorChanged>,
    mut player_armor_bar: Query<&mut Text, With<PlayerArmorBar>>,
) {
    let event = trigger.event();
    let armor = event.armor;

    if let Ok(mut text) = player_armor_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = armor.to_string();
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
    mut current_time: ResMut<CurrentTime>,
    mut next_state: ResMut<NextState<GameState>>,
    player_state: Res<State<GameState>>,
) {
    // Add multiplier to score based on the time left
    let mut seconds = current_time.seconds;
    seconds += current_time.minutes * 60;
    let score = SCORE_MULTIPLIER * seconds as f32;
    commands.trigger(ScoreChanged { score });

    // Update and cap current wave
    let new_wave = current_wave.0 + 1;
    if new_wave as usize > NUMBER_OF_WAVES {
        if *player_state.get() != GameState::Won {
            next_state.set(GameState::Won);
        }
        return;
    }
    current_wave.0 = new_wave;
    commands.trigger(CurrentWaveChanged);

    // Update current time
    let mut seconds: u16 = new_wave * 30;
    let mod_seconds = seconds % 60;
    let minutes: u16 = seconds / 60;
    if mod_seconds == 0 {
        seconds = 0;
    } else {
        seconds = mod_seconds;
    }
    *current_time = CurrentTime { minutes, seconds };
    commands.trigger(CurrentTimeChanged);
}

pub fn on_wave_changed(
    _trigger: Trigger<CurrentWaveChanged>,
    current_wave: Res<CurrentWave>,
    enemy_waves: Res<EnemyWaves>,
    weapon_waves: Res<WeaponWaves>,
    item_waves: Res<ItemWaves>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_wave_ui: Query<(&mut Text, &CurrentWaveUI), Without<CurrentTimeUI>>,
    weapons: Query<(Entity, Option<&Parent>), With<Weapon>>,
    items: Query<Entity, With<Item>>,
) {
    // Despawn items and weapons that were spawned on the map
    for item in items.iter() {
        commands.entity(item).despawn();
    }

    for weapon in weapons.iter() {
        if weapon.1.is_none() {
            commands.entity(weapon.0).despawn();
        }
    }

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
    if let Ok((mut text, _)) = current_wave_ui.get_single_mut() {
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
    commands.trigger(CurrentTimeChanged);
}

pub fn remove_outdated_buffs(
    mut commands: Commands,
    mut player: Query<(&mut Speed, &mut Armor, &Children), With<Player>>,
    player_buff_group_query: Query<(Entity, &BuffGroup)>,
) {
    if player.get_single_mut().is_err() {
        return;
    }
    let (_, mut player_armor, player_children) = player.get_single_mut().unwrap();

    let should_be_despawned = |buff_group: BuffGroup,
                               player_armor: &mut Armor,
                               commands: &mut Commands,
                               buff_ui_despawned: Option<ItemTypeEnum>|
     -> bool {
        match &buff_group.item {
            crate::ItemTypeEnum::Speed(_) | crate::ItemTypeEnum::Armor(_) => false,
            crate::ItemTypeEnum::Shield(shield) => {
                if shield.duration_seconds.is_none() {
                    return false;
                }

                let has_passed = buff_group.start_time.elapsed()
                    > Duration::from_secs(shield.duration_seconds.unwrap());

                if has_passed {
                    // update player armor
                    // TODO: check for shield type (magical vs physical)
                    if shield.defensive > 0. {
                        player_armor.0 -= shield.defensive * NUMBER_OF_BUFF_ITEMS as f32;
                        commands.trigger(PlayerArmorChanged {
                            armor: player_armor.0,
                        });
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
        if let Ok((player_buff_group_entity, player_buff_group)) =
            player_buff_group_query.get(child)
        {
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

    if player_query.get_single_mut().is_err() {
        return;
    }
    let player_children = player_query.get_single_mut().unwrap();

    for &child in player_children.iter() {
        if player_buff_group_query.get(child).is_err() {
            continue;
        }
        let (player_buff_group_children, _) = player_buff_group_query.get(child).unwrap();

        for (idx, &player_buff_group_child) in player_buff_group_children.iter().enumerate() {
            if let Ok((mut player_buff_transform, _player_buff)) =
                player_buff_query.get_mut(player_buff_group_child)
            {
                let radians = 0.017_453_292
                    * degrees
                    * (idx + 1) as f32
                    * (NUMBER_OF_POSITIONS / NUMBER_OF_BUFF_ITEMS) as f32;
                let (mut y, mut x) = f32::sin_cos(radians);
                y *= RADIUS_FROM_PLAYER;
                x *= RADIUS_FROM_PLAYER;

                player_buff_transform.translation = Vec3::new(x, y, 1.);
            }
        }
    }
}

pub fn on_current_time_changed(
    _trigger: Trigger<CurrentTimeChanged>,
    current_time: Res<CurrentTime>,
    mut current_time_ui: Query<(&mut Text, &CurrentTimeUI), Without<CurrentWaveUI>>,
) {
    if let Ok((mut text, _)) = current_time_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value =
            format!("{:02}:{:02}", current_time.minutes, current_time.seconds);
    }
}

pub fn on_buff_added(
    trigger: Trigger<BuffAdded>,
    mut commands: Commands,
    mut container_buff_ui: Query<(Entity, &Children, &ContainerBuffsUI)>,
    mut buff_ui_query: Query<(&mut BuffsUI, &Children)>,
    mut buff_ui_text: Query<&mut Text>,
) {
    if let Err(err) = container_buff_ui.get_single_mut() {
        eprintln!("{err}");
        return;
    }
    let (parent, container_buff_children, _) = container_buff_ui.get_single_mut().unwrap();

    let event = trigger.event();
    let item_type = event.item_type.clone();

    let mut buff_counter = 0;

    for &child in container_buff_children {
        if buff_ui_query.get_mut(child).is_err() {
            continue;
        }
        let (mut buff_ui, buff_ui_children) = buff_ui_query.get_mut(child).unwrap();
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
                if buff_ui_text.get_mut(buff_ui_child).is_err() {
                    continue;
                }
                let mut buff_ui_counter_text = buff_ui_text.get_mut(buff_ui_child).unwrap();
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

    let weapon_type = weapon.0.clone();
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
    );

    // despawn current player's weapon
    // (otherwise it will only remove the link
    // to the parent entity and will look like it
    // was spawned on the center of the screen)
    if let Some(player_weapon_entity_unwrapped) = player_weapon_entity {
        commands
            .entity(*player_entity)
            .remove_children(&[*player_weapon_entity_unwrapped]);
        commands
            .entity(*player_weapon_entity_unwrapped)
            .clear_children();
        commands.entity(*player_weapon_entity_unwrapped).despawn();
    }
    if let Some(player_ammo_entity_unwrapped) = player_ammo_entity {
        commands.entity(*player_ammo_entity_unwrapped).despawn();
    }

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

    if container_buff_ui.get_single_mut().is_err() {
        return;
    }
    let (children, _) = container_buff_ui.get_single_mut().unwrap();

    for &child in children {
        if buff_ui_query.get_mut(child).is_err() {
            continue;
        }
        let (buff_ui_entity, mut buff_ui, buff_ui_children) = buff_ui_query.get_mut(child).unwrap();
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
                        if buff_ui_text.get_mut(buff_ui_child).is_err() {
                            continue;
                        }
                        let mut buff_ui_counter_text = buff_ui_text.get_mut(buff_ui_child).unwrap();
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
    if let Err(err) = container_buff_ui.get_single_mut() {
        eprintln!("{err}");
        return;
    }
    let (parent, _) = container_buff_ui.get_single_mut().unwrap();

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
        )
    };

    let mut item_sprite = SpriteInfo::default();
    match &item_type {
        ItemTypeEnum::Speed(speed) => {
            item_sprite = get_item_sprite_based_on_item_type(
                ItemTypeEnum::Speed(speed.clone()).clone(),
                &sprites,
            );
        }
        ItemTypeEnum::Armor(armor) => {
            item_sprite = get_item_sprite_based_on_item_type(
                ItemTypeEnum::Armor(armor.clone()).clone(),
                &sprites,
            );
        }
        ItemTypeEnum::Shield(shield) => {
            item_sprite = get_item_sprite_based_on_item_type(
                ItemTypeEnum::Shield(shield.clone()).clone(),
                &sprites,
            );
        }
    }

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
