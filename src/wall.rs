

use bevy::{
	prelude::*,
};

pub fn spawn_wall(
	mut commands : &mut Commands, 
	asset_server: &Res<AssetServer>,
	position: Vec3
){
	let entity_builder = commands
	.spawn()
	.insert_bundle(
		SpriteBundle{	
			texture: asset_server.load("wall.png"),
			transform: Transform{
				translation: position,
				..Default::default()
			},
			..Default::default()
		}
	);
}