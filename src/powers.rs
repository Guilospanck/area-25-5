use std::f32::consts::PI;

use crate::{
    prelude::*,
    util::{
        get_key_code_based_on_power_type, get_power_sprite_based_on_power_type, get_random_vec3,
    },
    AnimationIndices, AnimationTimer, CleanupWhenPlayerDies, Damage, Direction, SpritesResources,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone))]
pub struct CircleOfDeath {
    pub inner_circle_radius: f32,
    pub outer_circle_radius: f32,
}

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone))]
pub struct Laser {
    max_bounces: u32,
    current_bounces: u32,
    pub center_position: Vec3,
}

#[cfg_attr(not(feature = "web"), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(feature = "web"), reflect(Component))]
#[cfg_attr(feature = "web", derive(Component, Debug, Clone))]
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
    asset_server: &Res<AssetServer>,

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
        power_type,
        stopping_condition,
        max_value,
    } = power;

    let keycode = get_key_code_based_on_power_type(power_type.clone());

    let power = Power {
        origin: Vec2::ZERO,
        power_type: power_type.clone(),
        stopping_condition: stopping_condition.clone(),
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
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    power: Power,
    power_damage: Damage,
    player_translation: Vec3,
) {
    let visibility = Visibility::Visible;

    let power_bundle = _get_power_bundle(
        texture_atlas_layout,
        sprites,
        &asset_server,
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
        PowerTypeEnum::CircleOfDeath => spawn_circle_of_death_power(
            commands,
            meshes,
            materials,
            power_bundle,
            quantity,
            player_translation,
        ),
        PowerTypeEnum::Laser => spawn_laser_power(
            commands,
            meshes,
            materials,
            power_bundle,
            quantity,
            player_translation,
        ),
    }
}

fn spawn_explosion_power(
    commands: &mut Commands,
    power_bundle: PowerBundle,
    max_value: u32,
    quantity: u32,
) {
    let base_camera_scale = Vec2::splat(BASE_CAMERA_PROJECTION_SCALE).extend(1.);

    for _ in 1..=quantity {
        for idx in 1..=max_value {
            let mut rng = rand::thread_rng();
            let n1: u8 = rng.gen();
            let random_spawning_pos =
                get_random_vec3(idx as u64, Some(n1 as u64 * POWER_RANDOM_SEED))
                    / base_camera_scale;

            let mut new_bundle = power_bundle.clone();
            new_bundle.sprite.transform.translation = random_spawning_pos;

            commands.spawn(new_bundle);
        }
    }
}

fn spawn_circle_of_death_power(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    power_bundle: PowerBundle,
    quantity: u32,
    player_translation: Vec3,
) {
    let circle = Mesh2dHandle(meshes.add(Annulus::new(40., 50.)));
    let color = Color::srgba(255., 0., 0., 0.8);

    let base_camera_scale = Vec2::splat(BASE_CAMERA_PROJECTION_SCALE).extend(1.);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(color),
            transform: Transform::from_translation(player_translation / base_camera_scale),
            ..default()
        },
        CircleOfDeath {
            inner_circle_radius: 40.,
            outer_circle_radius: 50.,
        },
        BASE_LAYER,
    ));

    for _ in 1..=quantity {
        let mut new_bundle = power_bundle.clone();
        new_bundle.sprite.visibility = Visibility::Hidden;
        commands.spawn(new_bundle);
    }
}

fn spawn_laser_power(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    power_bundle: PowerBundle,
    quantity: u32,
    player_translation: Vec3,
) {
    let rectangle = Mesh2dHandle(meshes.add(Rectangle::new(LASER_POWER_WIDTH, LASER_POWER_HEIGHT)));
    let color = Color::srgba(255., 0., 0., 0.8);

    let base_camera_scale = Vec2::splat(BASE_CAMERA_PROJECTION_SCALE).extend(1.);
    let translation = player_translation / base_camera_scale;
    let direction = Vec3::ONE;

    let max_bounces = power_bundle.marker.max_value;
    let current_bounces = 0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: rectangle,
            material: materials.add(color),
            transform: Transform {
                translation,
                scale: Vec3::ONE,
                rotation: Quat::from_rotation_z(PI / 4.),
            },
            ..default()
        },
        Laser {
            current_bounces,
            max_bounces,
            center_position: translation,
        },
        Direction(direction),
        BASE_LAYER,
    ));

    for _ in 1..=quantity {
        let mut new_bundle = power_bundle.clone();
        new_bundle.sprite.visibility = Visibility::Hidden;
        commands.spawn(new_bundle);
    }
}

pub fn move_laser_power(
    mut commands: Commands,
    mut laser_power_query: Query<(Entity, &mut Transform, &mut Direction, &mut Laser), With<Laser>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, mut laser_direction, mut laser) in &mut laser_power_query.iter_mut()
    {
        let mut new_translation_x = transform.translation.x
            + laser_direction.0.x * POWER_MOVE_SPEED * timer.delta_seconds();
        let mut new_translation_y = transform.translation.y
            + laser_direction.0.y * POWER_MOVE_SPEED * timer.delta_seconds();

        let off_screen_x =
            !(-WINDOW_RESOLUTION.x_px..=WINDOW_RESOLUTION.x_px).contains(&new_translation_x);
        let off_screen_y =
            !(-WINDOW_RESOLUTION.y_px..=WINDOW_RESOLUTION.y_px).contains(&new_translation_y);

        if off_screen_x {
            // invert direction
            *laser_direction = Direction(Vec3::new(
                (laser_direction.0.x) * -1.,
                laser_direction.0.y,
                laser_direction.0.z,
            ));
            new_translation_x = transform.translation.x
                + laser_direction.0.x * POWER_MOVE_SPEED * timer.delta_seconds();

            // rotate
            transform.rotation = transform.rotation.inverse();

            // check and increase bounces
            laser.current_bounces += 1;
            if laser.current_bounces > laser.max_bounces {
                commands.entity(entity).despawn();
            }
        }
        if off_screen_y {
            // invert direction
            *laser_direction = Direction(Vec3::new(
                laser_direction.0.x,
                (laser_direction.0.y) * -1.,
                laser_direction.0.z,
            ));
            new_translation_y = transform.translation.y
                + laser_direction.0.y * POWER_MOVE_SPEED * timer.delta_seconds();

            // rotate
            transform.rotation = transform.rotation.inverse();

            // check and increase bounces
            laser.current_bounces += 1;
            if laser.current_bounces > laser.max_bounces {
                commands.entity(entity).despawn();
            }
        }

        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // update Laser center position
        laser.center_position = transform.translation;
    }
}

fn _get_power_bundle(
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: &Res<SpritesResources>,
    asset_server: &Res<AssetServer>,

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
        asset_server,
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
