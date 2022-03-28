use bevy::{
    prelude::*, render::render_resource::*,
};
use rand::{thread_rng, Rng};
extern crate blit;

pub struct PluginBlood;

#[derive(Default)]
pub struct BloodState {
    blood: Handle<Image>,
    scale: f32,
    size: usize
}

impl BloodState{
    pub fn add_blood(&self, pos: Vec2, mut images: &mut ResMut<Assets<Image>>){
        // 512 * 512 * 4 = 1048576

        /* 
        let halfSize = ((self.size as f32) * self.scale)/2.0;
        

        let x = (((pos.x-halfSize)/self.scale)) as usize;
        
        let y = (((pos.y-halfSize)/self.scale)) as usize;
        */
        //println!("a  x: {}, y: {}", pos.x,pos.y);
        
        let halfSize = ((self.size as f32))/2.0;
        let x = (pos.x/self.scale+halfSize) as usize;        
        let y = (pos.y/self.scale+halfSize) as usize;
//
        // Add offset to center of image
        //+ (self.size/2 * self.size))

        let index : usize = 
        // Convert 2d to 1d index
        ( x + y * self.size)
        // multiply by 4 stride of image data
        * 4 +3;
        
        //incease opacity
        if let Some(img) = images.get_mut(&self.blood){
            if index > 0 && index < img.data.len() {
               img.data[index as usize] = u8::saturating_add(img.data[index as usize] ,128);
            }
        }

        /*if let Some(img) = images.get_mut(&self.blood){
            img.data = vec![128; img.data.len()];
        }*/
       

    }
}


impl Plugin for PluginBlood {
     fn build(&self, app: &mut App) {
       app.add_startup_system(setup);
       
    }
}

fn setup(
    mut commands: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
){
    let size_in_pixels:u32 = 1024;
    let scale:f32 = 32.0;

	let size = Extent3d {
        width: size_in_pixels,
        height: size_in_pixels,
        ..Default::default()
    };
	let mut image = Image {
        sampler_descriptor : SamplerDescriptor{
            mag_filter: FilterMode::Linear,
            ..Default::default()
        },
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::COPY_DST |TextureUsages::RENDER_ATTACHMENT , 
        },
        ..Default::default()
    };

    image.resize(size);

    // Set all red to max
    for x in 0..image.data.len() {
        if (x) % 4 == 0{
            if (x) < image.data.len()-1{
                image.data[x] = 255;
            }
        }
    }


    println!("{}",image.data.len());
    let image_handle = images.add(image);


    // Spawn the blood
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            flip_y: true,
            flip_x: false,
            ..Default::default()
        },
        
        texture: image_handle.clone(),
        
        transform: Transform {
            translation: Vec3::new(0.0,0.0,0.0),
            scale: Vec3::new(scale, scale, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(
        BloodState{
            blood: image_handle.clone(), 
            size: size_in_pixels as usize, 
            scale: scale
        });
        
} 