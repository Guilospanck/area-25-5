use crate::{prelude::*, ItemTypeEnum, PowerTypeEnum, SpriteInfo, SpritesResources};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub(crate) fn get_random_chance() -> f32 {
    let mut rand_thread_rng = rand::thread_rng();
    let n1: u8 = rand_thread_rng.gen();

    const RANDOM_SEED: u64 = 1282831746771;
    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED * n1 as u64);
    rng.gen::<f32>()
}

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
        PowerTypeEnum::Explosions => sprites.0.mine_bomb.clone(),
        PowerTypeEnum::CircleOfDeath => sprites.0.circle_of_death.clone(),
        PowerTypeEnum::Laser => sprites.0.laser.clone(),
    }
}

pub(crate) fn get_key_code_based_on_power_type(power_type: PowerTypeEnum) -> KeyCode {
    match power_type {
        PowerTypeEnum::Explosions => KeyCode::KeyL,
        PowerTypeEnum::CircleOfDeath => KeyCode::KeyJ,
        PowerTypeEnum::Laser => KeyCode::KeyH,
    }
}

pub(crate) fn get_enemy_sprite_based_on_enemy_class(
    enemy_class: EnemyClassEnum,
    sprites: &Res<SpritesResources>,
) -> SpriteInfo<'static> {
    match enemy_class {
        EnemyClassEnum::Orc => sprites.0.orc_idle.clone(),
        EnemyClassEnum::Mage => sprites.0.mage_idle.clone(),
        EnemyClassEnum::BossOrc => sprites.0.orc_idle.clone(),
    }
}
