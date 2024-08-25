use crate::{
    ammo::Ammo,
    audio::{hit_enemy_audio, hit_item_audio, player_hit_audio},
    enemy::Enemy,
    events::PlayerHealthChanged,
    item::Item,
    player::Player,
    prelude::*,
    util::check_if_collides_with_power_based_on_power_type,
    AllEnemiesDied, Armor, BaseCamera, Buff, BuffAdded, BuffBundle, BuffGroup, BuffGroupBundle,
    CircleOfDeath, Damage, EnemyHealthChanged, GameOver, Health, ItemTypeEnum, Laser,
    PlayerHitAudioTimeout, Power, ScoreChanged, Speed, SpritesResources, Weapon, WeaponFound,
};

pub fn check_for_offensive_buff_collisions_with_enemy(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &mut Health, &Damage), With<Enemy>>,

    player_query: Query<(&Transform, &Children), With<Player>>,
    player_buff_group_query: Query<(&Children, &BuffGroup)>,
    player_buff_query: Query<(&Transform, &Buff)>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        return;
    }

    let Ok((player_transform, player_children)) = player_query.get_single() else {
        return;
    };

    for &child in player_children {
        let Ok((player_buff_group_children, _)) = player_buff_group_query.get(child) else {
            continue;
        };

        for &player_buff_group_child in player_buff_group_children {
            let Ok((player_buff_transform, player_buff)) =
                player_buff_query.get(player_buff_group_child)
            else {
                continue;
            };

            match &player_buff.item {
                // Speed and armor do not deal damage to the enemies
                ItemTypeEnum::Speed(_) | ItemTypeEnum::Armor(_) => continue,
                ItemTypeEnum::Shield(shield) => {
                    if shield.offensive == 0. {
                        continue;
                    }

                    let damage = shield.offensive;
                    let transform = player_transform.translation.truncate()
                        + player_buff_transform.translation.truncate();
                    let buff_collider = Aabb2d::new(
                        transform,
                        // TODO: make this a config
                        Vec2::new(8., 8.),
                    );

                    for (enemy_entity, enemy_transform, mut enemy_health, enemy_damage) in
                        enemies.iter_mut()
                    {
                        let enemy_collider = Aabb2d::new(
                            enemy_transform.translation.truncate(),
                            Vec2::new(
                                ENEMY_COLLISION_BOX_WIDTH * enemy_transform.scale.x / 2.,
                                ENEMY_COLLISION_BOX_HEIGHT * enemy_transform.scale.y / 2.,
                            ),
                        );

                        if buff_collider.intersects(&enemy_collider) {
                            damage_enemy(
                                &mut commands,
                                enemy_entity,
                                &mut enemy_health,
                                damage,
                                enemy_damage,
                            );
                            continue;
                        }
                    }
                }
            }
        }
    }
}

pub fn check_for_ammo_collisions_with_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ammos_query: Query<(Entity, &Transform), With<Ammo>>,
    mut enemies: Query<(Entity, &Transform, &mut Health, &Damage), With<Enemy>>,

    player_query: Query<&Children, With<Player>>,
    player_weapon_query: Query<(&Children, &Weapon, &Damage)>,
    player_ammo_query: Query<(Entity, &Ammo)>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        commands.trigger(AllEnemiesDied);
        return;
    }

    let Ok(player_children) = player_query.get_single() else {
        return;
    };

    let mut player_weapon = None;
    let mut player_ammo = None;
    for &child in player_children {
        if let Ok(pw) = player_weapon_query.get(child) {
            player_weapon = Some(pw);
            for &child in pw.0 {
                if let Ok(pa) = player_ammo_query.get(child) {
                    player_ammo = Some(pa);
                }
            }
            break;
        }
    }
    let Some(player_weapon) = player_weapon else {
        return;
    };
    let player_weapon_damage = player_weapon.2;

    for (enemy_entity, enemy_transform, mut enemy_health, enemy_damage) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        for (ammo_entity, ammo_transform) in ammos_query.iter() {
            // Do not check for collision with the ammo that the player
            // carries within himself.
            if let Some(player_ammo_unwrapped) = player_ammo {
                if player_ammo_unwrapped.0 == ammo_entity {
                    continue;
                }
            }

            // TODO: turn this half size into config
            let ammo_collider =
                Aabb2d::new(ammo_transform.translation.truncate(), Vec2::new(16., 16.));

            if ammo_collider.intersects(&enemy_collider) {
                hit_enemy_audio(&asset_server, &mut commands);
                damage_enemy_from_ammo_or_power(
                    &mut commands,
                    Some(ammo_entity),
                    enemy_entity,
                    &mut enemy_health,
                    player_weapon_damage.0,
                    enemy_damage,
                );
                continue;
            }
        }
    }
}

