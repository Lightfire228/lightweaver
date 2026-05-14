mod buffer;
mod command_buffers;
mod command_pool;
mod depth_image;
mod descriptor_set_layout;
mod descriptor_sets;
mod device;
mod framebuffers;
mod image;
mod image_view;
mod instance;
mod pipeline;
mod render_pass;
mod surface;
mod swapchain;
mod sync_objects;
mod texture_image;
mod uniform_buffer;

use std::hash::{Hash, Hasher};
use std::mem::{self, ManuallyDrop};
use std::ptr::{copy_nonoverlapping as memcpy, drop_in_place};
use std::rc::Rc;
use std::time::Instant;

use anyhow::{Result, anyhow};
use cgmath::{Deg, point3, vec3};
use log::debug;
use vulkanalia::vk::{DeviceV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands};
use vulkanalia::{Entry, Instance as VkInstance, Version, vk};
use vulkanalia::loader::{LIBRARY, LibloadingLoader};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};


use crate::app::buffer::Buffer;
use crate::app::command_pool::CommandPool;
use crate::app::device::{Device, SuitabilityError};
use crate::app::instance::Instance;
use crate::app::render_pass::RenderPass;
use crate::app::surface::Surface;
use crate::app::swapchain::{Swapchain, SwapchainOpts};
use crate::app::descriptor_set_layout::DescriptorSetLayout;
use crate::app::sync_objects::{SyncObjects};
use crate::app::texture_image::TextureImage;
use crate::shapes::{Cube, Mesh, Shape};

pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;


pub const PORTABILITY_MACOS_VERSION:  Version              = Version::new(1, 3, 216);

pub const VALIDATION_ENABLED:         bool                 = cfg!(debug_assertions);
pub const VALIDATION_LAYER:           vk::ExtensionName    = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub const DEVICE_EXTENSIONS:          &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

pub const MAX_FRAMES_IN_FLIGHT:       usize                = 2;


pub fn main(shapes: Vec<Shape>) -> Result<()> {

    pretty_env_logger::init();

    let loader   = unsafe { LibloadingLoader::new(LIBRARY)? };
    let entry    = unsafe { Entry::new(loader).map_err(|b| anyhow!("{}", b))? };


    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        entry: Rc::new(entry),
        state: None,
        shapes,
    };
    event_loop.run_app(&mut app)?;

    Ok(())

}


struct App {
    entry:  Rc<Entry>,
    state:  Option<AppState>,
    shapes: Vec<Shape>,
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
            AppState::new(window, self.entry.clone(), &self.shapes).unwrap()
        )

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.state = None;

                event_loop.exit();
            },
            WindowEvent::Resized(_) => {

                if let Some(state) = self.state.as_mut() {
                    state.resized = true;
                }
            }
            WindowEvent::RedrawRequested => {
                let state  = self.state.as_mut().unwrap();

                unsafe {
                    state.render().unwrap();
                }

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
    command_pool:          Rc<CommandPool>,
    surface:               Surface,
    vertex_buffer:         Buffer,
    index_buffer:          Buffer,
    indices:               Vec<Index>,
    swapchain:             ManuallyDrop<Swapchain>,
    descriptor_set_layout: DescriptorSetLayout,
    texture_image:         TextureImage,
    sync_objects:          SyncObjects,
    start:                 Instant,
    frame:                 usize,
    resized:               bool,
}

impl AppState {
    pub fn new(window: Window, entry: Rc<Entry>, shapes: &[Shape]) -> Result<Self> {
        type DSLayout = DescriptorSetLayout;

        let instance                 = Instance    ::new(&window,          entry   )?;
        let surface                  = Surface     ::new(instance.clone(), &window )?;
        let device                   = Device      ::new(instance.clone(), &surface)?;
        let descriptor_set_layout    = DSLayout    ::new(device  .clone())?;
        let command_pool             = CommandPool ::new(device  .clone(), &instance, &surface)?;
        let texture_image            = TextureImage::new(device  .clone(), &instance, &command_pool, device.graphics_queue)?;

        let (vertices, indices) = load_shapes(&shapes)?;

        let vertex_buffer = unsafe {
            create_vertex_buffer(device.clone(), &instance, &command_pool, device.graphics_queue, &vertices)?
        };

        let index_buffer = unsafe {
            create_index_buffer(device.clone(), &instance, &command_pool, device.graphics_queue, &indices)?
        };

        let opts = SwapchainOpts {
            window:                &window,
            instance:              &instance,
            surface:               &surface,
            texture_image:         &texture_image,
            descriptor_set_layout: &descriptor_set_layout,
            vertex_buffer:         &vertex_buffer,
            index_buffer:          &index_buffer,
            indices:               &indices,
        };
        let swapchain    = Swapchain  ::new(device.clone(), command_pool.clone(), opts)?;
        let sync_objects = SyncObjects::new(device.clone(), swapchain.images())?;

        Ok(Self {
            window,
            instance,
            device,
            surface,
            swapchain: ManuallyDrop::new(swapchain),
            descriptor_set_layout,
            command_pool,
            vertex_buffer,
            index_buffer,
            indices,
            texture_image,
            sync_objects,
            start:   Instant::now(),
            frame:   0,
            resized: false,
        })
    }


