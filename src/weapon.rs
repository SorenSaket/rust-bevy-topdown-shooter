use std::borrow::Borrow;

use bevy::{
    prelude::*, render::render_resource::*,
};
use rand::{thread_rng, Rng};

use crate::{projectile::Projectile, player::Player};

pub struct PluginWeapon;

#[derive(Component, Clone, PartialEq, Default)]
pub struct Weapon{
	pub texture: Handle<Image>,
	pub firerate: f32,
	pub projectilesPerShot: u32,
	pub firepointoffset: Vec2,
	pub spread: f32,
	pub inaccuracy: f32,
	pub owner: Option<Entity>
}


impl Plugin for PluginWeapon {
     fn build(&self, app: &mut App) {
       app
	   .add_startup_system(setup)
	   .add_system(system_weapon_pickup);
    }
}

fn setup(
    mut commands: Commands,
	mut asset_server: Res<AssetServer>,
){
	spawn_weapon(
		&mut commands, 
		Vec3::new(100.0,0.0,1.0), 
		&Weapon{
			texture: asset_server.load("gun.png"),
			firerate: 1.0,
			projectilesPerShot: 1,
			spread: 0.0,
			inaccuracy:  0.0,
			owner: None,
			..Default::default()
		}
	)
}

pub fn spawn_weapon(
    commands: &mut Commands, 
    position: Vec3,
	weapon: &Weapon
    ){
    
    commands.spawn()
	.insert_bundle(SpriteBundle {
        sprite: Sprite {
            flip_y: false,
            flip_x: false,
            ..Default::default()
        },
        
        texture: weapon.texture.clone(),
        
        transform: Transform {
            translation: position,
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(weapon.clone());
}


fn system_weapon_pickup(
	mut commands: Commands,
	mut query_weapon: Query<(Entity, &mut Weapon, &mut Transform), Without<Player>>,
	mut query_players: Query<(Entity, &mut Player, &Transform), Without<Weapon>>
){
	for (entity_weapon, mut weapon, mut transform_weapon) in query_weapon.iter_mut(){
		if weapon.owner.is_none() {

			// Get closest player
			for (entity_player, mut player, transform_player) in query_players.iter_mut() {
				if Vec2::distance(transform_weapon.translation.truncate(), transform_player.translation.truncate()) < 64.0 {
					
					
					commands.entity(entity_player).push_children(&[entity_weapon]);

					transform_weapon.translation = Vec3::ZERO;

					weapon.owner = Some(entity_player);

					player.weapon = Some(entity_weapon);
					
					break;
				}
			}

		}
	}

}
