use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_prototype_lyon::prelude::*;
use bevy_screen_diags::{ScreenDiagsPlugin, ScreenDiagsTimer};

use crate::player::PluginPlayer;
use crate::enemy::PluginEnemy;

mod player;
mod enemy;

/// An implementation of the classic game "Breakout"
const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor{width: 800.0, height: 600.0, title: "sjovt".to_string(), vsync: true, resizable: false, ..Default::default()})

        .insert_resource(Msaa { samples: 4 })
        .insert_resource(GameState { active: false })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))

        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(PluginPlayer)
        .add_plugin(PluginEnemy)

        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_system(mouse_handler)
        .run();
}

struct SpawnTimer(Timer);


#[derive(Component)]
pub struct Enemy;

struct GameState{
    active : bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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