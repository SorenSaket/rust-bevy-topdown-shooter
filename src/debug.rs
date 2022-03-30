use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::weapon::WeaponHolder;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<WeaponHolder>()
            //.register_type::<ReflectedType>() 
            ;
        }
    }
}