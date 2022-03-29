use bevy::prelude::*;
use bevy::utils::Instant;
use bevy::ecs::schedule::SystemSet;

use bevy::core::FixedTimestep;





use crate::{player::{Player}, Health};
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


fn system_spawner_enemy(mut commands: Commands, time: Res<Time>, mut wave_state: ResMut<WaveState>, asset_server :Res<AssetServer> ){
    if time.last_update().is_some() && time.last_update().unwrap().duration_since(wave_state.last_spawn).as_secs_f32() < 0.2 {
        return;
    } 
    else if time.last_update().is_some() {
        wave_state.last_spawn = time.last_update().unwrap();
    }

    create_enemy(&mut commands,asset_server);
}


fn create_enemy(commands : &mut Commands,asset_server: Res<AssetServer>){
  

    commands.spawn_bundle(SpriteBundle{
        texture : asset_server.load("sh.png"),
        transform: Transform{
            translation : Vec3::new(0.0,0.0,0.5),
            scale : Vec3::new(0.1,0.1,1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Enemy)
    .insert(Health{maxHealth: 10,health: 10});
}


fn system_move_enemies(_time: Res<Time>, 
    mut query: QuerySet<(
        QueryState<(&mut Enemy, &mut Transform)>, 
        QueryState<&Transform, With<Player>>
    )>,) 
{
    let speed = 3.0;

    if let Ok(&player_transform) = query.q1().get_single() {
        
        for (_enemy, mut transform) in query.q0().iter_mut() {

            let movement = (player_transform.translation - transform.translation).normalize() * speed;
           
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
            //colliderPosition.translation.x = transform.translation.x;
            //colliderPosition.translation.y = transform.translation.y;
        }
    }

}