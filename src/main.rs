#![allow(unused_parens)]
#![feature(let_chains)]
use bevy::{
	core::FixedTimestep,
	prelude::*,
};

use bevy_ecs_tilemap::prelude::*;


use bevy_prototype_lyon::prelude::*;
use bevy_screen_diags::{ScreenDiagsTimer};
use blood::BloodState;
use player::Player;
use rand::{Rng};


use crate::player::PluginPlayer;
mod player;

use crate::enemy::PluginEnemy;
mod enemy;

use crate::projectile::PluginProjectile;
mod projectile;

use crate::blood::PluginBlood;
mod blood;

use debug::DebugPlugin;
mod debug;

use weapon::PluginWeapon;
mod weapon;

use crate::wall::*;
mod wall;

/// An implementation of the classic game "Breakout"
pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const PPU: f32 = 32.0;

fn main() {
	//env::set_var("RUST_BACKTRACE", "full");

	App::new()
		.insert_resource(
			WindowDescriptor{
				width: 800.0, 
				height: 600.0, 
				title: "sjovt".to_string(), 
				vsync: false, 
				resizable: true,
				
				..Default::default()
			})

		.insert_resource(Msaa { samples: 4 })
		.insert_resource(GameState { active: false })
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))

		.add_plugins(DefaultPlugins)

		.add_plugin(ShapePlugin)

		.add_startup_system(setup)
		.add_system(bevy::input::system::exit_on_esc_system)
		.add_system(system_health)
		.add_system(system_shake)
		//.add_system(system_bloodtrail)

		.add_system_set_to_stage(CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                .with_system(system_camera_move)
        )

		.add_plugin(PluginPlayer)
		.add_plugin(PluginEnemy)
		.add_plugin(PluginProjectile)
		.add_plugin(PluginWeapon)
		.add_plugin(PluginBlood)
		 //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
		.add_plugin(DebugPlugin)



		.run();
}

struct SpawnTimer(Timer);


#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct Health{
	pub health: i32,
	pub healthmax: i32
}

impl Health{
	fn damage(&mut self, damageAmount: i32){
		self.health -= damageAmount;
	}
}


#[derive(Component)]
pub struct Shake{
	pub amount: f32,
	pub time: f32
}

struct GameState{
	active : bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Add the game's entities to our world
	// cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Shake{amount: 1.0, time: 0.0});

	commands.spawn_bundle(UiCameraBundle::default());


	// tilemap
/*	let handle: Handle<TiledMap> = asset_server.load("map.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });*/
}


fn system_shake(mut query: Query<(&mut Shake, &mut Transform)>, time: Res<Time>,){
	let mut rng = rand::thread_rng();
	for (mut shaker, mut transform) in query.iter_mut(){
		shaker.time -= time.delta_seconds();
		if shaker.time > 0.0 {
			transform.translation.x += (rng.gen::<f32>()-0.5)*shaker.amount;
			transform.translation.y += (rng.gen::<f32>()-0.5)*shaker.amount;
		}
	}
}


fn system_camera_move(
	mut query_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>, 
	query_player: Query<&Transform, (With<Player>, Without<Camera>)>, 
	_time: Res<Time>
){
	let cameraSpeed =0.1;
	let mut avgPosition = Vec2::new(0.0,0.0);

	for transform_player in query_player.iter(){
		avgPosition += transform_player.translation.truncate();
	}

	for mut transform_camera in  query_camera.iter_mut(){
		transform_camera.translation =
		Vec2::lerp(
			transform_camera.translation.truncate(), 
			avgPosition,
			cameraSpeed
		).extend(transform_camera.translation.z);
	
}

	
}



fn mouse_handler(
	mouse_button_input: Res<Input<MouseButton>>,
	mut query: Query<&mut Timer, With<ScreenDiagsTimer>>,
) {
	if mouse_button_input.just_released(MouseButton::Left) {
		let mut timer = query.single_mut();
		if timer.paused() {
			timer.unpause();
		} else {
			timer.pause();
		}
	}
}

fn system_health(
	mut commands: Commands, 
	query : Query<(Entity, &Health, &Transform)>, 
	mut images: ResMut<Assets<Image>>,
	blood: ResMut<BloodState>
){
	for (entity, health, transform) in query.iter(){
		if health.health <= 0{
			commands.entity(entity).despawn();
			blood.add_blood(transform.translation.truncate(), &mut images);
		}
	}
}


fn system_spawner(time: Res<Time>, mut timer: ResMut<SpawnTimer>, mut commands: Commands) {

	if timer.0.tick(time.delta()).just_finished() {
		commands
		.spawn_bundle(SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 0.0),
				scale: Vec3::new(20.0, 20.0, 1.0),
				..Default::default()
			},
			sprite: Sprite {
				color: Color::rgb(1.0, 0.5, 0.5),
				..Default::default()
			},
			..Default::default()
		}).insert(Enemy);
	}
}