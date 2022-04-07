

use std::ops::Mul;

use bevy::{
	core::FixedTimestep,
	prelude::*, utils::Instant,
};
use bevy_prototype_lyon::prelude::*;

use crate::PPU;


use crate::{weapon::*, wall::*, grid::*};







pub struct PluginPlayer;

impl Plugin for PluginPlayer {
	fn build(&self, app: &mut App) {
		app.insert_resource(PlayerStates { counter: 0 })
			.add_system(gamepad_connections)
			.add_startup_system(setup)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1. / 60. as f64))
					.with_system(system_player_movement),
			)
			.add_system(animate_sprite)
			.add_system(system_player_holder)
			.add_system(system_player_buildertoggle)
			.add_system(system_player_builder)
			
			;
	}
}

struct PlayerStates {
	counter: usize,
}
#[derive(Component)]
struct AnimationTimer(Timer);
#[derive(Component, Clone)]
pub struct Player {
	pub builder: Entity,
	pub editing: bool,
	pub direction: f32,
	pub velocity: Vec2,
	pub gamepad: Gamepad,
	pub timer_shoot: Instant,
	pub timer_build: Instant,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	/*let _new_entity = commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("sword.png"),
		..Default::default()
	});*/
}

fn spawn_player(
	commands: &mut Commands, 
	gamepad: Gamepad, 
	asset_server: &Res<AssetServer>,
	mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>
){

	//let line = shapes::Line()
	let entity_builder = commands
	
	.spawn()
	.insert_bundle(
		SpriteBundle{	
			texture: asset_server.load("builder.png"),
			..Default::default() 
		}
	).insert(GridObject{ position: IVec2::new(0,0)})
	
	.id();

	let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 120.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands
		.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        }
		)
		.insert(AnimationTimer(Timer::from_seconds(0.1, true)))
		.insert(WeaponHolder{ request_pickup: false, weapon: None })
		.insert(Player {
			velocity: Vec2::new(0., 0.),
			gamepad,
			direction: 0.0,
			timer_shoot: Instant::now(),
			timer_build: Instant::now(),
			editing: false,
			builder: entity_builder
		});
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn remove_player(_commands: &mut Commands, _playerID: usize) {}

fn system_player_movement(
	mut query_player: Query<(&mut Player, &mut Transform, &WeaponHolder), Without<Weapon>>,
	mut query_weapon: Query<(Entity, &mut Weapon, &mut Transform, &mut Sprite), Without<Player>>,
	axes: Res<Axis<GamepadAxis>>,
	buttons: Res<Input<GamepadButton>>
) {
	

	let acc = 1.5;
	let friction = 0.1;

	for (mut player, mut transform, holder) in query_player.iter_mut() {
		if (player.editing) {
			continue;
		}

		let axis_lx = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickX);
		let axis_ly = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickY);

		let axis_rx = GamepadAxis(player.gamepad, GamepadAxisType::RightStickX);
		let axis_ry = GamepadAxis(player.gamepad, GamepadAxisType::RightStickY);

		let btn_rt2 = GamepadButton(player.gamepad, GamepadButtonType::RightTrigger2);
		
		let _btn_south = GamepadButton(player.gamepad, GamepadButtonType::South);
		let _btn_north = GamepadButton(player.gamepad, GamepadButtonType::North);

		// Player Directions
		if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
			let right_stick_pos = Vec2::new(x, y);
			if right_stick_pos.length_squared() > 0.02 {
				player.direction = f32::atan2(y, x);
			}
		}

		// Weapon Stuff
		if let Some(entity_playerWeapon) = &holder.weapon {
			if let Ok((_entity_weapon, mut weapon, mut transform_weapon,mut sprite_weapon)) = query_weapon.get_mut(*entity_playerWeapon) {
				
				// Shooting
				weapon.request_shoot = buttons.pressed(btn_rt2);
				
				//Weapon Rotation
				transform_weapon.rotation = Quat::from_axis_angle(Vec3::Z, player.direction);

				if f32::cos(player.direction) < 0.0 {
					sprite_weapon.flip_y = true;
				} else {
					sprite_weapon.flip_y = false;
				}
			}
		}
		if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
			player.velocity.x += x * acc;
			player.velocity.y += y * acc;
		}

		// Friction
		player.velocity *= 1.0 - friction;

		// Apply Velocity
		transform.translation.x += player.velocity.x;
		transform.translation.y += player.velocity.y;
	}
}

fn system_player_builder(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut query_player: Query<(&mut Player, &mut Transform)>,
	mut query_builder: Query<(&mut Sprite, &mut Transform, &mut GridObject), Without<Player>>,
	axes: Res<Axis<GamepadAxis>>,
	buttons: Res<Input<GamepadButton>>,
	time : Res<Time>
){	
	
	for (mut player, mut transform) in query_player.iter_mut() {
		if( !player.editing){
			continue;
		}

		if(let Ok((_,mut transform_builder, mut gridobject_builder)) =  query_builder.get_mut(player.builder)){
			
			if time.last_update().is_some() && time.last_update().unwrap().duration_since(player.timer_build).as_secs_f32() > 0.1 {
				// reset timer
				player.timer_build = time.last_update().unwrap();
				
				let axis_lx = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickX);
				let axis_ly = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickY);
	
				if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
					if f32::abs(x) > 0.5{
						gridobject_builder.position.x += f32::signum(x) as i32;
					}
					if f32::abs(y) > 0.5{
						gridobject_builder.position.y += f32::signum(y) as i32;
					}
				}
	
						
				
			}
			let btn_south = GamepadButton(player.gamepad, GamepadButtonType::South);
	
			if buttons.just_pressed(btn_south){
				spawn_wall(&mut commands, &asset_server, transform_builder.translation);
			}
		}
	}
}

fn system_player_buildertoggle(mut query_player: Query<(&mut Player, &mut Transform)>, buttons: Res<Input<GamepadButton>>){
	
	for (mut player, mut transform) in query_player.iter_mut() {
		let btn_west = GamepadButton(player.gamepad, GamepadButtonType::West);

		if(buttons.just_pressed(btn_west)){
			player.editing = !player.editing;
		}
	}
}



fn system_player_holder(
	mut query_player: Query<(&mut Player, &mut WeaponHolder)>,
	buttons: Res<Input<GamepadButton>>
){
	for (player, mut weaponholder) in query_player.iter_mut() {
		let btn_north = GamepadButton(player.gamepad, GamepadButtonType::North);
		weaponholder.request_pickup = buttons.pressed(btn_north);
	}
}

fn gamepad_connections(
	mut commands: Commands,
	mut gamepad_evr: EventReader<GamepadEvent>,
	mut query: Query<&Player>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let mut count = 0;
	for _ in query.iter_mut() {
		count += 1;
	}

	for GamepadEvent(id, kind) in gamepad_evr.iter() {
		match kind {
			GamepadEventType::Connected => {
				println!("New gamepad connected with ID: {:?}", id);

				spawn_player(&mut commands, *id,&asset_server,&mut texture_atlases);
			}
			GamepadEventType::Disconnected => {
				// remove_player(&mut commands,count);
			}
			GamepadEventType::ButtonChanged(GamepadButtonType::South, 0.5)=>{

			}
			_ => {}
		}
	}
}


