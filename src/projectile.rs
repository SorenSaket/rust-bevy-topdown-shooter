use bevy::{
    prelude::*,
    utils::Instant,
    ecs::schedule::SystemSet,
    core::FixedTimestep,
};


use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::Health;
use crate::enemy::Enemy;

pub struct PluginProjectile;

#[derive(Component, Clone)]
pub struct Projectile{
    pub speed: f32,
    pub dir: f32,
    pub lifetime: f32,
    pub damage: i32
}

impl Plugin for PluginProjectile {
    fn build(&self, app: &mut App){
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                .with_system(system_move_projectiles)
                .with_system(system_projectile_lifetime)
        )
        .add_system_to_stage(CoreStage::PostUpdate, system_handle_projectile_events);
    }
}


pub fn spawn_projectile(
    commands: &mut Commands, 
    position: Vec3, 
    dir: f32
    ){
    
    let shape = shapes::Circle{
        radius: 20.0,
        center: Vec2::ZERO,
    };


    
    
    commands.spawn()
    // Shape bundle
    .insert_bundle(
        GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined{
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::TEAL, 2.0),
            },
            Transform {
                translation: position,
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
        )
    )
    .insert(Projectile{speed: 50., dir: dir, lifetime: 3.0, damage: 4});


}

fn system_handle_projectile_events(

    query_projectile: Query<(Entity, &Projectile, &mut Transform), (Without<Enemy>)>,
    mut query_enemy: Query<(Entity, &mut Transform, &mut Health), (With<Enemy>, Without<Projectile>)>,
    mut commands: Commands,
) {

    for (entity_projectile, projectile, transform_projectile) in query_projectile.iter(){
        for (entity_enemy, transform_enemy, mut health) in query_enemy.iter_mut(){
            if(Vec2::distance(transform_projectile.translation.truncate(), transform_enemy.translation.truncate() )<40.0){
                commands.entity(entity_projectile).despawn();
                health.health -= projectile.damage;
                break;
            }
        }
    }
}

fn system_projectile_lifetime(time: Res<Time>, mut query: Query<(Entity,&mut Projectile)>, mut commands : Commands){
    for (entity , mut projectile) in query.iter_mut() {
        projectile.lifetime -= 1.0/60.0;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn system_move_projectiles(time: Res<Time>, 
    mut query: Query<(&Projectile, &mut Transform)>) 
{
    for (projectile, mut transform) in query.iter_mut() {

        transform.translation.x += projectile.dir.cos() * projectile.speed;
        transform.translation.y += projectile.dir.sin() * projectile.speed;
        //colliderPosition.translation.x = transform.translation.x;
        //colliderPosition.translation.y = transform.translation.y;
        
    }
}