use glium::glutin;
use std::collections::VecDeque;

pub fn get_primary_monitor() -> glutin::MonitorId {
    glutin::EventsLoop::new().get_primary_monitor()
}

pub fn new_window(
    window_builder: glutin::WindowBuilder,
    context_builder: glutin::ContextBuilder,
) -> (glium::glutin::EventsLoop, glium::Display) {
    let ev_loop = glium::glutin::EventsLoop::new();
    let display = glium::Display::new(window_builder, context_builder, &ev_loop).unwrap();
    (ev_loop, display)
}

pub fn get_events(ev_loop: &mut glium::glutin::EventsLoop) -> VecDeque<glutin::Event> {
    let mut ev_vec = VecDeque::new();
    ev_loop.poll_events(|ev| ev_vec.push_back(ev));
    ev_vec
}
