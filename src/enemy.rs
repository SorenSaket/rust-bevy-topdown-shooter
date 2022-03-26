use bevy::prelude::*;
use bevy::utils::Instant;
use bevy::ecs::schedule::SystemSet;

use bevy::core::FixedTimestep;

use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::player::{PluginPlayer, Player};
pub struct PluginEnemy;


impl Plugin for PluginEnemy {
    fn build(&self, app: &mut App){
        app
        .insert_resource(WaveState {
            last_spawn: Instant::now(),
        })
        .add_system(system_spawner_enemy)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                .with_system(system_move_enemies)
        );
    }
}

struct WaveState {
    pub last_spawn: Instant,
}

#[derive(Clone, Component)]
pub struct Enemy;


fn system_spawner_enemy(mut commands: Commands, time: Res<Time>, mut wave_state: ResMut<WaveState>){
    if time.last_update().is_some() && time.last_update().unwrap().duration_since(wave_state.last_spawn).as_secs_f32() < 1. {
        return;
    } 
    else if time.last_update().is_some() {
        wave_state.last_spawn = time.last_update().unwrap();
    }

    create_enemy(&mut commands);
}


fn create_enemy(commands : &mut Commands){
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(32.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 4.0),
        },
        Transform::default()
    ))
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(32.).into(),
        collider_type: ColliderType::Solid.into(),
        flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        }.into(),
        ..ColliderBundle::default()
    }).insert_bundle( RigidBodyBundle {
        body_type: RigidBodyType::KinematicPositionBased.into(),
        ..Default::default()
    }


    )
    
    .insert(ColliderPositionSync::Discrete)
    .insert(Enemy);
}


fn system_move_enemies(time: Res<Time>, 
    mut query: QuerySet<(
        QueryState<(&mut Enemy, &mut Transform, &mut RigidBodyPositionComponent)>, 
        QueryState<(&Transform), With<Player>>
    )>,) 
{
    let speed = 3.0;



    
    if let Ok(&player_transform) = query.q1().get_single() {
        
        for (mut enemy, mut transform, mut bodyPosition) in query.q0().iter_mut() {

            let movement = (player_transform.translation - transform.translation).normalize() * speed;
           
            bodyPosition.next_position.translation.x += movement.x;
            bodyPosition.next_position.translation.y += movement.y;
            //colliderPosition.translation.x = transform.translation.x;
            //colliderPosition.translation.y = transform.translation.y;
        }
    }

}