    pub unsafe fn render(&mut self) -> Result<()> {

        let d = self.device.device();

        d.wait_for_fences(
            &[self.sync_objects.in_flight_fences[self.frame]],
            true,
            u64::MAX
        )?;

        let result = self
            .device
            .device()
            .acquire_next_image_khr(
                self.swapchain.swapchain(),
                u64::MAX,
                self.sync_objects.image_available_sempahore[self.frame],
                vk::Fence::null()
            )
        ;

        let image_index = match result {
            Ok ((image_index, _))               => image_index as usize,

            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(),
            Err(e)                              => return Err(anyhow!(e)),
        };


        if !self.sync_objects.images_in_flight[image_index].is_null() {
            d.wait_for_fences(
                &[self.sync_objects.images_in_flight[image_index]],
                true,
                u64::MAX
            )?;
        }

        self.sync_objects.images_in_flight[image_index as usize] = self.sync_objects.in_flight_fences[self.frame];

        self.update_uniform_buffer(image_index)?;

        let wait_semaphores   = &[self.sync_objects.image_available_sempahore[self.frame]];
        let wait_stages       = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers   = &[self.swapchain.command_buffers()[image_index]];
        let signal_semaphores = &[self.sync_objects.render_finished_sempahore[self.frame]];
        let submit_info       = vk::SubmitInfo::builder()
            .wait_semaphores    (wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers    (command_buffers)
            .signal_semaphores  (signal_semaphores)
        ;

        d.reset_fences(&[self.sync_objects.in_flight_fences[self.frame]])?;

        d.queue_submit(
            self.device.graphics_queue,
            &[submit_info],
            self.sync_objects.in_flight_fences[self.frame]
        )?;

        let swapchains    = &[self.swapchain.swapchain()];
        let image_indices = &[image_index as u32];
        let present_info  = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains     (swapchains)
            .image_indices  (image_indices)
        ;

        let result  = d.queue_present_khr(self.device.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain()?;
        }
        else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        self.frame = (self.frame +1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    unsafe fn recreate_swapchain(&mut self) -> Result<()> {

        self.device.device().device_wait_idle()?;



        let opts = SwapchainOpts {
            window:                &self.window,
            instance:              &self.instance,
            surface:               &self.surface,
            texture_image:         &self.texture_image,
            descriptor_set_layout: &self.descriptor_set_layout,
            vertex_buffer:         &self.vertex_buffer,
            index_buffer:          &self.index_buffer,
            indices:               &self.indices,
        };

        ManuallyDrop::drop(&mut self.swapchain);

        self.swapchain = ManuallyDrop::new(Swapchain::new(
             self.device      .clone(),
             self.command_pool.clone(),
             opts,
        )?);

        Ok(())
    }


    unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {

        let time = self.start.elapsed().as_secs_f32();

        // from model space to world space
        let model = Mat4::from_axis_angle(
            vec3(1.0, 1.0, 1.0),
            Deg(90.0) * time,
        );

        // world space to view space (camera)
        // TODO: setting the eye.Y to 0 causes the image to dissapear
        let view = Mat4::look_at_rh(
            point3(2.0, 2.0, 10.0),
            // point3(0.0, 0.01, 10.0),
            point3(0.0, 0.0,  0.0),
            vec3  (0.0, 0.0,  1.0),
        );

        // correct cgmath's output from openGL to vulkan
        let correction = Mat4::new(
            1.0,  0.0,       0.0, 0.0,
            0.0, -1.0,       0.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 1.0,
        );

        // let proj = correction * cgmath::perspective(
        // // let proj = cgmath::perspective(
        //     Deg(45.0),
        //     self.swapchain.extent().width as f32 / self.swapchain.extent().height as f32,
        //     1.0,
        //     100.0,
        // );


        let proj = correction * cgmath::ortho(
            - 10.0,  10.0,
            - 10.0,  10.0,
            -100.0, 100.0,
        );

        let ubo = UniformBufferObject { model, view, proj };


        let memory = self.device.device().map_memory(
            self.swapchain.uniform_buffers()[image_index].memory(),
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device.device().unmap_memory(self.swapchain.uniform_buffers()[image_index].memory());

        Ok(())
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        debug!("Dropping App State");

        unsafe {
            self.device.device().device_wait_idle().unwrap();

            ManuallyDrop::drop(&mut self.swapchain);
        }

        debug!("/Dropping App State");
    }
}



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

fn load_shapes(shapes: &[Shape]) -> Result<(Vec<Vertex>, Vec<u32>)> {

    // let mut unique_vertices = HashMap::new();

    let mut vertices        = Vec::new();
    let mut indices         = Vec::new();

    for (i, shape) in shapes.iter().enumerate() {
        let mesh: Mesh = Cube {}.into();

        // quad
        //     .vertices
        //     .iter_mut()
        //     .for_each(|v| v.pos.z -= i as f32 * 0.1)
        // ;

        vertices.extend(mesh.vertices.iter());
        // indices .extend(quad.indices .iter().map(|x| (i * quad.indices.len()) as u32 + x));
        indices .extend(mesh.indices .iter());

        break;
    }

    println!("count {}", indices.len());



    Ok((vertices, indices))
}


// TODO:
unsafe fn create_vertex_buffer(
    device:         Rc<Device>,
    instance:       &Instance,
    command_pool:   &CommandPool,
    graphics_queue: vk::Queue,
    vertices:       &[Vertex],
)
    -> Result<Buffer>
{
    let d = device.device();

    let size = (size_of::<Vertex>() * vertices.len()) as u64;

    let staging_buffer = Buffer::create_buffer(
        device.clone(),
        instance,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        "Staging"
    )?;

    let memory = d.map_memory(
        staging_buffer.memory(),
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(vertices.as_ptr(), memory.cast(), vertices.len());

    d.unmap_memory(staging_buffer.memory());

    let vertex_buffer = Buffer::create_buffer(
        device.clone(),
        instance,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        "Vertex"
    )?;

    copy_buffer(
        &device,
        command_pool,
        graphics_queue,
        staging_buffer.buffer(),
        vertex_buffer .buffer(),
        size
    )?;


    Ok(vertex_buffer)
}


unsafe fn create_index_buffer(
    device:         Rc<Device>,
    instance:       &Instance,
    command_pool:   &CommandPool,
    graphics_queue: vk::Queue,
    indices:        &[Index],

) -> Result<Buffer> {

    let d = device.device();

    let size = (size_of::<Index>() * indices.len()) as u64;

    let staging_buffer = Buffer::create_buffer(
        device.clone(),
        instance,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        "Staging"
    )?;

    let memory = d.map_memory(
        staging_buffer.memory(),
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(indices.as_ptr(), memory.cast(), indices.len());

    d.unmap_memory(staging_buffer.memory());

    let index_buffer = Buffer::create_buffer(
        device.clone(),
        instance,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
        "Index",
    )?;

    copy_buffer(
        &device,
        command_pool,
        graphics_queue,
        staging_buffer.buffer(),
        index_buffer  .buffer(),
        size
    )?;

    Ok(index_buffer)
}



unsafe fn copy_buffer(
    device:         &Device,
    command_pool:   &CommandPool,
    graphics_queue: vk::Queue,
    source:         vk::Buffer,
    destination:    vk::Buffer,
    size:           vk::DeviceSize,
) -> Result<()> {

    let command_buffer = begin_single_time_commands(device, command_pool.command_pool())?;

    let regions = vk::BufferCopy::builder().size(size);
    device.device().cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, command_pool.command_pool(), command_buffer, graphics_queue)?;

    Ok(())
}


// TODO: move these to the command buffer struct
unsafe fn begin_single_time_commands(
    device:       &Device,
    command_pool: vk::CommandPool
)
    -> Result<vk::CommandBuffer>
{
    let info = vk::CommandBufferAllocateInfo::builder()
        .level               (vk::CommandBufferLevel::PRIMARY)
        .command_pool        (command_pool)
        .command_buffer_count(1)
    ;

    let command_buffer = device.device().allocate_command_buffers(&info)?[0];

    let info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
    ;

    device.device().begin_command_buffer(command_buffer, &info)?;

    Ok(command_buffer)
}

unsafe fn end_single_time_commands(
    device:         &Device,
    command_pool:   vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    graphics_queue: vk::Queue,
)
    -> Result<()>
{
    device.device().end_command_buffer(command_buffer)?;

    let command_buffers = &[command_buffer];
    let info            = vk::SubmitInfo::builder()
        .command_buffers(command_buffers)
    ;

    device.device().queue_submit        (graphics_queue, &[info], vk::Fence::null())?;
    device.device().queue_wait_idle     (graphics_queue)?;

    device.device().free_command_buffers(command_pool, &[command_buffer]);

    Ok(())
}
