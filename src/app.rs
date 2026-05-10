mod depth_image;
mod descriptor_set_layout;
mod device;
mod framebuffers;
mod image;
mod image_view;
mod instance;
mod pipeline;
mod render_pass;
mod surface;
mod swapchain;

use std::hash::{Hash, Hasher};
use std::rc::Rc;

use anyhow::{Ok, Result, anyhow};
use log::debug;
use vulkanalia::vk::{HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands};
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
use crate::app::render_pass::RenderPass;
use crate::app::surface::Surface;
use crate::app::swapchain::Swapchain;
use crate::app::descriptor_set_layout::DescriptorSetLayout;

pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;


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
    window:                Window,
    instance:              Rc<Instance>,
    device:                Rc<Device>,
    swapchain:             Swapchain,
    render_pass:           RenderPass,
    descriptor_set_layout: DescriptorSetLayout,

}

impl AppState {
    pub fn new(window: Window, entry: Rc<Entry>) -> Result<Self> {
        type DSLayout = DescriptorSetLayout;

        let instance                 = Instance ::new(&window,          entry   )?;
        let surface                  = Surface  ::new(instance.clone(), &window )?;
        let device                   = Device   ::new(instance.clone(), &surface)?;
        let descriptor_set_layout    = DSLayout ::new(device  .clone())?;
        let (swapchain, render_pass) = Swapchain::new(instance.clone(), device.clone(), &window, surface, &descriptor_set_layout)?;

        Ok(Self {
            window,
            instance,
            device,
            swapchain,
            render_pass,
            descriptor_set_layout,
        })
    }
}

// impl Drop for AppState {
//     fn drop(&mut self) {
//         debug!("Dropping App State");

//         debug!("/Dropping App State");
//     }
// }



#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos:        Vec3,
    color:      Vec3,
    tex_coord:  Vec2,
}

pub type Index = u32;

impl Vertex {
    pub const fn new(pos: Vec3, color: Vec3, tex_coord: Vec2) -> Self {
        Self {
            pos,
            color,
            tex_coord,
        }
    }

    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding   (0)
            .stride    (size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding (0)
            .location(0)
            .format  (vk::Format::R32G32B32_SFLOAT)
            .offset  (0)
            .build   ()
        ;

        let color = vk::VertexInputAttributeDescription::builder()
            .binding (0)
            .location(1)
            .format  (vk::Format::R32G32B32_SFLOAT)
            .offset  (size_of::<Vec3>() as u32)
            .build   ()
        ;

        let tex_coord = vk::VertexInputAttributeDescription::builder()
            .binding (0)
            .location(2)
            .format  (vk::Format::R32G32_SFLOAT)
            .offset  ((size_of::<Vec3>() + size_of::<Vec3>()) as u32)
            .build   ()
        ;

        [pos, color, tex_coord]
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
            self.pos       == other.pos
         && self.color     == other.color
         && self.tex_coord == other.tex_coord
    }
}

// this assumes vertex data doesn't have any NaNs
impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos      [0].to_bits().hash(state);
        self.pos      [1].to_bits().hash(state);
        self.pos      [2].to_bits().hash(state);
        self.color    [0].to_bits().hash(state);
        self.color    [1].to_bits().hash(state);
        self.color    [2].to_bits().hash(state);
        self.tex_coord[0].to_bits().hash(state);
        self.tex_coord[1].to_bits().hash(state);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    model: Mat4,
    view:  Mat4,
    proj:  Mat4,
}





unsafe fn get_memory_type_index(
    instance:        &Instance,
    physical_device: vk::PhysicalDevice,
    properties:      vk::MemoryPropertyFlags,
    requirements:    vk::MemoryRequirements,
) -> Result<u32> {

    let memory = instance.instance().get_physical_device_memory_properties(physical_device);

    (0..memory.memory_heap_count)
        .find      (|i| {
            let suitable    = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];

            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type"))
}
