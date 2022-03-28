use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_prototype_lyon::prelude::*;

use bevy_inspector_egui::Inspectable;

use crate::{projectile::spawn_projectile, Shake, weapon::Weapon};

use rand::prelude::random;
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


#[derive(Component, Clone)]
pub struct Player {
    pub velocity: Vec2,
    pub weapon : Option<Entity>,
    pub gamepad : Gamepad
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let new_entity = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sword.png"),
        ..Default::default()
    });
}

fn spawn_player(commands: &mut Commands, playerID: Gamepad){
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
            translation: Vec3::new(0.0,0.0,0.8),
            ..Default::default()
        }
    )).insert(Player{velocity : Vec2::new(0.,0.), gamepad:playerID, weapon: None});
}

fn player_shoot(commands: &mut Commands, playerID: usize, dir: f32){
    
}

fn remove_player(commands: &mut Commands, playerID: usize){

}



fn system_player_movement(
    mut query: Query<(&mut Player, &mut Transform)>, 
    mut query_weapon: Query<(Entity, &Weapon, &mut Transform, &mut Sprite), Without<Player>>, 
    mut query_camera: Query<&mut Shake, With<Camera>>,

    axes: Res<Axis<GamepadAxis>>, 
    buttons: Res<Input<GamepadButton>>, 
    mut commands : Commands
) {
    let acc = 1.5;
    let friction = 0.1;

    for (mut player, mut transform) in query.iter_mut(){
        
        let axis_lx = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(player.gamepad, GamepadAxisType::LeftStickY);

        let axis_rx = GamepadAxis(player.gamepad, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis(player.gamepad, GamepadAxisType::RightStickY);

        let btn_rt2 = GamepadButton(player.gamepad, GamepadButtonType::RightTrigger2);
         /*
            if(right_stick_pos.length_squared()< 0.02){
                return;
            }*/
        // Shooting
        if buttons.pressed(btn_rt2) {
            
            if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                spawn_projectile(&mut commands, transform.translation, f32::atan2(y,x) + random::<f32>());

                let mut shaker = query_camera.single_mut();
               
                    shaker.time = 0.1;
            }
        }

        //Weapon Rotation

        if let Some(entity_playerWeapon) = &player.weapon{
            if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
                
                for (entity_weapon, w,mut t, mut sprite_weapon) in query_weapon.iter_mut(){
                    if &entity_weapon == entity_playerWeapon{
                        let angle = f32::atan2(y,x);
                        t.rotation = Quat::from_axis_angle(Vec3::Z, angle);
                        if x < 0.0{
                            sprite_weapon.flip_y = true;
                        }else{
                            sprite_weapon.flip_y = false;
                        }
                        
                        break;
                    }
                }
            }
        }

        


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

                spawn_player(&mut commands,*id);
            }
            GamepadEventType::Disconnected => {
               // remove_player(&mut commands,count);
            }
            // other events are irrelevantxx
            _ => {}
        }
    }
}


fn gamepad_input_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    mut gamepad_evr: EventReader<GamepadEvent>, 
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>, 
    query: Query<(&Player, &Transform)>) {
    
    /* 
    for player in query.iter(){
        
        gamepad_evr.iter().f

    }
    
       // iterates every active game pad
    for gamepad in gamepads.iter() {
        
    }
    
    */
    
    
    
 
    // Iterate though all the gamepad events
    for GamepadEvent(id, event) in gamepad_evr.iter() {

        let a = query.iter().find(|&(x,_) | x.gamepad == *id);

        if a.is_none()
        {
            continue;
        }
        
        let (player, transform ) = a.unwrap();


        use GamepadEventType::{AxisChanged, ButtonChanged};

        match event {
            AxisChanged(GamepadAxisType::RightStickX, x) => {
                // Right Stick moved (X)
            }
            AxisChanged(GamepadAxisType::RightStickY, y) => {
                // Right Stick moved (Y)
            }
            ButtonChanged(GamepadButtonType::RightTrigger2, val) => {

            }
            _ => {} // don't care about other inputs
     
        }
    }
}