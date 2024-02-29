mod landmass;
mod gen_terrain;
mod terrain_task;
mod camera;
mod mouse_inputs;

use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin,FrameTimeDiagnosticsPlugin};
use bevy::pbr::StandardMaterial;
use  bevy::render::render_resource::Face;
use crate::terrain_task::{add_terrain,handle_terrain_task};
use crate::camera::{animate_camera,make_camera};
use crate::mouse_inputs::{mouse_button_events,mouse_move_events};

const DEFAULT_SEED : u64 = 0x44d5afba2c391c8a;
const SIZE: u32 = 512;

const INV_X: bool = true;
const INV_Y: bool = false;

const CAM_DIST: f32 = 400.0;
const CAM_RX: f32 = if INV_X { 1.0 } else { -1.0 } * 1.0;
const CAM_RY: f32 = if INV_Y { 1.0 } else { -1.0 } * 1.0;

pub struct Palette(image::RgbaImage);
pub struct TerrainSize(u32);

pub struct GenTerrainEvent { seed: u64 }

fn main() {
    let palette = image::open("assets/palette.png")
	.expect("palette file not found in asset directory")
	.into_rgba8();

    App::new()
        .add_plugins(DefaultPlugins)
	.add_plugin(LogDiagnosticsPlugin::default())
	.add_plugin(FrameTimeDiagnosticsPlugin::default())
	.insert_resource(ClearColor(Color::rgb(0.3, 0.6, 1.0)))
	.insert_resource(Palette(palette))
	.insert_resource(TerrainSize(SIZE))
	.add_event::<GenTerrainEvent>()
        .add_startup_system(setup)
	.add_system(add_terrain)
	.add_system(handle_terrain_task)
	.add_system(animate_camera)
	.add_system(mouse_button_events)
	.add_system(mouse_move_events)
        .run();
}

fn setup(
    mut commands: Commands,
    mut terrain_events: EventWriter<GenTerrainEvent>,
    size: Res<TerrainSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Sea plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: size.0 as f32 })),
        material: materials.add(Color::rgb(0.031, 0.067, 0.133).into()),
	transform: Transform::from_xyz(0.0, 0.01, 0.0),
        ..default()
    });

    // Sky sphere
    let sky_mat = StandardMaterial {
	base_color_texture: Some(asset_server.load("sky.png")),
	double_sided: true,
	cull_mode: Some(Face::Front),
	unlit: true,
	..default()
    };
    let sphere = Mesh::from(shape::Icosphere { radius: size.0 as f32 * 10.0,
					       subdivisions: 1 });
    commands.spawn_bundle(PbrBundle {    
	mesh: meshes.add(sphere),
	material: materials.add(sky_mat),
	..default()
    });
    
    // Camera
    commands.spawn_bundle(make_camera(Vec2::new(CAM_RX, CAM_RY),
				      CAM_DIST, INV_X, INV_Y));

    // Lights
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
	    color: Color::rgb(1.0, 1.0, 0.95),
            illuminance: 60_000.0,
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

    terrain_events.send(GenTerrainEvent { seed: DEFAULT_SEED });
}
