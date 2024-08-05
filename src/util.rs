use crate::{prelude::*, SpriteInfo, SpritesResources};
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
        (rng.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.x_px,
        (rng.gen::<f32>() - 0.5) * WINDOW_RESOLUTION.y_px,
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
    item_type: ItemStatsType,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match item_type {
        ItemStatsType::Speed => sprites.0.speed_potion.clone(),
        ItemStatsType::Armor => sprites.0.shield.clone(),
    }
}
