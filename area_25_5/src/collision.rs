use crate::{enemy::Enemy, events::AlienHealthChanged, player::Alien, prelude::*, weapon::Ammo};

pub fn check_for_ammo_collisions(
    mut commands: Commands,
    ammos: Query<(Entity, &Transform, &Ammo), (With<Ammo>, Without<Alien>)>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
) {
    let capsule_collider = Vec2::new((CAPSULE_LENGTH + CAPSULE_RADIUS * 2.) / 2., CAPSULE_RADIUS);

    for (enemy_entity, enemy_transform, mut enemy) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(enemy_transform.translation.truncate(), capsule_collider);

        for (ammo_entity, ammo_transform, ammo) in ammos.iter() {
            let ammo_collider =
                Aabb2d::new(ammo_transform.translation.truncate(), capsule_collider);

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

pub fn check_for_alien_collisions(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Enemy), With<Enemy>>,
    mut alien: Query<(Entity, &Transform, &mut Alien)>,
) {
    let capsule_collider = Vec2::new((CAPSULE_LENGTH + CAPSULE_RADIUS * 2.) / 2., CAPSULE_RADIUS);

    for (enemy_transform, enemy) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(enemy_transform.translation.truncate(), capsule_collider);

        if let Ok(result) = alien.get_single_mut() {
            let (alien_entity, alien_transform, mut alien) = result;
            let alien_collider =
                Aabb2d::new(alien_transform.translation.truncate(), capsule_collider);

            if alien_collider.intersects(&enemy_collider) {
                damage_alien(&mut commands, alien_entity, &mut alien, enemy.damage);
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

fn damage_alien(commands: &mut Commands, alien_entity: Entity, alien: &mut Alien, damage: f32) {
    alien.health -= damage;

    commands.trigger(AlienHealthChanged {
        health: alien.health,
    });

    if alien.health <= 0. {
        // YOU DIED!!!
        commands.entity(alien_entity).despawn();
    }
}
