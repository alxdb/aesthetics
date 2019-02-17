use glium::glutin;
use shred_derive::SystemData;
use specs::prelude::*;

use super::event_handler::DeviceEvents;
use std::collections::HashMap;

#[derive(Default)]
pub struct KeyState(pub HashMap<u32, bool>);
// A HashMap might be a bit inefficient
// could probably just use a fixed array with indexing

#[derive(SystemData)]
pub struct InputHandlerData<'a> {
    device_events: Read<'a, DeviceEvents>,
    key_state: Write<'a, KeyState>,
}

pub struct InputHandler;

impl<'a> System<'a> for InputHandler {
    type SystemData = InputHandlerData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for device_event in &data.device_events.0 {
            match device_event {
                glutin::DeviceEvent::Key(key) => match key.state {
                    glutin::ElementState::Pressed => {
                        data.key_state.0.insert(key.scancode, true);
                    }
                    glutin::ElementState::Released => {
                        data.key_state.0.insert(key.scancode, false);
                    }
                },
                _ => (),
            }
        }
    }
}
