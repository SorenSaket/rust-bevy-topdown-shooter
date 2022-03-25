use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_prototype_lyon::prelude::*;

pub struct PluginPlayer;

impl Plugin for PluginPlayer {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PlayerStates{ counter: 0})    
        .add_system(gamepad_connections)
        .add_system(gamepad_input_events)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1./60. as f64))
                .with_system(system_player_movement)
        );
    }
}

struct PlayerStates {
    counter: usize,
}


#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub gamepad : Gamepad
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let new_entity = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sword.png"),
        ..Default::default()
    });
}

fn spawn_player(commands: &mut Commands, playerID: usize){
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
        Transform::default()
    )).insert(Player{velocity : Vec2::new(0.,0.), gamepad: Gamepad(playerID)});
}


fn remove_player(commands: &mut Commands, playerID: usize){

}



fn system_player_movement(mut query: Query<(&mut Player, &mut Transform)>, axes: Res<Axis<GamepadAxis>>) {
    let acc = 1.5;
    let friction = 0.1;

    for (mut player, mut transform) in query.iter_mut(){
        let axis_lx = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickY);
        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            player.velocity.x += x * acc;
            player.velocity.y += y * acc;
        }

        // Friction
        player.velocity *= 1.0-friction;

        // Apply Velocity
        transform.translation.x += player.velocity.x;
        transform.translation.y += player.velocity.y;
    }
}


fn gamepad_connections(mut commands: Commands, mut gamepad_evr: EventReader<GamepadEvent>, mut query: Query<(&Player)>) {
    let mut count = 0;
    for _ in query.iter_mut(){
        count+=1;
    }
    
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                spawn_player(&mut commands,count);
            }
            GamepadEventType::Disconnected => {
               // remove_player(&mut commands,count);
            }
            // other events are irrelevantxx
            _ => {}
        }
    }
}


fn gamepad_input_events(mut gamepad_evr: EventReader<GamepadEvent>) {

    for GamepadEvent(id, event) in gamepad_evr.iter() {

        use GamepadEventType::{AxisChanged, ButtonChanged};

        match event {
            AxisChanged(GamepadAxisType::RightStickX, x) => {
                // Right Stick moved (X)
            }
            AxisChanged(GamepadAxisType::RightStickY, y) => {
                // Right Stick moved (Y)
            }
            ButtonChanged(GamepadButtonType::DPadDown, val) => {
                // buttons are also reported as analog, so use a threshold
                if *val > 0.5 {
                    // button pressed
                }
            }
            _ => {} // don't care about other inputs
        }
    }
}