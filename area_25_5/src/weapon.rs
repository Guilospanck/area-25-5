use crate::player::Alien;
use crate::prelude::*;
use bevy::sprite::Mesh2dHandle;

#[derive(Component, Debug, Clone)]
pub struct Ammo {
    pub direction: Vec2,
    pub mesh: Mesh2dHandle,
    pub color: Color,
    pub damage: f32,
}

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub ammo: Ammo,
}

pub fn move_ammo(
    mut commands: Commands,
    mut ammos: Query<(Entity, &mut Transform, &mut Ammo), Without<Alien>>,
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
