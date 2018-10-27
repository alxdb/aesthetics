use glium::glutin;
use std::collections::VecDeque;

pub fn get_primary_monitor() -> glutin::MonitorId {
    glutin::EventsLoop::new().get_primary_monitor()
}

pub struct Window {
    ev_loop: glutin::EventsLoop,
    display: glium::Display,
}

impl Window {
    pub fn new(
        window_builder: glutin::WindowBuilder,
        context_builder: glutin::ContextBuilder,
    ) -> Self {
        let mut ev_loop = glium::glutin::EventsLoop::new();
        let display = glium::Display::new(window_builder, context_builder, &ev_loop).unwrap();
        // hacky fix for https://github.com/tomaka/glutin/issues/1069
        {
            use glium::backend::glutin::glutin::GlContext;
            ev_loop.poll_events(|_| {});
            let win = display.gl_window();
            let res: (u32, u32) = win.get_outer_size().unwrap().into();
            win.resize(res.into());
        }

        Window { ev_loop, display }
    }

    pub fn display_ref(&self) -> &glium::Display {
        &self.display
    }

    pub fn get_events(&mut self) -> VecDeque<glutin::Event> {
        let mut ev_vec = VecDeque::new();
        self.ev_loop.poll_events(|ev| ev_vec.push_back(ev));
        ev_vec
    }
}
