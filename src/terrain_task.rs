use bevy::prelude::*;
use bevy::pbr::StandardMaterial;
use bevy::render::render_resource::{Extent3d,TextureDimension,TextureFormat};
use bevy::tasks::{Task,AsyncComputeTaskPool};
use futures_lite::future;
use crate::landmass::Landmass;
use crate::gen_terrain::gen_mesh_terrain;
use crate::{Palette,TerrainSize};

#[derive(Component)]
pub struct ComputeLandmass(Task<(u32,Mesh,Image)>);

#[derive(Component,Default,Debug)]
pub struct IsLandMass { }

#[derive(Bundle, Default)]
pub struct LandMassBundle {
    #[bundle]
    pbr: PbrBundle,
    is_land_mass: IsLandMass
}

pub fn add_terrain(
    commands: &mut Commands,
    res_palette: &Res<Palette>,
    res_size: &Res<TerrainSize>,
    seed: u64
) {
    info!("new terrain seed: 0x{:x}", seed);
    let thread_pool = AsyncComputeTaskPool::get();
    let palette = res_palette.0.clone();
    let size = res_size.0;
    let task = thread_pool.spawn(async move {
	let landmass = Landmass::with_palette(size, seed, palette);
	let mesh = gen_mesh_terrain(&landmass);
	
	let tex = Image::new_fill(
	    Extent3d { width: landmass.img.width(),
		       height: landmass.img.height(),
		       depth_or_array_layers: 1 },
	    TextureDimension::D2,
	    landmass.img.as_raw(),
	    TextureFormat::Rgba8UnormSrgb
	);

	(size, mesh, tex)
    });
    
    commands.spawn().insert(ComputeLandmass(task));
}

pub fn handle_terrain_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeLandmass)>,    
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some((size, mesh,tex)) =
	    future::block_on(future::poll_once(&mut task.0)) {

		let material = StandardMaterial {
		    base_color_texture: Some(images.add(tex)),
		    perceptual_roughness: 0.6,
		    metallic: 0.0,
		    reflectance: 0.0,
		    ..default()
		};
		
		let center = size as f32 / 2.0;
		commands.spawn_bundle(LandMassBundle {
		    pbr: PbrBundle {
			mesh: meshes.add(mesh),
			material: materials.add(material),
			transform: Transform::from_xyz(-center, 0.0, -center),
			..default()
		    },
		    is_land_mass: IsLandMass {}
		});
		
		commands.entity(entity).despawn();
            }
    }
}
