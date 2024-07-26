use crate::{
    animation::*,
    prelude::*,
    sprites::{SpriteInfo, Sprites},
    weapon::{Ammo, Weapon},
};
use bevy::sprite::Mesh2dHandle;

#[derive(Component, Debug, Clone)]
pub struct Alien {
    pub weapon: Weapon,
    pub health: f32,
    pub speed: f32,
    pub armor: f32,
}

#[derive(Bundle, Clone)]
pub(crate) struct AlienBundle {
    pub(crate) marker: Alien,
    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
}

impl AlienBundle {
    pub(crate) fn idle(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        meshes: ResMut<Assets<Mesh>>,
        sprites: &Sprites,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            meshes,
            sprites.alien_char_idle.clone(),
        )
    }

    pub(crate) fn walking(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        meshes: ResMut<Assets<Mesh>>,
        sprites: &Sprites,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            meshes,
            sprites.alien_char_walking.clone(),
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        mut meshes: ResMut<Assets<Mesh>>,
        alien_sprite: SpriteInfo,
    ) -> Self {
        let alien_animation = alien_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(alien_sprite.layout);

        let mesh = Mesh2dHandle(meshes.add(Capsule2d::new(CAPSULE_RADIUS, CAPSULE_LENGTH)));
        let color = Color::BLACK;
        let ammo = Ammo {
            mesh,
            color,
            direction: Vec2::splat(0.0),
            damage: AMMO_DAMAGE,
        };
        let pos: Vec3 = Vec3::new(
            -WINDOW_RESOLUTION.x_px / 2. + 50.,
            WINDOW_RESOLUTION.y_px / 2. - 80.,
            CHAR_Z_INDEX,
        );

        AlienBundle {
            marker: Alien {
                health: ALIEN_HEALTH,
                weapon: Weapon { ammo, pos },
                speed: ALIEN_MOVE_SPEED,
                armor: ALIEN_ARMOR,
            },
            sprite: SpriteBundle {
                texture: alien_sprite.source.clone(),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale: Vec3::new(2., 2., 1.),
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: alien_animation.indices.first,
            },
            animation_indices: alien_animation.indices,
            animation_timer: alien_animation.timer,
            layer: GAME_LAYER,
        }
    }
}
