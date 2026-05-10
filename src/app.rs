mod instance;

use std::rc::Rc;

use anyhow::{Ok, Result, anyhow};
use vulkanalia::Entry;
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::app::instance::Instance;

struct App {
    entry: Rc<Entry>,
    state: Option<AppState>
}

struct AppState {
    window:   Window,
    instance: Rc<Instance>
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title     ("Lightweaver")
                .with_inner_size(LogicalSize::new(1024, 768))
        )
            .unwrap()
        ;

        let state = AppState {
            instance: Rc::new(Instance::new(&window, self.entry.clone()).unwrap()),
            window,
        };

        self.state = Some(state)

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.state = None;

                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let state  = self.state.as_mut().unwrap();



                state.window.request_redraw();
            }
            _ => (),
        }
    }
}

pub fn main() -> Result<()> {

    pretty_env_logger::init();

    let loader   = unsafe { LibloadingLoader::new(LIBRARY)? };
    let entry    = unsafe { Entry::new(loader).map_err(|b| anyhow!("{}", b))? };


    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        entry: Rc::new(entry),
        state: None,
    };
    event_loop.run_app(&mut app)?;

    Ok(())

}
