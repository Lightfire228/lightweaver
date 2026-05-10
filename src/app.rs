mod instance;
mod device;
mod surface;
mod swapchain;
mod image_view;

use std::rc::Rc;

use anyhow::{Ok, Result, anyhow};
use vulkanalia::vk::{InstanceV1_0, KhrSurfaceExtensionInstanceCommands};
use vulkanalia::{Entry, vk, Instance as VkInstance};
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};


use crate::app::device::{Device, SuitabilityError};
use crate::app::instance::Instance;
use crate::app::surface::Surface;
use crate::app::swapchain::Swapchain;


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


struct App {
    entry: Rc<Entry>,
    state: Option<AppState>
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

        self.state = Some(
            AppState::new(window, self.entry.clone()).unwrap()
        )

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



struct AppState {
    window:        Window,
    instance:      Rc<Instance>,
    device:        Rc<Device>,
    surface:       Surface,
    swapchain:     Swapchain,

}

impl AppState {
    pub fn new(window: Window, entry: Rc<Entry>) -> Result<Self> {

        let instance  = Instance ::new(&window,          entry   )?;
        let surface   = Surface  ::new(instance.clone(), &window )?;
        let device    = Device   ::new(instance.clone(), &surface)?;
        let swapchain = Swapchain::new(instance.clone(), device.clone(), &window, &surface)?;

        Ok(Self {
            window,
            instance,
            surface,
            device,
            swapchain,
        })
    }
}
