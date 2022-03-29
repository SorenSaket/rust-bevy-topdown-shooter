use std::borrow::Borrow;

use bevy::{
    prelude::*, render::render_resource::*,
};
use rand::{thread_rng, Rng};

use crate::{projectile::{Projectile, ProjectileSettings}, player::Player};

pub struct PluginWeapon;

pub struct WeaponData{
	pub weapons : Vec<WeaponSettings>
}


#[derive(Default)]
pub struct WeaponSettings{
	pub texture: Handle<Image>,
	pub projectile: ProjectileSettings,
	pub firerate: f32,
	pub projectilesPerShot: u32,
	pub firepointoffset: Vec2,
	pub spread: f32,
	pub inaccuracy: f32,
}

#[derive(Component )]
pub struct Weapon{
	pub settings: usize,
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
	mut asset_server: Res<AssetServer>
){
	let mut weapondata = WeaponData {weapons : Vec::new() };

	weapondata.weapons.push( WeaponSettings{
		texture: asset_server.load("gun.png"),
		firerate: 1.0,
		projectilesPerShot: 1,
		spread: 0.0,
		inaccuracy:  0.0,
		projectile: ProjectileSettings{
			texture: asset_server.load("sword.png"),
			speed: 16.0,
			lifetime: 2.0,
			bounces: 0,
			damage: 1,
		},
		..Default::default()
	});
	spawn_weapon(
		&mut commands, 
		Vec3::new(100.0,0.0,1.0), 
		&weapondata.weapons[0],
		0
	);
	
	spawn_weapon(
		&mut commands, 
		Vec3::new(-100.0,0.0,1.0),
		&weapondata.weapons[0],
		0
	);

	commands.insert_resource(weapondata);


}

pub fn spawn_weapon(
    commands: &mut Commands, 
    position: Vec3,
	weaponsettings: &WeaponSettings,
	weaponIndex: usize
    ){
    
    commands.spawn()
	.insert_bundle(SpriteBundle {
        sprite: Sprite {
            flip_y: false,
            flip_x: false,
            ..Default::default()
        },
        
        texture: weaponsettings.texture.clone(),
        
        transform: Transform {
            translation: position,
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Weapon{settings: weaponIndex, owner: None });
}


fn system_weapon_pickup(
	mut commands: Commands,
	mut query_weapon: Query<(Entity, &mut Weapon, &mut Transform), Without<Player>>,
	mut query_players: Query<(Entity, &mut Player, &Transform), Without<Weapon>>,
	buttons: Res<Input<GamepadButton>>,
){
	
	// Get closest player
	for (entity_player, mut player, transform_player) in query_players.iter_mut() {
		let btn_north = GamepadButton(player.gamepad, GamepadButtonType::North);

		if(buttons.just_pressed(btn_north)){
			for (entity_weapon, mut weapon, mut transform_weapon) in query_weapon.iter_mut(){
				
				if weapon.owner.is_none() {
					if Vec2::distance(transform_weapon.translation.truncate(), transform_player.translation.truncate()) < 128.0 {
			
						commands.entity(entity_player).push_children(&[entity_weapon]);
			
						transform_weapon.translation = Vec3::new(0.0, 0.0, transform_weapon.translation.z);
			
						weapon.owner = Some(entity_player);
			
						player.weapon = Some(entity_weapon);
						
						break;
					}
				}
			}
		}
	}
}




