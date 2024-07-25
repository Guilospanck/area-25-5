use crate::{
    game_actions::shoot, player::Alien, prelude::*, spawn_item, ui::AlienHealthBar, AlienSpeedBar,
};

#[derive(Event)]
pub struct ShootBullets {
    pub pos: Vec2,
}

#[derive(Event)]
pub struct AlienHealthChanged {
    pub health: f32,
}

#[derive(Event)]
pub struct AlienSpeedChanged {
    pub speed: f32,
}

#[derive(Event)]
pub struct AlienSpawned;

pub fn on_mouse_click(
    trigger: Trigger<ShootBullets>,
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    alien: Query<(&Transform, &Alien)>,
) {
    let event = trigger.event();
    let Vec2 { x, y } = event.pos;

    shoot(commands, materials, x, y, alien);
}

pub fn on_alien_health_changed(
    trigger: Trigger<AlienHealthChanged>,
    mut alien_health_bar: Query<&mut Text, With<AlienHealthBar>>,
) {
    let event = trigger.event();
    let health = event.health;

    if let Ok(mut text) = alien_health_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = health.to_string();
    }
}

pub fn on_alien_speed_changed(
    trigger: Trigger<AlienSpeedChanged>,
    mut alien_speed_bar: Query<&mut Text, With<AlienSpeedBar>>,
) {
    let event = trigger.event();
    let speed = event.speed;

    if let Ok(mut text) = alien_speed_bar.get_single_mut() {
        text.sections.first_mut().unwrap().value = speed.to_string();
    }
}

pub fn on_alien_spawned(
    _trigger: Trigger<AlienSpawned>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_item(
        commands,
        meshes,
        materials,
        crate::ItemStatsType::Speed,
        ITEM_SPEED_VALUE,
    );
}
