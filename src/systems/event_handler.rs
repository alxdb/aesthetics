use components::{ActiveCamera, Camera};
use glium::glutin;
use shred_derive::SystemData;
use specs::prelude::*;
use std::collections::HashSet;

// uses virutal key codes at the moment, some keys may be missed
// eg numpad + - *
// Also, premature optimization when using BTreeSet? Use HashSet for now
#[derive(Default)]
pub struct InputState {
    pub key_state: HashSet<glutin::VirtualKeyCode>,
    pub mouse_delta: Option<(f64, f64)>,
    pub wheel_delta: Option<(f32, f32)>,
}

#[derive(Default)]
pub struct ShouldClose(pub bool);

#[derive(SystemData)]
pub struct EventHandlerData<'a> {
    input_state: Write<'a, InputState>,
    should_close: Write<'a, ShouldClose>,
    active_camera: Read<'a, ActiveCamera>,
    camera: WriteStorage<'a, Camera>,
}

#[derive(Default)]
struct DeviceHandlerState {
    wheel_was_moved: bool,
    mouse_was_moved: bool,
}

pub struct EventHandler {
    events_loop: glutin::EventsLoop,
}

impl EventHandler {
    pub fn new(events_loop: glutin::EventsLoop) -> Self {
        EventHandler { events_loop }
    }

    fn handle_window_event(
        event: glutin::WindowEvent,
        data: &mut <EventHandler as System>::SystemData,
    ) {
        match event {
            glutin::WindowEvent::Resized(new_size) => {
                if let Some(active_camera) = data.active_camera.0 {
                    if let Some(camera) = (&mut data.camera).get_mut(active_camera) {
                        camera.update_aspect((new_size.width / new_size.height) as f32);
                    }
                }
            }
            glutin::WindowEvent::CloseRequested => data.should_close.0 = true,
            _ => (),
        }
    }

    fn handle_device_event(
        event: glutin::DeviceEvent,
        data: &mut <EventHandler as System>::SystemData,
        state: &mut DeviceHandlerState,
    ) {
        match event {
            glutin::DeviceEvent::Key(key) => match key.state {
                glutin::ElementState::Pressed => {
                    if let Some(v_keycode) = key.virtual_keycode {
                        if !data.input_state.key_state.insert(v_keycode) {
                            panic!("keyboard desync press");
                        }
                    }
                }
                glutin::ElementState::Released => {
                    if let Some(v_keycode) = key.virtual_keycode {
                        if !data.input_state.key_state.remove(&v_keycode) {
                            eprintln!("keyboard desync release");
                        }
                    }
                }
            },
            glutin::DeviceEvent::MouseMotion { delta } => {
                state.mouse_was_moved = true;
                data.input_state.mouse_delta = Some(delta);
            }
            glutin::DeviceEvent::MouseWheel { delta } => match delta {
                glutin::MouseScrollDelta::LineDelta(rows, cols) => {
                    state.wheel_was_moved = true;
                    data.input_state.wheel_delta = Some((rows, cols))
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl<'a> System<'a> for EventHandler {
    type SystemData = EventHandlerData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut device_handler_state = DeviceHandlerState::default();
        self.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => Self::handle_window_event(event, &mut data),
            glutin::Event::DeviceEvent { event, .. } => {
                Self::handle_device_event(event, &mut data, &mut device_handler_state)
            }
            _ => (),
        });
        if !device_handler_state.mouse_was_moved {
            data.input_state.mouse_delta = None;
        }
        if !device_handler_state.wheel_was_moved {
            data.input_state.wheel_delta = None;
        }
    }
}
