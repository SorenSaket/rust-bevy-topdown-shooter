use bevy::prelude::*;

use crate::PPU;


pub struct PluginGrid;

impl Plugin for PluginGrid {
	fn build(&self, app: &mut App) {
		app
		.add_system_to_stage(CoreStage::PostUpdate,  system_grid_snap)
			
			
			;

	}
}



#[derive(Component)]
pub struct GridObject{
	pub position: IVec2
}

fn system_grid_snap(
	mut query: Query<(&mut Transform, &GridObject ), Changed<GridObject>>
){
	for (mut transform,gridobject) in query.iter_mut(){
		transform.translation = Vec3::new(gridobject.position.x as f32 * PPU,gridobject.position.y as f32 * PPU ,transform.translation.z);
	}
}