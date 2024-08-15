use crate::{
    prelude::*,
    util::{
        get_key_code_based_on_power_type, get_power_sprite_based_on_power_type, get_random_vec3,
    },
    AnimationIndices, AnimationTimer, CleanupWhenPlayerDies, Damage, Direction, SpritesResources,
};
use bevy::math::VectorSpace;
use rand::Rng;

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub struct Power {
    // TODO: maybe remove this
    pub origin: Vec2,
    pub power_type: PowerTypeEnum,
    pub stopping_condition: StoppingCondition,
    pub value: u32,
    pub max_value: u32,
    pub mana_needed: f32,
    pub trigger_key: KeyCode,
    // How many of them should be spawned
    pub quantity: u32,
}

#[derive(Bundle, Clone)]
pub(crate) struct PowerBundle {
    pub(crate) marker: Power,
    pub(crate) direction: Direction,
    pub(crate) damage: Damage,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    pub(crate) cleanup: CleanupWhenPlayerDies,
    name: Name,
}

impl PowerBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        direction: Vec3,
        damage: f32,
        rotation: Quat,
        layer: RenderLayers,
        power_type: PowerTypeEnum,
        stopping_condition: StoppingCondition,
        value: u32,
        max_value: u32,
        mana_needed: f32,
        trigger_key: KeyCode,
        visibility: Visibility,
        quantity: u32,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites,
            asset_server,
            scale,
            pos,
            direction,
            damage,
            rotation,
            layer,
            power_type,
            stopping_condition,
            value,
            max_value,
            mana_needed,
            trigger_key,
            visibility,
            quantity,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Res<SpritesResources>,
        asset_server: &Res<AssetServer>,
        scale: Vec3,
        pos: Vec3,
        direction: Vec3,
        damage: f32,
        rotation: Quat,
        layer: RenderLayers,
        power_type: PowerTypeEnum,
        stopping_condition: StoppingCondition,
        value: u32,
        max_value: u32,
        mana_needed: f32,
        trigger_key: KeyCode,
        visibility: Visibility,
        quantity: u32,
    ) -> Self {
        let power_sprite = get_power_sprite_based_on_power_type(power_type.clone(), sprites);
        let power_animation = power_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(power_sprite.layout);

        let marker = Power {
            power_type,
            origin: pos.truncate(),
            stopping_condition,
            value,
            max_value,
            mana_needed,
            trigger_key,
            quantity,
        };

        PowerBundle {
            name: Name::new("Power"),
            marker,
            direction: Direction(direction),
            damage: Damage(damage),
            sprite: SpriteBundle {
                texture: asset_server.load(power_sprite.source),
                transform: Transform {
                    rotation,
                    translation: pos,
                    scale,
                },
                visibility,
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: power_animation.indices.first,
            },
            animation_indices: power_animation.indices,
            animation_timer: power_animation.timer,
            layer,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn equip_player_with_power(
    commands: &mut Commands,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    power_by_level: &PowerByLevel,
    player_entity: Entity,
) {
    let visibility = Visibility::Hidden;

    let PowerByLevel {
        level: _,
        power,
        quantity,
    } = power_by_level;
    let PowerType {
        damage,
        mana_needed,
        source: _,
        power_type,
        stopping_condition,
        max_value,
    } = power;

    let keycode = get_key_code_based_on_power_type(power_type.clone());

    let power = Power {
        origin: Vec2::ZERO,
        power_type: power_type.clone(),
        stopping_condition: stopping_condition.clone(),
        // TODO: change this value depending on the Power type
        value: *max_value,
        max_value: *max_value,
        mana_needed: *mana_needed,
        trigger_key: keycode,
        quantity: *quantity,
    };

    let power_bundle = _get_power_bundle(
        texture_atlas_layout,
        sprites,
        asset_server,
        power,
        *damage,
        visibility,
    );

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn(power_bundle);
    });
}

pub fn spawn_power(
    commands: &mut Commands,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,
    power: Power,
    power_damage: Damage,
) {
    let visibility = Visibility::Visible;

    let power_bundle = _get_power_bundle(
        texture_atlas_layout,
        sprites,
        asset_server,
        power.clone(),
        power_damage.0,
        visibility,
    );

    let power_type = power.power_type;
    let max_value = power.max_value;
    let quantity = power.quantity;

    match power_type {
        PowerTypeEnum::Explosions => {
            spawn_explosion_power(commands, power_bundle, max_value, quantity)
        }
    }
}

fn spawn_explosion_power(
    commands: &mut Commands,
    power_bundle: PowerBundle,
    max_value: u32,
    quantity: u32,
) {
    for _ in 1..=quantity {
        for idx in 1..=max_value {
            let mut rng = rand::thread_rng();
            let n1: u8 = rng.gen();
            let random_spawning_pos =
                get_random_vec3(idx as u64, Some(n1 as u64 * ITEM_RANDOM_SEED));

            let mut new_bundle = power_bundle.clone();
            new_bundle.sprite.transform.translation = random_spawning_pos;

            commands.spawn(new_bundle);
        }
    }
}

fn _get_power_bundle(
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,

    power: Power,
    damage: f32,
    visibility: Visibility,
) -> PowerBundle {
    let Power {
        origin: _,
        power_type,
        stopping_condition,
        value: _,
        max_value,
        mana_needed,
        trigger_key: _,
        quantity,
    } = power;

    let scale = Vec3::ONE;
    let direction = Vec3::ZERO;
    let rotation = Quat::default();
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let layer = BASE_LAYER;

    // TODO: these will change based on the power type
    // Right now they're being set bringing into consideration
    // that they are for Explosions
    let value = max_value;

    let keycode = get_key_code_based_on_power_type(power_type.clone());

    PowerBundle::new(
        &mut texture_atlas_layout,
        sprites,
        &asset_server,
        scale,
        pos,
        direction,
        damage,
        rotation,
        layer,
        power_type.clone(),
        stopping_condition.clone(),
        value,
        max_value,
        mana_needed,
        keycode,
        visibility,
        quantity,
    )
}
