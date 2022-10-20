use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MyCamBundle {
    #[bundle]
    pub cam3d: Camera3dBundle,
    pub cam_state: CamState
}

#[derive(Component,Default,Debug)]
pub struct CamState {
    rot: Vec2,
    pub vel: Vec2,
    pub dist: f32,
    inv_x: bool,
    inv_y: bool
}

pub fn animate_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Camera3d, &mut CamState)>
) {
    let (mut transform, _camera, mut cam_state) = query.single_mut();

    let v = cam_state.vel * time.delta_seconds();
    cam_state.rot += v;
    
    let rot_x = if cam_state.inv_x { -1.0 } else { 1.0 } * cam_state.rot.x;
    let rot_y = if cam_state.inv_y { -1.0 } else { 1.0 } * cam_state.rot.y;
    
    let q = Quat::from_rotation_y(rot_x)
	.mul_quat(Quat::from_rotation_z(rot_y));
    
    let mut tr = Transform::from_xyz(0.0, cam_state.dist, 0.0);
    tr.translate_around(Vec3::ZERO, q);
    *transform = tr.looking_at(Vec3::ZERO, Vec3::Y);
}

pub fn make_camera(    
    rot: Vec2,
    dist: f32,    
    inv_x: bool, inv_y: bool) -> MyCamBundle {
    MyCamBundle {
	cam3d: Camera3dBundle {
	    transform: Transform::from_xyz(-175.0, 64.0, -175.0)
		.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
	},
	cam_state: CamState { rot,
			      vel: Vec2::ZERO,
			      dist,
			      inv_x, inv_y }
    }
}
