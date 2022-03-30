

use bevy::{
    prelude::*, utils::Instant,
};
use bevy_inspector_egui::Inspectable;
use rand::{random};

use crate::{projectile::{ProjectileSettings, spawn_projectile}};

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
	/// The spread between multiple projectiles in radians
	pub spread: f32,
	pub inaccuracy: f32,
	pub kickback: f32,
}

#[derive(Component )]
pub struct Weapon{
	pub settings: usize,
	pub timer_shoot: Instant,
	pub request_shoot: bool,
	pub owner: Option<Entity>
}


#[derive(Component, Inspectable)]
pub struct WeaponHolder{
	pub request_pickup: bool,
	pub weapon: Option<Entity>,
}


impl Plugin for PluginWeapon {
     fn build(&self, app: &mut App) {
		
		
       app
	   .add_startup_system(setup)
	   .add_system(system_weapon_pickup)
	   .add_system(system_weapon_shoot)
	   ;
	   
    }
}

fn setup(
    mut commands: Commands,
	asset_server: Res<AssetServer>
){
	let mut weapondata = WeaponData {weapons : Vec::new() };

	weapondata.weapons.push( WeaponSettings{
		texture: asset_server.load("gun.png"),
		firerate: 1.0,
		projectilesPerShot: 10,
		spread: 1.2,
		inaccuracy:  1.0,
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
    .insert(Weapon{
		settings: weaponIndex, 
		owner: None , 
		timer_shoot: Instant::now(),
		request_shoot: false
	});
}


 fn system_weapon_shoot(
	mut commands: Commands,
	mut query_weapon: Query<(Entity, &mut Weapon, &GlobalTransform)>,
	time : Res<Time>,
	weapons : Res<WeaponData>
){
	for (_entity_weapon, mut weapon, transform_weapon) in query_weapon.iter_mut(){
		if weapon.request_shoot {
			if time.last_update().is_some() && time.last_update().unwrap().duration_since(weapon.timer_shoot).as_secs_f32() > weapons.weapons[weapon.settings].firerate {
				let weaponsettings = &weapons.weapons[weapon.settings];
				// reset timer
				weapon.timer_shoot = time.last_update().unwrap();

				let rotstart = -(weaponsettings.spread/2.0);
				let rotspacing = (weaponsettings.spread/(weaponsettings.projectilesPerShot as f32));

				for x in 0..weaponsettings.projectilesPerShot{
					spawn_projectile(
						&mut commands,
						weapon.owner,
						&weaponsettings.projectile,
						transform_weapon.translation,
						transform_weapon.rotation.to_euler(EulerRot::YXZ).2 + rotstart + rotspacing*(x as f32) + (weaponsettings.inaccuracy * (random::<f32>() - 0.5)),
					);
				}
				
			
			} 
		}
	}
}



fn system_weapon_pickup(
	mut commands: Commands,
	mut query_weapon: Query<(Entity, &mut Weapon, &mut Transform), Without<WeaponHolder>>,
	mut query_holder: Query<(Entity, &mut WeaponHolder, &Transform), Without<Weapon>>,
	_buttons: Res<Input<GamepadButton>>,
){
	// Get closest player
	for (entity_holder, mut holder, transform_holder) in query_holder.iter_mut() {
		// If  do not want to pickup continue
		if !holder.request_pickup{
			continue;
		}
		// Search for weapon
		for (entity_weapon, mut weapon, mut transform_weapon) in query_weapon.iter_mut(){
			// Ignore if the weapon already has a owner
			if weapon.owner.is_some() {
				continue;
			}
			
			if Vec2::distance(transform_weapon.translation.truncate(), transform_holder.translation.truncate()) < 128.0 {
	
				commands.entity(entity_holder).push_children(&[entity_weapon]);
	
				transform_weapon.translation = Vec3::new(0.0, 0.0, transform_weapon.translation.z);
	
				weapon.owner = Some(entity_holder);
	
				holder.weapon = Some(entity_weapon);
				
				break;
			}
		}
	}

}




