use crate::{
    enemy::Enemy,
    events::{PlayerHealthChanged, PlayerSpeedChanged},
    item::{Item, ItemStatsType},
    player::Player,
    prelude::*,
    weapon::Ammo,
    AllEnemiesDied, Armor, Health, Speed, Weapon,
};

pub fn check_for_ammo_collisions(
    mut commands: Commands,
    ammos: Query<(Entity, &Transform, &Ammo), (With<Ammo>, Without<Player>)>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        commands.trigger(AllEnemiesDied);
        return;
    }

    for (enemy_entity, enemy_transform, mut enemy) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        for (ammo_entity, ammo_transform, ammo) in ammos.iter() {
            let ammo_collider =
                Aabb2d::new(ammo_transform.translation.truncate(), CAPSULE_COLLIDER);

            if ammo_collider.intersects(&enemy_collider) {
                damage_enemy(
                    &mut commands,
                    ammo_entity,
                    enemy_entity,
                    &mut enemy,
                    ammo.damage,
                );
                continue;
            }
        }
    }
}

pub fn check_for_player_collisions_to_enemy(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Enemy), With<Enemy>>,
    mut player: Query<(Entity, &Transform, &mut Health, &mut Armor), With<Player>>,
) {
    for (enemy_transform, enemy) in enemies.iter_mut() {
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
                    enemy.damage,
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

pub fn check_for_weapon_collisions(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Weapon), With<Player>>,
    weapons: Query<(Entity, &Transform, &Weapon), Without<Player>>,
) {
    for (weapon_entity, weapon_transform, weapon) in weapons.iter() {
        let weapon_collider = Aabb2d::new(
            weapon_transform.translation.truncate(),
            CAPSULE_COLLIDER + 5.,
        );

        if let Ok(result) = player_query.get_single_mut() {
            let (player_entity, player_transform, mut player_weapon) = result;
            // if weapon already belongs to player, there's no need to check
            // for collision
            if weapons.get(player_entity).is_ok() {
                continue;
            }

            let player_collider =
                Aabb2d::new(player_transform.translation.truncate(), CAPSULE_COLLIDER);

            if player_collider.intersects(&weapon_collider) {
                let weapon_cloned = weapon.clone();
                player_weapon.ammo = weapon_cloned.ammo;
                player_weapon.pos = weapon_cloned.pos;
                player_weapon.source = weapon_cloned.source;
                commands.entity(weapon_entity).despawn();
            }
        }
    }
}

fn damage_enemy(
    commands: &mut Commands,
    ammo_entity: Entity,
    enemy_entity: Entity,
    enemy: &mut Enemy,
    damage: f32,
) {
    enemy.health -= damage;

    // Always despawns ammo
    commands.entity(ammo_entity).despawn();

    if enemy.health <= 0. {
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
