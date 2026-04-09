
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use anyhow::{Ok, Result};

use crate::vulkan::VulkanApp;

// mod vk_app;

pub fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[derive(Default, Debug)]
struct App {
    window: Option<Window>,
    vk_app: Option<VulkanApp>,
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Lightweaver")
                    .with_inner_size(LogicalSize::new(1024, 768))
            )
            .unwrap()
        ;

        self.vk_app = unsafe {
            Some(VulkanApp::new(&window).unwrap())
        };

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                
            }
            _ => (),
        }
    }
}
