use crate::{
    prelude::*, CircleOfDeath, ItemTypeEnum, Laser, PowerTypeEnum, SpriteInfo, SpritesResources,
};
use bevy::math::bounding::BoundingVolume;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub(crate) fn get_unit_direction_vector(origin: Vec2, end: Vec2) -> Vec2 {
    let direction_x = end.x - origin.x;
    let direction_y = -(end.y - origin.y);
    let direction = Vec2::new(direction_x, direction_y);
    get_unit_vector(direction)
}

pub(crate) fn get_unit_vector(vec: Vec2) -> Vec2 {
    let modulo_x: f32 = vec.x.powi(2);
    let modulo_y: f32 = vec.y.powi(2);
    let modulo: f32 = modulo_x + modulo_y;
    let modulo: f32 = modulo.sqrt();

    let normalized_direction_x = vec.x / modulo;
    let normalized_direction_y = vec.y / modulo;

    Vec2::new(normalized_direction_x, normalized_direction_y)
}
pub(crate) fn get_random_vec3(increment: u64, seed: Option<u64>) -> Vec3 {
    let random_seed = seed.unwrap_or(ENEMY_RANDOM_SEED);
    let mut rng = ChaCha8Rng::seed_from_u64(random_seed + increment);

    Vec3::new(
        (rng.gen::<f32>() - 0.5) * (BACKGROUND_TEXTURE_RESOLUTION.x_px - 100.0),
        (rng.gen::<f32>() - 0.5) * (BACKGROUND_TEXTURE_RESOLUTION.y_px - 100.0),
        CHAR_Z_INDEX,
    )
}

pub(crate) fn get_ammo_sprite_based_on_weapon_type(
    weapon_type: WeaponTypeEnum,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match weapon_type {
        WeaponTypeEnum::Bow => sprites.0.arrow.clone(),
        WeaponTypeEnum::Wand => sprites.0.magic_ball.clone(),
    }
}

pub(crate) fn get_weapon_sprite_based_on_weapon_type(
    weapon_type: WeaponTypeEnum,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match weapon_type {
        WeaponTypeEnum::Bow => sprites.0.bow.clone(),
        WeaponTypeEnum::Wand => sprites.0.wand.clone(),
    }
}

pub(crate) fn get_item_sprite_based_on_item_type(
    item_type: ItemTypeEnum,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match item_type {
        ItemTypeEnum::Speed(_) => sprites.0.lightning.clone(),
        ItemTypeEnum::Armor(_) => sprites.0.shield.clone(),
        ItemTypeEnum::Shield(_) => sprites.0.diamond.clone(),
        ItemTypeEnum::Health(_) => sprites.0.hp_pack.clone(),
    }
}

pub(crate) fn get_power_sprite_based_on_power_type(
    power_type: PowerTypeEnum,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match power_type {
        PowerTypeEnum::Explosions => sprites.0.diamond.clone(),
        PowerTypeEnum::CircleOfDeath => sprites.0.magic_ball.clone(),
        PowerTypeEnum::Laser => sprites.0.diamond.clone(),
    }
}

pub(crate) fn get_key_code_based_on_power_type(power_type: PowerTypeEnum) -> KeyCode {
    match power_type {
        PowerTypeEnum::Explosions => KeyCode::KeyL,
        PowerTypeEnum::CircleOfDeath => KeyCode::KeyJ,
        PowerTypeEnum::Laser => KeyCode::KeyH,
    }
}

pub(crate) fn check_if_collides_with_power_based_on_power_type(
    power_type: PowerTypeEnum,
    collider: Aabb2d,
    power_collider: Aabb2d,
    circle_of_death_query: &Query<&CircleOfDeath, With<CircleOfDeath>>,
    laser_query: &Query<&Laser, With<Laser>>,
) -> bool {
    match power_type {
        PowerTypeEnum::Explosions => power_collider.intersects(&collider),
        PowerTypeEnum::CircleOfDeath => {
            for circle_of_death in circle_of_death_query.iter() {
                let CircleOfDeath {
                    inner_circle_radius,
                    outer_circle_radius,
                } = circle_of_death;

                if (collider.min.x >= *inner_circle_radius
                    || -collider.min.x >= *inner_circle_radius)
                    && (collider.max.x <= *outer_circle_radius
                        || -collider.max.x <= *outer_circle_radius)
                    && (collider.min.y >= *inner_circle_radius
                        || -collider.min.y >= *inner_circle_radius)
                    && (collider.max.y <= *outer_circle_radius
                        || -collider.max.y <= *outer_circle_radius)
                {
                    return true;
                }
            }
            false
        }
        PowerTypeEnum::Laser => {
            let cos_45 = 2.0_f32.sqrt() / 2.;
            let sin_45 = 2.0_f32.sqrt() / 2.;

            for laser in laser_query.iter() {
                let Laser {
                    center_position, ..
                } = laser;

                let mut laser_collider = Aabb2d::new(
                    Vec2::ZERO,
                    Vec2::new(LASER_POWER_WIDTH / 2., LASER_POWER_HEIGHT / 2.),
                );
                let rotation_matrix = Rot2::from_sin_cos(sin_45, cos_45);
                let translation_matrix = center_position.truncate();

                laser_collider.transform_by(translation_matrix, rotation_matrix);

                if laser_collider.intersects(&collider) {
                    return true;
                }
            }
            false
        }
    }
}
