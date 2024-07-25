use crate::{game_actions::shoot, player::Alien, prelude::*, ui::AlienHealthBar};

#[derive(Event)]
pub struct ShootBullets {
    pub pos: Vec2,
}

#[derive(Event)]
pub struct AlienHealthChanged {
    pub health: f32,
}

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
