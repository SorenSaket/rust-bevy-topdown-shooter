use bevy::prelude::*;
use bevy::utils::Instant;
use bevy::ecs::schedule::SystemSet;

use bevy::core::FixedTimestep;

use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub struct PluginBullet;


impl Plugin for PluginBullet {
    fn build(&self, app: &mut App){
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                .with_system(system_move_enemies)
        );
    }
}