pub fn check_for_player_collisions_to_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut audio_timeout: ResMut<PlayerHitAudioTimeout>,
    mut enemies: Query<(&Transform, &Damage), With<Enemy>>,
    mut player: Query<(&Transform, &mut Health, &Armor), With<Player>>,
) {
    let Ok((player_transform, mut player_health, player_armor)) = player.get_single_mut() else {
        return;
    };

    let player_collider = Aabb2d::new(player_transform.translation.truncate(), CAPSULE_COLLIDER);

    for (enemy_transform, enemy_damage) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        if player_collider.intersects(&enemy_collider) {
            // play audio when player was hit
            player_hit_audio(&asset_server, &time, &mut commands, &mut audio_timeout);

            damage_player(
                &mut commands,
                &mut player_health,
                player_armor.0,
                enemy_damage.0,
            );
        }
    }
}

/// Item collision with the player
pub fn check_for_item_collisions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,

    mut player: Query<(&Transform, &mut Speed, &mut Armor, Entity), With<Player>>,
    items: Query<(Entity, &Transform, &Item)>,
    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,
) {
    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    let Ok((player_transform, mut player_speed, mut player_armor, player_entity)) =
        player.get_single_mut()
    else {
        return;
    };

    // This gets the current player position on the world based on his
    // screen position.
    let player_center = Vec2::new(
        player_transform.translation.x + base_camera_transform.translation.x,
        player_transform.translation.y + base_camera_transform.translation.y,
    );
    let player_collider = Aabb2d::new(player_center, Vec2::splat(PLAYER_SPRITE_SIZE as f32 / 2.));

    for (item_entity, item_transform, item) in items.iter() {
        let item_collider = Aabb2d::new(
            item_transform.translation.truncate(),
            Vec2::splat(ITEM_SPRITE_SIZE as f32 / 2.),
        );

        if player_collider.intersects(&item_collider) {
            match &item.item_type {
                ItemTypeEnum::Speed(speed) => {
                    player_speed.0 += speed.0;
                }
                ItemTypeEnum::Armor(armor) => {
                    player_armor.0 += armor.0;
                }
                ItemTypeEnum::Shield(shield) => {
                    // TODO: check for shield type (magical vs physical)
                    if shield.defensive > 0. {
                        player_armor.0 += shield.defensive * NUMBER_OF_BUFF_ITEMS as f32;
                    }

                    // Add new buff to player
                    let layer = PLAYER_LAYER;
                    let scale = Vec3::splat(0.5);
                    let pos = Vec3::new(RADIUS_FROM_PLAYER, RADIUS_FROM_PLAYER, 0.0);

                    let buff_group_bundle =
                        BuffGroupBundle::new(item.item_type.clone(), layer.clone());

                    commands.entity(player_entity).with_children(|parent| {
                        parent.spawn(buff_group_bundle).with_children(|parent| {
                            for _ in 0..NUMBER_OF_BUFF_ITEMS {
                                let buff_bundle = BuffBundle::new(
                                    &mut texture_atlas_layout,
                                    &sprites,
                                    &asset_server,
                                    scale,
                                    pos,
                                    item.item_type.clone(),
                                    layer.clone(),
                                );
                                parent.spawn(buff_bundle);
                            }
                        });
                    });
                }
            }

            commands.trigger(BuffAdded {
                item_type: item.item_type.clone(),
            });

            // play audio when colliding item
            hit_item_audio(&asset_server, &mut commands);
            commands.entity(item_entity).despawn();
        }
    }
}

/// Player with weapon
pub fn check_for_weapon_collisions(
    mut commands: Commands,

    player_query: Query<(Entity, &Transform, &Children), With<Player>>,
    player_weapon_query: Query<(&Children, Entity, &Weapon)>,
    player_ammo_query: Query<(Entity, &Ammo)>,
    weapons_not_from_player_query: Query<(Entity, &Weapon, &Damage, &Transform), Without<Player>>,
    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,
) {
    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    // Get an entity that has player
    let Ok((player_entity, player_transform, player_children)) = player_query.get_single() else {
        return;
    };

    // This gets the current player position on the world based on his
    // screen position.
    let player_center = Vec2::new(
        player_transform.translation.x + base_camera_transform.translation.x,
        player_transform.translation.y + base_camera_transform.translation.y,
    );
    let player_collider = Aabb2d::new(player_center, CAPSULE_COLLIDER);

    let mut player_weapon = None;
    let mut player_ammo = None;
    for &child in player_children {
        if let Ok(pw) = player_weapon_query.get(child) {
            player_weapon = Some(pw);
            for &child in pw.0 {
                if let Ok(pa) = player_ammo_query.get(child) {
                    player_ammo = Some(pa);
                }
            }
            break;
        }
    }

    let Some((_, player_weapon_entity, _)) = player_weapon else {
        return;
    };

    let Some((player_ammo_entity, _)) = player_ammo else {
        return;
    };

    // Check for collision of the player entity with the weapons on the map
    for (weapon_entity, weapon, weapon_damage, weapon_transform) in
        weapons_not_from_player_query.iter()
    {
        // if the weapon belongs to the player, do not check for collision
        if weapon_entity == player_weapon_entity {
            continue;
        }

        let weapon_collider = Aabb2d::new(
            weapon_transform.translation.truncate(),
            CAPSULE_COLLIDER + 5.,
        );

        // if we interact with a weapon on the map,
        // we despawn it and swap our current weapon by the new one
        if player_collider.intersects(&weapon_collider) {
            commands.trigger(WeaponFound {
                weapon_entity,
                weapon: weapon.clone(),
                weapon_damage: weapon_damage.clone(),
                player_entity,
                player_weapon_entity,
                player_ammo_entity,
            });
            return;
        }
    }
}

