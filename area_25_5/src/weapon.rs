use crate::prelude::*;
use crate::AnimationIndices;
use crate::AnimationTimer;
use crate::SpriteInfo;
use crate::Sprites;
use crate::SpritesResources;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Reflect, Component, Default, Debug, Clone, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Ammo {
    pub direction: Vec2,
    pub source: String,
    pub damage: f32,
}

use bevy_inspector_egui::prelude::*;
#[derive(Reflect, Component, Default, Debug, Clone, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Weapon {
    pub pos: Vec3,
    pub ammo: Ammo,
    pub source: String,
}

impl Weapon {
    fn random(rand: &mut ChaCha8Rng, ammo: Ammo, source: String) -> Self {
        Weapon {
            pos: Vec3::new(
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.x_px - 100.0),
                (rand.gen::<f32>() - 0.5) * (WINDOW_RESOLUTION.y_px - 100.0),
                CHAR_Z_INDEX,
            ),
            ammo,
            source,
        }
    }
}

#[derive(Bundle, Clone)]
pub(crate) struct WeaponBundle {
    pub(crate) marker: Weapon,
    pub(crate) weapon_sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    name: Name,
}

impl WeaponBundle {
    pub(crate) fn new(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
        weapon: Weapon,
        scale: Vec3,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.bow.clone(),
            asset_server,
            weapon,
            scale,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        weapon_sprite: SpriteInfo<'static>,
        asset_server: &Res<AssetServer>,
        weapon: Weapon,
        scale: Vec3,
    ) -> Self {
        let weapon_animation = weapon_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(weapon_sprite.layout);

        WeaponBundle {
            name: Name::new("Weapon"),
            marker: weapon.clone(),
            weapon_sprite: SpriteBundle {
                texture: asset_server.load(weapon_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: weapon.pos,
                    scale,
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: weapon_animation.indices.first,
            },
            animation_indices: weapon_animation.indices,
            animation_timer: weapon_animation.timer,
            layer: GAME_LAYER,
        }
    }
}

pub fn spawn_weapon(
    commands: &mut Commands,
    weapon_by_level: &WeaponByLevel,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    sprites: Res<SpritesResources>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    let mut rng = ChaCha8Rng::seed_from_u64((y * 19838367467713.) as u64);

    let weapon_source = weapon_by_level.weapon.source;
    let ammo_source = weapon_by_level.weapon.ammo_source;
    let ammo_damage = weapon_by_level.weapon.damage;

    let ammo = Ammo {
        source: ammo_source.to_string(),
        direction: Vec2::splat(0.0),
        damage: ammo_damage,
    };

    for _ in 1..=weapon_by_level.quantity {
        let weapon = Weapon::random(&mut rng, ammo.clone(), weapon_source.to_string());
        let bundle = WeaponBundle::new(
            &mut texture_atlas_layout,
            &sprites.0,
            &asset_server,
            weapon,
            Vec3::ONE,
        );

        commands.spawn(bundle);
    }
}

pub fn move_ammo(
    mut commands: Commands,
    mut ammos: Query<(Entity, &mut Transform, &mut Ammo), Without<Weapon>>,
    timer: Res<Time>,
) {
    for (entity, mut transform, ammo) in &mut ammos {
        let new_translation_x =
            transform.translation.x + ammo.direction.x * AMMO_MOVE_SPEED * timer.delta_seconds();
        let new_translation_y =
            transform.translation.y - ammo.direction.y * AMMO_MOVE_SPEED * timer.delta_seconds();

        let off_screen_x = !(-WINDOW_RESOLUTION.x_px / 2.0..=WINDOW_RESOLUTION.x_px / 2.0)
            .contains(&new_translation_x);
        let off_screen_y = !(-WINDOW_RESOLUTION.y_px / 2.0..=WINDOW_RESOLUTION.y_px / 2.0)
            .contains(&new_translation_y);

        if off_screen_x || off_screen_y {
            commands.entity(entity).despawn();
            return;
        }

        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;
    }
}
