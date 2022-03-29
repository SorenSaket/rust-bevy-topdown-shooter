use bevy::{
	core::FixedTimestep,
	prelude::*, utils::Instant,
};
use bevy_prototype_lyon::prelude::*;



use crate::{weapon::*};


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

			.add_system(system_player_holder);


	}
}

struct PlayerStates {
	counter: usize,
}

#[derive(Component, Clone)]
pub struct Player {
	pub direction: f32,
	pub velocity: Vec2,
	pub weapon: Option<Entity>,
	pub gamepad: Gamepad,
	pub timer_shoot: Instant,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let _new_entity = commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("sword.png"),
		..Default::default()
	});
}

fn spawn_player(commands: &mut Commands, playerID: Gamepad) {
	let shape = shapes::RegularPolygon {
		sides: 6,
		feature: shapes::RegularPolygonFeature::Radius(32.0),
		..shapes::RegularPolygon::default()
	};
	//let line = shapes::Line()

	commands
		.spawn_bundle(GeometryBuilder::build_as(
			&shape,
			DrawMode::Outlined {
				fill_mode: FillMode::color(Color::CYAN),
				outline_mode: StrokeMode::new(Color::BLACK, 4.0),
			},
			Transform {
				translation: Vec3::new(0.0, 0.0, 0.8),
				..Default::default()
			},
		))
		.insert(Player {
			velocity: Vec2::new(0., 0.),
			gamepad: playerID,
			weapon: None,
			direction: 0.0,
			timer_shoot: Instant::now()
		});
}


fn remove_player(_commands: &mut Commands, _playerID: usize) {}

fn system_player_movement(
	mut query: Query<(&mut Player, &mut Transform, &mut WeaponHolder), Without<Weapon>>,
	mut query_weapon: Query<(Entity, &mut Weapon, &mut Transform, &mut Sprite), Without<Player>>,
	axes: Res<Axis<GamepadAxis>>,
	buttons: Res<Input<GamepadButton>>,
	_commands: Commands,
) {
	let acc = 1.5;
	let friction = 0.1;

	for (mut player, mut transform, _weaponholder) in query.iter_mut() {
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
		if let Some(entity_playerWeapon) = &player.weapon {
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
) {
	let mut count = 0;
	for _ in query.iter_mut() {
		count += 1;
	}

	for GamepadEvent(id, kind) in gamepad_evr.iter() {
		match kind {
			GamepadEventType::Connected => {
				println!("New gamepad connected with ID: {:?}", id);

				spawn_player(&mut commands, *id);
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
