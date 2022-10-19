mod landmass;
mod gen_terrain;

use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput,MouseWheel,MouseMotion};
use bevy::input::ButtonState;
use bevy::diagnostic::{LogDiagnosticsPlugin,FrameTimeDiagnosticsPlugin};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::render::render_resource::{Extent3d,TextureDimension,TextureFormat};
				    

use image::open;

const SIZE: u32 = 512;
const CAM_DIST: f32 = 200.0;
//const CAM_ALT: f32 = 60.0;
const CAM_RX: f32 = 0.5;
const CAM_RY: f32 = -0.25;

#[derive(Bundle, Default)]
struct MyCamBundle {
    #[bundle]
    cam3d: Camera3dBundle,
    cam_state: CamState
}

#[derive(Component,Default,Debug)]
struct CamState {
    dist: f32,
    rot_x: f32,
    rot_y: f32,
    v_x: f32,
    v_y: f32
}

#[derive(Bundle, Default)]
struct LandMassBundle {
    #[bundle]
    pbr: PbrBundle,
    land_mass: LandMass
}

#[derive(Component,Default,Debug)]
struct LandMass { }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
	.add_plugin(LogDiagnosticsPlugin::default())
	.add_plugin(FrameTimeDiagnosticsPlugin::default())
	.insert_resource(ClearColor(Color::rgb(0.3, 0.6, 1.0)))
        .add_startup_system(setup)
	.add_system(animate_camera)
	.add_system(mouse_button_events)
	.add_system(mouse_move_events)
        .run();
}

fn animate_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Camera3d, &mut CamState)>
) {
    for (mut transform, _camera, mut cam_state) in &mut query {	
	let t = time.delta_seconds();
	cam_state.rot_x += cam_state.v_x * t;
	cam_state.rot_y += cam_state.v_y * t;
	
	let a = cam_state.rot_x;
	let b = -cam_state.rot_y;
	let x = a.cos() as f32 ;
	let y = b.cos() as f32 ;
	let z = a.sin() as f32 ;	
	*transform = Transform::from_xyz(x * cam_state.dist,
					 y * cam_state.dist,
					 z * cam_state.dist)
	    .looking_at(Vec3::ZERO, Vec3::Y)
    }
}

fn mouse_move_events(
    buttons: Res<Input<MouseButton>>,    
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,        
    mut query: Query<&mut CamState>
) {
    for event in mouse_wheel_events.iter() {
	for mut cam_state in &mut query {
	    cam_state.dist += event.y * 5.0;
	}
    }
    if buttons.just_released(MouseButton::Left) {
	if mouse_motion_events.is_empty() {
	    for mut cam_state in &mut query {
		cam_state.v_x = 0.0;
		cam_state.v_y = 0.0;
	    }
	}
    }
    if buttons.pressed(MouseButton::Left) {
	for mut cam_state in &mut query {
	    if mouse_motion_events.is_empty() {
		cam_state.v_x = 0.0;
		cam_state.v_y = 0.0;
	    } else {
		for event in mouse_motion_events.iter() {
		    cam_state.v_x = event.delta.x / 2.0;
		    cam_state.v_y = event.delta.y / 2.0;
		}
	    }
	}
    }
}

fn mouse_button_events(
    mut commands: Commands,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,    
    mut query: Query<(Entity, &LandMass)>,
) {
    for event in mouse_button_events.iter() {
	if event.button == MouseButton::Left &&
	    event.state == ButtonState::Pressed {
		//println!("{:?}", event);
	    } else if event.button == MouseButton::Right &&
	    event.state == ButtonState::Pressed {
		for (entity,_) in &mut query {
		    commands.entity(entity).despawn();
		}
		add_terrain(&mut commands, &mut meshes, &mut images,
			    &mut materials, rand::random::<u64>());
	    }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // terain
    add_terrain(&mut commands, &mut meshes, &mut images,
		&mut materials, 0x9b8ff544d42e7939);

    // plane / sea
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: SIZE as f32 / 2.0 })),
        material: materials.add(Color::rgb(0.031, 0.067, 0.133).into()),
	transform: Transform::from_xyz(0.0, 0.01, 0.0),
        ..default()
    });

    //sky
    let sky_mat = StandardMaterial {
	base_color: Color::rgb(1.0, 1.0, 1.0),
	base_color_texture: Some(asset_server.load("sky.png")),
	perceptual_roughness: 0.0,
	metallic: 0.0,
	reflectance: 0.0,
	unlit: true,
	..default()
    };
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
	    Mesh::from(shape::Icosphere { radius: SIZE as f32 * 4.0,
					  subdivisions: 16})),
        material: materials.add(sky_mat),
	transform: Transform::from_scale(Vec3::new(1.0, -1.0, 1.0)),
        ..default()
    });    
    
    // lights
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
	    color: Color::rgb(1.0, 1.0, 0.97),
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 3.0, 3.0)
	    .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });    
    commands.insert_resource(AmbientLight {
	color: Color::rgb(0.22, 0.29, 0.4),
	brightness: 4.0
    });

    // camera
    commands.spawn_bundle(MyCamBundle {
	cam3d: Camera3dBundle {
	    transform: Transform::from_xyz(-175.0, 64.0, -175.0)
		.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
	},
	cam_state: CamState { dist: CAM_DIST,
			      rot_x: CAM_RX, rot_y: CAM_RY,
			      v_x: 0.0,
			      v_y: 0.0 }
    });
}

fn add_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    seed: u64
) {
    
    let palette = open("assets/palette2.png").unwrap().into_rgba8();
    let landmass = landmass::Landmass::with_palette(SIZE, seed, palette);
    //let landmass = landmass::Landmass::new(SIZE, seed); 
    let scale = 0.5;
    let mesh = gen_terrain::gen_mesh_terrain(&landmass, scale);
    let center = scale * landmass.size as f32 / 2.0;

    let tex = Image::new_fill(
	Extent3d { width: landmass.img.width(),
		   height: landmass.img.height(),
		   depth_or_array_layers: 1 },
	TextureDimension::D2,
	landmass.img.as_raw(),
	TextureFormat::Rgba8UnormSrgb
    );
    
    let material = StandardMaterial {
	base_color: Color::rgb(1.0, 1.0, 1.0),
	base_color_texture: Some(images.add(tex)),
	//alpha_mode: AlphaMode::Blend,	
	perceptual_roughness: 0.6,
	metallic: 0.0,
	reflectance: 0.0,
	..default()
    };
    
    commands.spawn_bundle(LandMassBundle {
	pbr: PbrBundle {
	    mesh: meshes.add(mesh),
	    material: materials.add(material),
	    transform: Transform::from_xyz(-center, 0.0, -center),
	    ..default()
	},
	land_mass: LandMass {}
    });
}
