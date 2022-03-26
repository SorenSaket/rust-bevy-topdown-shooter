use bevy::{
    prelude::*,
    utils::Instant,
    ecs::schedule::SystemSet,
    core::FixedTimestep,
};

use bevy_rapier2d::prelude::*;

use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub struct PluginProjectile;

#[derive(Component)]
pub struct Projectile{
    pub speed: f32,
    pub dir: f32,
    pub lifetime: f32,
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
    // Collider
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(shape.radius).into(),
        collider_type: ColliderType::Sensor.into(),
        flags:ColliderFlags{
            active_events: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(), 
            active_collision_types: (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC).into(), 
            ..Default::default() 
        }.into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        }.into(),
        ..ColliderBundle::default()
    }).insert_bundle( RigidBodyBundle {
        body_type: RigidBodyType::KinematicPositionBased.into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(Projectile{speed: 10., dir: dir, lifetime: 3.0});


}

fn system_handle_projectile_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
    query: Query<Entity, With<Projectile>>,
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>
) {

   
    for intersection_event in intersection_events.iter() {
        println!("intersection_event");
        commands.entity(intersection_event.collider1.entity()).despawn();
    }

    for contact_event in contact_events.iter() {
        println!("contact_event");
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
    mut query: Query<(&Projectile, &mut Transform, &mut RigidBodyPositionComponent)>) 
{
    for (projectile, mut transform, mut colliderPosition) in query.iter_mut() {

        colliderPosition.next_position.translation.x += projectile.dir.cos() * projectile.speed;
        colliderPosition.next_position.translation.y += projectile.dir.sin() * projectile.speed;
        //colliderPosition.translation.x = transform.translation.x;
        //colliderPosition.translation.y = transform.translation.y;
        
    }
}