use crate::{
    animation::*,
    prelude::*,
    sprites::{SpriteInfo, Sprites},
    weapon::{Ammo, Weapon},
    Armor, Health, Speed,
};

#[derive(Component, Debug, Clone)]
pub struct Player;

#[derive(Bundle, Clone)]
pub(crate) struct PlayerBundle {
    pub(crate) marker: Player,

    pub(crate) weapon: Weapon,
    pub(crate) health: Health,
    pub(crate) armor: Armor,
    pub(crate) speed: Speed,

    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
}

impl PlayerBundle {
    pub(crate) fn idle(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.player_char_idle.clone(),
            sprites.bow.clone(),
            sprites.arrow.clone(),
            asset_server,
        )
    }

    pub(crate) fn walking(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        Self::_util(
            texture_atlas_layout,
            sprites.player_char_walking.clone(),
            sprites.bow.clone(),
            sprites.arrow.clone(),
            asset_server,
        )
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        player_sprite: SpriteInfo<'static>,
        weapon_sprite: SpriteInfo<'static>,
        ammo_sprite: SpriteInfo<'static>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let player_animation = player_sprite.animation.unwrap();
        let texture_atlas_layout = texture_atlas_layout.add(player_sprite.layout);

        let ammo = Ammo {
            source: ammo_sprite.source.to_string(),
            direction: Vec2::splat(0.0),
            damage: AMMO_DAMAGE,
        };
        let pos: Vec3 = Vec3::new(
            -WINDOW_RESOLUTION.x_px / 2. + 50.,
            WINDOW_RESOLUTION.y_px / 2. - 80.,
            CHAR_Z_INDEX,
        );

        PlayerBundle {
            marker: Player,
            health: Health(PLAYER_HEALTH),
            weapon: Weapon {
                ammo,
                pos,
                source: weapon_sprite.source.to_string(),
            },
            speed: Speed(PLAYER_MOVE_SPEED),
            armor: Armor(PLAYER_ARMOR),
            sprite: SpriteBundle {
                texture: asset_server.load(player_sprite.source),
                transform: Transform {
                    rotation: Quat::default(),
                    translation: pos,
                    scale: Vec3::new(2., 2., 1.),
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.indices.first,
            },
            animation_indices: player_animation.indices,
            animation_timer: player_animation.timer,
            layer: GAME_LAYER,
        }
    }
}
