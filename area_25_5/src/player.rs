use crate::{
    ammo::Ammo, animation::*, prelude::*, sprites::Sprites, weapon::Weapon, AmmoBundle, Armor,
    Damage, Health, Speed, WeaponBundle,
};

#[derive(Component, Debug, Clone)]
pub struct Player;

#[derive(Bundle, Clone)]
pub(crate) struct PlayerBundle {
    pub(crate) marker: Player,

    pub(crate) health: Health,
    pub(crate) armor: Armor,
    pub(crate) speed: Speed,

    pub(crate) sprite: SpriteBundle,
    pub(crate) atlas: TextureAtlas,
    pub(crate) animation_indices: AnimationIndices,
    pub(crate) animation_timer: AnimationTimer,
    pub(crate) layer: RenderLayers,
    name: Name,
}

impl PlayerBundle {
    pub(crate) fn idle(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        Self::_util(texture_atlas_layout, sprites, asset_server)
    }

    fn _util(
        texture_atlas_layout: &mut ResMut<Assets<TextureAtlasLayout>>,
        sprites: &Sprites<'static>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let player_sprite = sprites.player_char_idle.clone();

        let player_animation = player_sprite.animation.unwrap();
        let handle_texture_atlas_layout = texture_atlas_layout.add(player_sprite.layout);

        let pos: Vec3 = Vec3::new(
            -WINDOW_RESOLUTION.x_px / 2. + 50.,
            WINDOW_RESOLUTION.y_px / 2. - 80.,
            CHAR_Z_INDEX,
        );

        PlayerBundle {
            name: Name::new("Player"),
            marker: Player,
            health: Health(PLAYER_HEALTH),
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
                layout: handle_texture_atlas_layout,
                index: player_animation.indices.first,
            },
            animation_indices: player_animation.indices,
            animation_timer: player_animation.timer,
            layer: GAME_LAYER,
        }
    }
}
