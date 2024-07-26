use crate::prelude::*;

#[derive(Resource)]
pub struct CurrentWave(pub u32);

#[derive(Resource)]
pub struct EnemyWaves(pub [EnemyByLevel; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct WeaponWaves(pub [WeaponByLevel; NUMBER_OF_WAVES]);

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(CurrentWave(1));
    commands.insert_resource(EnemyWaves(ENEMIES_PER_WAVE));
    commands.insert_resource(WeaponWaves(WEAPONS_PER_WAVE));
}
