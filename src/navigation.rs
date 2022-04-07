use bevy::prelude::*;
use dubble::DoubleBuffered;

extern crate queues;
use queues::*;

extern crate bit_vec;
use bit_vec::BitVec;

struct NavgiationRegenEvent;

#[derive(Component)]
struct Affector;

#[derive(Component)]
struct Source;


struct NavigationData{
	map: Vec<f32>,
	heatmap: Vec<f32>,
	vectorfield: Vec<Vec2>,
	pub width: usize,
	pub height: usize,
}


impl NavigationData{
	
	pub fn get_field_value(&self, x: usize, y:usize) -> Vec2{
		return self.vectorfield[x + y *self.width];
	}

	fn generate_heatmap(&mut self, affectorX: usize, affectorY: usize){
		// the index of the affector
		let index_affector : usize = (affectorX + affectorY*self.width);
		
		let mut closedMap = BitVec::from_elem((self.width*self.height), false);

		self.heatmap.iter_mut().map(|x| *x = f32::INFINITY).count();
		
		let mut openList: Queue<IVec2> = queue![];
		

		openList.add(IVec2::new(affectorX as i32, affectorY as i32));
		// Set the source point to zero
		self.heatmap[index_affector] = 0.0;

		while (let Ok(realPos) = openList.remove() )
		{
			let rx = realPos.x as usize;
			let ry = realPos.y as usize;
			let index_r =rx+ ry*self.width;
			
			closedMap.set(index_r, true);

			for x in -1..2 as i32 
			{
				for y in -1..2 as i32 
				{
					// Do not run on self or corners
					if ((x == 0 && y == 0) || (x == 1 && y == 1) || (x == 1 && y == -1) || (x == -1 && y == 1) || (x == -1 && y == -1)){
						continue;
					}

					let cx = (rx as i32 + x);
					let cy = (ry as i32 + y);
					
					// Do not run out of bounds
					if (cx >= self.width as i32 || cx < 0 || cy >= self.height as i32 || cy < 0){
						continue;
					}

					let index_c = cx as usize + cy as usize *self.width;

				
					
					// Cancel if closed
					if (closedMap[index_c]){
						continue;
					}

					let newValue = self.map[index_c] + self.heatmap[index_r] + 1.0;
					
					// If this tile has not been set or lower value is possible
					if (self.heatmap[index_c] == 0.0 || f32::abs(self.heatmap[index_c]) > f32::abs(newValue))
					{
						self.heatmap[index_c] = newValue;
						openList.add(IVec2::new(cx, cy));
						closedMap.set(index_c,true);
					}
				}
			}
		}
	}

	fn generate_vectorfield(&mut self){
		// The size of the heightmap and heatmap
		let convsize: i32 = 1;

		for rx in 0..self.width as i32
		{
			for ry in 0..self.height as i32
			{
				// Do not generate a vector for walls
				//if (map.values[i] >= wallValue)
				//    continue; 
				let mut dir = Vec2::ZERO;
				let mut minValue = f32::MAX;

				for x in -convsize..convsize+1
				{
					for y in -convsize..convsize+1
					{
						// Do not run on self
						if (x == 0 && y == 0){
							continue;
						}
						let cx = (rx + x)as usize;
						let cy = (ry + y)as usize;
						let index_c = (cx + cy*self.width) as usize;

						// Do not run out of bounds
						if (cx >= self.width || cx < 0 || cy >=  self.height || cy < 0){
							continue;
						}

						if (self.heatmap[index_c] < minValue)
						{
							dir = Vec2::new(x as f32, y as f32);
							minValue = self.heatmap[index_c];
						}
					}
				}
				let index_r = (rx + ry*(self.width as i32)) as usize;
				self.vectorfield[index_r] = dir.normalize();
			}
		}

      
	}

}


pub struct PluginNavigation;

impl Plugin for PluginNavigation {
	fn build(&self, app: &mut App) {
		let width = 64;
		let height = 64;
		let count = width*height;

		app
		.insert_resource(
			NavigationData {
				map:  vec![0.0;count],
				heatmap: vec![0.0;count],
				vectorfield: vec![Vec2::ZERO; count],
				width : width,
				height: height,
			}
		);
		//.add_system(system_navigation_regen);

	}
}




fn system_navigation_regen(
	mut event_regen: EventReader<NavgiationRegenEvent>, 
	mut data: Res<NavigationData>,
	mut query_affector: Query<(&Affector, &Transform)>
){
	/*let Ok(affector) = query_affector.get_single();

	// Only do this once if a single event exsists
	for ev in event_regen.iter() {
	
		
		//data.GenerateHeatmap();
		//data.GenerateVectorfield();


		break;
    }*/
}