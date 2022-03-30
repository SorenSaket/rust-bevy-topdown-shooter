

use bevy::{
	prelude::*,
};

use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::*;

pub fn spawn_wall(
	mut commands : &mut Commands, 
	asset_server: &Res<AssetServer>,
	position: Vec3
){
	let entity_builder = commands
	.spawn()
	/* .insert_bundle(
		SpriteBundle{	
			texture: asset_server.load("wall.png"),
			..Default::default()
		}
	)*/.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(32.0,32.0).into(),
        collider_type: ColliderType::Solid.into(),
        flags: (ActiveEvents::INTERSECTION_EVENTS | ActiveEvents::CONTACT_EVENTS).into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        }.into(),
		position: position.truncate().into(),
        ..ColliderBundle::default()
    })
	.insert(ColliderPositionSync::Discrete)
	.insert(ColliderDebugRender::with_id(1))
	;
}