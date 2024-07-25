use crate::prelude::*;

#[derive(Resource)]
pub struct CurrentWave(pub u32);

#[derive(Resource)]
pub struct Waves {
    pub info: [u32; NUMBER_OF_WAVES],
}

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(CurrentWave(1));
    commands.insert_resource(Waves { info: WAVES });
}
