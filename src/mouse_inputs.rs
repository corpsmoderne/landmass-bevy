use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput,MouseWheel,MouseMotion};
use bevy::input::ButtonState;

use crate::camera::CamState;
use crate::terrain_task::IsLandMass;
use crate::GenTerrainEvent;

pub fn mouse_move_events(
    buttons: Res<Input<MouseButton>>,    
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,        
    mut query: Query<&mut CamState>
) {
    let mut cam_state = query.single_mut();

    for event in mouse_wheel_events.iter() {
	cam_state.dist += event.y * 10.0;
    }
    
    if buttons.pressed(MouseButton::Left) {
	if mouse_motion_events.is_empty() {
	    cam_state.vel = Vec2::ZERO;
	} else {
	    for event in mouse_motion_events.iter() {
		cam_state.vel = event.delta / 2.0;
	    }
	}
    } else if buttons.just_released(MouseButton::Left) &&
	mouse_motion_events.is_empty() {
	    cam_state.vel = Vec2::ZERO;
	}
}

pub fn mouse_button_events(
    mut commands: Commands,
    mut terrain_events: EventWriter<GenTerrainEvent>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    query: Query<(Entity, &IsLandMass)>,
) {
    for event in mouse_button_events.iter() {
	if event.button == MouseButton::Left &&
	    event.state == ButtonState::Pressed {
	    } else if event.button == MouseButton::Right &&
	    event.state == ButtonState::Pressed {
		for (entity,_) in &query {
		    commands.entity(entity).despawn();
		    terrain_events
			.send(GenTerrainEvent { seed: rand::random() });
		}
	    }
    }
}