pub fn check_for_power_collisions_with_enemy(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &mut Health, &Damage), With<Enemy>>,

    base_camera: Query<(&Transform, &BaseCamera), Without<Player>>,

    player_query: Query<(&Children, &Player)>,
    player_powers_query: Query<(Entity, &Power)>,

    powers_query: Query<(Entity, &Transform, &Damage, &Power), With<Power>>,
    circle_of_death_query: Query<&CircleOfDeath, With<CircleOfDeath>>,
    laser_query: Query<&Laser, With<Laser>>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        return;
    }

    let Ok((base_camera_transform, _)) = base_camera.get_single() else {
        return;
    };

    let Ok((player_children, _)) = player_query.get_single() else {
        return;
    };

    let mut current_player_powers_entity: Vec<Entity> = vec![];
    for &child in player_children {
        if let Ok(player_powers) = player_powers_query.get(child) {
            current_player_powers_entity.push(player_powers.0);
        }
    }

    for (enemy_entity, enemy_transform, mut enemy_health, enemy_damage) in enemies.iter_mut() {
        // This gets the current enemy position on the world based on his
        // screen position.
        let enemy_center = Vec2::new(
            enemy_transform.translation.x + base_camera_transform.translation.x,
            enemy_transform.translation.y + base_camera_transform.translation.y,
        );
        let enemy_collider = Aabb2d::new(
            enemy_center,
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        for (power_entity, power_transform, power_damage, power) in powers_query.iter() {
            // if current power is from player, do not collide it
            if current_player_powers_entity.contains(&power_entity) {
                continue;
            }

            let power_collider = Aabb2d::new(
                power_transform.translation.truncate(),
                Vec2::new(
                    (POWER_SPRITE_SIZE / 2) as f32,
                    (POWER_SPRITE_SIZE / 2) as f32,
                ),
            );

            let collides = check_if_collides_with_power_based_on_power_type(
                power.power_type.clone(),
                enemy_collider,
                power_collider,
                &circle_of_death_query,
                &laser_query,
            );

            let mut power_entity_to_be_despawned = None;

            // Only despawns the explosion because the
            // Laser and Circle have their own way of despawning
            if power.power_type == PowerTypeEnum::Explosions {
                power_entity_to_be_despawned = Some(power_entity);
            }

            if collides {
                damage_enemy_from_ammo_or_power(
                    &mut commands,
                    power_entity_to_be_despawned,
                    enemy_entity,
                    &mut enemy_health,
                    power_damage.0,
                    enemy_damage,
                );
            }
        }
    }
}

fn damage_enemy_from_ammo_or_power(
    commands: &mut Commands,
    ammo_or_power_entity: Option<Entity>,
    enemy_entity: Entity,
    enemy_health: &mut Health,
    damage: f32,
    enemy_damage: &Damage,
) {
    if let Some(entity) = ammo_or_power_entity {
        commands.entity(entity).despawn();
    }
    damage_enemy(commands, enemy_entity, enemy_health, damage, enemy_damage);
}

fn damage_enemy(
    commands: &mut Commands,
    enemy_entity: Entity,
    enemy_health: &mut Health,
    damage: f32,
    enemy_damage: &Damage,
) {
    enemy_health.0 -= damage;

    if enemy_health.0 <= 0. {
        commands.entity(enemy_entity).despawn_recursive();
        // INFO: we use the damage of the enemy to how much points the player
        // will get
        commands.trigger(ScoreChanged {
            score: enemy_damage.0,
        });
    }

    commands.trigger(EnemyHealthChanged {
        health: enemy_health.0,
        entity: enemy_entity,
    });
}

fn damage_player(
    commands: &mut Commands,
    player_health: &mut Health,
    player_armor: f32,
    damage: f32,
) {
    // reduces damage based on the armor of the player
    let mut new_damage = damage - player_armor * 0.02;
    if new_damage <= 0. {
        new_damage = 0.0;
    }

    let new_player_health = player_health.0 - new_damage;
    if new_player_health <= 0. {
        commands.trigger(GameOver);
        return;
    }

    player_health.0 = new_player_health;

    commands.trigger(PlayerHealthChanged {
        health: player_health.0,
    });
}
