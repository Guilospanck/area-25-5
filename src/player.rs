use crate::{
    animation::*, prelude::*, spawn_health_bar, spawn_player_camera, sprites::Sprites, AmmoBundle,
    Armor, CleanupWhenPlayerDies, Health, PlayerCamera, PlayerSpawned, Speed, SpritesResources,
    WeaponBundle,
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

    pub(crate) cleanup: CleanupWhenPlayerDies,
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

        let pos: Vec3 = Vec3::new(0.0, 0.0, CHAR_Z_INDEX);

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
            layer: PLAYER_LAYER,
            cleanup: CleanupWhenPlayerDies,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpritesResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_player(
        &mut commands,
        texture_atlas_layout,
        asset_server,
        sprites,
        &mut meshes,
        &mut materials,
    );
}

pub(crate) fn spawn_player(
    commands: &mut Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    sprites_resources: Res<SpritesResources>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let player = PlayerBundle::idle(
        &mut texture_atlas_layout,
        &sprites_resources.0,
        &asset_server,
    );

    let damage = AMMO_DAMAGE;
    let direction = Vec3::ZERO;
    let pos = Vec3::new(8.0, 0.0, CHAR_Z_INDEX);
    let weapon_scale = Vec3::new(0.5, 0.5, 1.);
    let weapon_type = WeaponTypeEnum::Bow;

    let weapon_bundle = WeaponBundle::new(
        &mut texture_atlas_layout,
        &sprites_resources,
        &asset_server,
        weapon_scale,
        pos,
        direction,
        damage,
        weapon_type,
    );

    let scale = Vec3::ONE;
    let weapon_type = WeaponTypeEnum::default();
    let rotation = Quat::default();

    let ammo_bundle = AmmoBundle::new(
        &mut texture_atlas_layout,
        &sprites_resources,
        &asset_server,
        scale,
        pos,
        weapon_type,
        direction,
        damage,
        rotation,
    );

    let player_entity_id = commands.spawn(player).id();

    let health_bar_translation = Vec3::new(2.0, 12.0, 0.0);
    let health_bar = spawn_health_bar(
        commands,
        meshes,
        materials,
        PLAYER_HEALTH,
        PLAYER_HEALTH,
        health_bar_translation,
    );

    commands
        .entity(player_entity_id)
        .with_children(|parent| {
            parent.spawn(weapon_bundle).with_children(|parent| {
                parent.spawn(ammo_bundle);
            });
        })
        .push_children(&[health_bar]);

    commands.trigger(PlayerSpawned);
}
