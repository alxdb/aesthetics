use glium::glutin;
use shred_derive::SystemData;
use specs::prelude::*;

#[derive(Default)]
pub struct WindowEvents(pub Vec<glutin::WindowEvent>);
#[derive(Default)]
pub struct DeviceEvents(pub Vec<glutin::DeviceEvent>);

#[derive(SystemData)]
pub struct EventHandlerData<'a> {
    window_events: Write<'a, WindowEvents>,
    device_events: Write<'a, DeviceEvents>,
}

pub struct EventHandler {
    events_loop: glutin::EventsLoop,
}

impl EventHandler {
    pub fn new(events_loop: glutin::EventsLoop) -> Self {
        EventHandler { events_loop }
    }
}

impl<'a> System<'a> for EventHandler {
    type SystemData = EventHandlerData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.window_events.0.clear();
        data.device_events.0.clear();
        self.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => data.window_events.0.push(event),
            glutin::Event::DeviceEvent { event, .. } => data.device_events.0.push(event),
            _ => (),
        });
    }
}
