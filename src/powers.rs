use crate::{
    prelude::*,
    util::{get_power_sprite_based_on_power_type, get_random_vec3},
    AnimationIndices, AnimationTimer, CleanupWhenPlayerDies, Damage, Direction, SpritesResources,
};
use rand::Rng;

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum StoppingCondition {
    #[default]
    Instances,
    // Limit,
    // ScreenBounces,
}

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum PowerTypeEnum {
    #[default]
    Explosions,
}

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub struct Power {
    pub origin: Vec2,
    pub power_type: PowerTypeEnum,
    pub stopping_condition: StoppingCondition,
    pub value: f32,
    pub max_value: f32,
    pub mana_needed: f32,
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
        value: f32,
        max_value: f32,
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
        value: f32,
        max_value: f32,
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
            mana_needed: 50.,
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

pub fn spawn_power(
    commands: &mut Commands,
    // weapon_by_level: &WeaponByLevel,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    let damage = 10.;
    let scale = Vec3::ONE;
    let direction = Vec3::ZERO;
    let rotation = Quat::default();
    let layer = BASE_LAYER;
    let power_type = PowerTypeEnum::Explosions;
    let stopping_condition = StoppingCondition::Instances;
    let value = 5.0;
    let max_value = 5.0;

    for idx in 1..=value as usize {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let random_spawning_pos = get_random_vec3(idx as u64, Some(n1 as u64 * WEAPON_RANDOM_SEED));

        let bundle = PowerBundle::new(
            &mut texture_atlas_layout,
            sprites,
            &asset_server,
            scale,
            random_spawning_pos,
            direction,
            damage,
            rotation,
            layer.clone(),
            power_type.clone(),
            stopping_condition.clone(),
            value,
            max_value,
        );
        commands.spawn(bundle);
    }
}
