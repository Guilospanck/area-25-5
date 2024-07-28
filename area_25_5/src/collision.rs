use crate::{
    ammo::Ammo,
    enemy::Enemy,
    events::{PlayerHealthChanged, PlayerSpeedChanged},
    item::{Item, ItemStatsType},
    player::Player,
    prelude::*,
    AllEnemiesDied, AmmoBundle, Armor, Damage, Health, Speed, SpritesResources, Weapon,
    WeaponBundle,
};

pub fn check_for_ammo_collisions(
    mut commands: Commands,
    ammos_query: Query<(Entity, &Transform), With<Ammo>>,
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,

    player_query: Query<&Children, With<Player>>,
    player_weapon_query: Query<(&Weapon, &Damage)>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        commands.trigger(AllEnemiesDied);
        return;
    }

    let player_children = player_query.get_single().unwrap();
    let mut player_weapon = None;
    for &child in player_children {
        if let Ok(pw) = player_weapon_query.get(child) {
            player_weapon = Some(pw);
            break;
        }
    }
    let player_weapon = player_weapon.unwrap();
    let player_weapon_damage = player_weapon.1;

    for (enemy_entity, enemy_transform, mut enemy_health) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        for (ammo_entity, ammo_transform) in ammos_query.iter() {
            let ammo_collider =
                Aabb2d::new(ammo_transform.translation.truncate(), Vec2::new(16., 16.));

            if ammo_collider.intersects(&enemy_collider) {
                damage_enemy(
                    &mut commands,
                    ammo_entity,
                    enemy_entity,
                    &mut enemy_health,
                    player_weapon_damage.0,
                );
                continue;
            }
        }
    }
}

pub fn check_for_player_collisions_to_enemy(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &Damage), With<Enemy>>,
    mut player: Query<(Entity, &Transform, &mut Health, &mut Armor), With<Player>>,
) {
    for (enemy_transform, enemy_damage) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        if let Ok(result) = player.get_single_mut() {
            let (player_entity, player_transform, mut player_health, mut player_armor) = result;
            let player_collider =
                Aabb2d::new(player_transform.translation.truncate(), CAPSULE_COLLIDER);

            if player_collider.intersects(&enemy_collider) {
                damage_player(
                    &mut commands,
                    player_entity,
                    &mut player_health,
                    &mut player_armor,
                    enemy_damage.0,
                );
            }
        }
    }
}

pub fn check_for_item_collisions(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut Speed), With<Player>>,
    items: Query<(Entity, &Transform, &Item)>,
) {
    for (item_entity, item_transform, item) in items.iter() {
        let item_collider =
            Aabb2d::new(item_transform.translation.truncate(), CAPSULE_COLLIDER + 5.);

        if let Ok(result) = player.get_single_mut() {
            let (player_transform, mut player_speed) = result;
            let player_collider =
                Aabb2d::new(player_transform.translation.truncate(), CAPSULE_COLLIDER);

            if player_collider.intersects(&item_collider) {
                player_speed.0 += item.value;
                match item.stats {
                    ItemStatsType::Speed => {
                        commands.trigger(PlayerSpeedChanged {
                            speed: player_speed.0,
                        });
                    }
                    ItemStatsType::Armor => todo!(),
                }
                commands.entity(item_entity).despawn();
            }
        }
    }
}

/// Player with weapon
pub fn check_for_weapon_collisions(
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    player_query: Query<(Entity, &Transform, &Children), With<Player>>,
    player_weapon_query: Query<(Entity, &Weapon)>,
    weapons_not_from_player_query: Query<(Entity, &Weapon, &Damage, &Transform), Without<Player>>,
) {
    // Get an entity that has player
    if player_query.get_single().is_err() {
        println!("Player not found on check_for_weapon_collisions");
        return;
    }
    let player = player_query.get_single().unwrap();
    let player_entity = player.0;
    let player_transform = player.1;
    let player_children = player.2;
    let player_collider = Aabb2d::new(player_transform.translation.truncate(), CAPSULE_COLLIDER);

    let mut player_weapon = None;
    for &child in player_children {
        if let Ok(pw) = player_weapon_query.get(child) {
            player_weapon = Some(pw);
        }
    }

    // Check for collision of the player entity with the weapons on the map
    let direction = Vec3::ZERO;
    let pos = Vec3::new(8.0, 0.0, CHAR_Z_INDEX);
    let weapon_scale = Vec3::new(0.5, 0.5, 1.);
    let ammo_scale = Vec3::ONE;
    let rotation = Quat::default();
    for (weapon_entity, weapon, weapon_damage, weapon_transform) in
        weapons_not_from_player_query.iter()
    {
        // if the weapon belongs to the player, do not check for collision
        if weapon_entity == player_weapon.unwrap().0 {
            continue;
        }

        let weapon_collider = Aabb2d::new(
            weapon_transform.translation.truncate(),
            CAPSULE_COLLIDER + 5.,
        );

        // if we interact with a weapon on the map,
        // we despawn it and swap our current weapon by the new one
        if player_collider.intersects(&weapon_collider) {
            let weapon_type = weapon.0.clone();
            let damage = weapon_damage.0;

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
                weapon_type,
            );

            // despawn current player's weapon
            // (otherwise it will only remove the link
            // to the parent entity and will look like it
            // was spawned on the center of the screen)
            commands
                .entity(player_weapon.unwrap().0)
                .despawn_recursive();

            commands.entity(player_entity).clear_children();

            // Add new weapon and ammo to player's entity
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn(weapon_bundle).with_children(|parent| {
                    parent.spawn(ammo_bundle);
                });
            });

            // remove collided weapon
            commands.entity(weapon_entity).despawn();

            return;
        }
    }
}

fn damage_enemy(
    commands: &mut Commands,
    ammo_entity: Entity,
    enemy_entity: Entity,
    enemy_health: &mut Health,
    damage: f32,
) {
    enemy_health.0 -= damage;

    // Always despawns ammo
    commands.entity(ammo_entity).despawn();

    if enemy_health.0 <= 0. {
        commands.entity(enemy_entity).despawn();
    }
}

fn damage_player(
    commands: &mut Commands,
    player_entity: Entity,
    player_health: &mut Health,
    player_armor: &mut Armor,
    damage: f32,
) {
    let new_damage = damage - player_armor.0 * 0.02;
    let mut new_player_health = player_health.0 - new_damage;
    if new_player_health <= 0. {
        new_player_health = 0.;
    }

    player_health.0 = new_player_health;

    commands.trigger(PlayerHealthChanged {
        health: player_health.0,
    });

    if player_health.0 <= 0. {
        // YOU DIED!!!
        println!("DEAD");
        commands.entity(player_entity).despawn();
    }
}
