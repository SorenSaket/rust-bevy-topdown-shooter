use bevy::{
    prelude::*,
    ecs::schedule::SystemSet,
    core::FixedTimestep,
};


use bevy_rapier2d::prelude::*;

use bevy_prototype_lyon::prelude::*;

use crate::Health;
use crate::enemy::Enemy;
use crate::weapon::WeaponData;

pub struct PluginProjectile;


pub struct ProjectileData{
    pub projectiles: Vec<ProjectileSettings>
}


#[derive(Default)]
pub struct ProjectileSettings{
    pub texture: Handle<Image>,
    pub speed: f32,
    pub lifetime: f32,
    pub bounces: u32,
    pub damage: i32
}

#[derive(Component, Default)]
pub struct Projectile{
    pub settings: usize,
    pub speed: f32,
    pub dir: f32,
    pub lifetime: f32,
    pub bounces: u32,
    pub damage: i32,
    pub owner: Option<Entity>
}

impl Plugin for PluginProjectile {
    fn build(&self, app: &mut App){
        app
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                //.with_system(system_move_projectiles)
                .with_system(system_projectile_lifetime)
        )
        .add_system_to_stage(CoreStage::PostUpdate, system_handle_projectile_events)
        .add_system(display_events);
    }
}



pub fn spawn_projectile(
    commands: &mut Commands,
    owner: Option<Entity>,
    settings: &ProjectileSettings,
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
    .insert(
        Projectile{
            owner: owner , 
            speed: settings.speed, 
            dir: dir, 
            lifetime: settings.lifetime,
            damage: settings.damage,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(20.0).into(),
            collider_type: ColliderType::Sensor.into(),
            flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            }.into(),
            ..ColliderBundle::default()
        })
        .insert_bundle( RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: position.truncate().into(),
            velocity: RigidBodyVelocity { 
                linvel: Vec2::new(dir.cos() * settings.speed, dir.sin() * settings.speed).into(), 
                angvel: 0.0
            }.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(2))
        ;


}

fn setup(
    _commands: Commands,
	_asset_server: Res<AssetServer>
){
}

fn system_handle_projectile_events(

    query_projectile: Query<(Entity, &Projectile, &mut Transform), Without<Enemy>>,
    mut query_enemy: Query<(Entity, &mut Transform, &mut Health), (With<Enemy>, Without<Projectile>)>,
    mut commands: Commands,
    _weapons: Res<WeaponData>
) {

    for (entity_projectile, projectile, transform_projectile) in query_projectile.iter(){
        for (_entity_enemy, transform_enemy, mut health) in query_enemy.iter_mut(){
            if Vec2::distance(transform_projectile.translation.truncate(), transform_enemy.translation.truncate() )<40.0 {
                commands.entity(entity_projectile).despawn();
                health.health -= projectile.damage;
                break;
            }
        }
    }
}

fn system_projectile_lifetime(_time: Res<Time>, mut query: Query<(Entity,&mut Projectile)>, mut commands : Commands){
    for (entity , mut projectile) in query.iter_mut() {
        projectile.lifetime -= 1.0/60.0;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn system_move_projectiles(_time: Res<Time>, 
    mut query: Query<(&Projectile, &mut Transform, &mut RigidBodyPositionComponent)>) 
{
    for (projectile, mut transform, mut pos) in query.iter_mut() {

        pos.next_position.translation.x += projectile.dir.cos() * projectile.speed;
        pos.next_position.translation.y += projectile.dir.sin() * projectile.speed;
        //colliderPosition.translation.x = transform.translation.x;
        //colliderPosition.translation.y = transform.translation.y;
        
    }
}

fn display_events(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    mut query_projectile: Query<(Entity, &Projectile)>
) {
    for intersection_event in intersection_events.iter() {
        if let Ok((selected_character)) = query_projectile.get(intersection_event.collider1.entity()) {
            commands.entity(selected_character.0).despawn();
        }
        if let Ok(selected_character) = query_projectile.get(intersection_event.collider2.entity()) {
            commands.entity(selected_character.0).despawn();
        }
    }
}