#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::mem::{self, size_of};
use std::rc::Rc;
use cgmath::{vec2, vec3};
use winit::application::ApplicationHandler;
use std::result::Result::Ok;
use anyhow::{Result, anyhow};
use winit::dpi::LogicalSize;
use winit::event::{WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::{window as vk_window};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use std::collections::HashSet;
use std::{u64};
use thiserror::Error;
use vulkanalia::bytecode::Bytecode;
use cgmath::{point3, Deg};
use std::time::Instant;
use std::fs::File;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::BufReader;


use std::ptr::copy_nonoverlapping as memcpy;

// Note: This trait was called `ExtDebugUtilsExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::{CommandBuffer, DescriptorSetLayout, ExtDebugUtilsExtensionInstanceCommands};

// Note: This trait was called `KhrSurfaceExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;

// Note: This trait was called `KhrSwapchainExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::KhrSwapchainExtensionDeviceCommands;

use crate::rendering::swapchain::depth_objects::DepthImage;
use crate::rendering::swapchain::{create_image_view, get_swapchain_extent};
use crate::rendering::texture_image::TextureImage;


type Vec2            = cgmath::Vector2<f32>;
type Vec3            = cgmath::Vector3<f32>;
type Mat4            = cgmath::Matrix4<f32>;

type VkAllocator<'a> = Option<&'a vk::AllocationCallbacks>;

pub const ALLOCATOR:                  VkAllocator          = None;

pub const PORTABILITY_MACOS_VERSION:  Version              = Version::new(1, 3, 216);

pub const VALIDATION_ENABLED:         bool                 = cfg!(debug_assertions);
pub const VALIDATION_LAYER:           vk::ExtensionName    = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

pub const DEVICE_EXTENSIONS:          &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

pub const MAX_FRAMES_IN_FLIGHT:       usize                = 2;

mod instance;
mod device;
mod swapchain;
mod texture_image;

pub fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Poll);


    let loader   = unsafe { LibloadingLoader::new(LIBRARY)? };
    let entry    = unsafe { Entry::new(loader).map_err(|b| anyhow!("{}", b))? };

    let mut app = AppWindow {
        window:   None,
        app:      None,
        minimize: false,
        entry:    Rc::new(entry),
    };

    event_loop.run_app(&mut app)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}


struct AppWindow {
    window:   Option<Window>,
    app:      Option<App>,
    minimize: bool,

    entry:    Rc<Entry>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title     ("Lightweaver")
                .with_inner_size(LogicalSize::new(1024, 768))
        ).unwrap();

        let app = unsafe { App::create(&window, self.entry.clone()) }.unwrap();

        self.window = Some(window);
        self.app    = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id:  winit::window    ::WindowId,
        event:       WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {

                if let Some(mut app) = self.app.take() {
                    unsafe { app.destroy(); }
                }

                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

                let (Some(app), Some(window)) = (self.app.as_mut(), self.window.as_ref()) else {
                    return;
                };

                unsafe { app.render(window).unwrap(); }

                window.request_redraw();
            },
            _ => (),

        }
    }
}




// Make sure to initialize all members in create()
#[derive(Debug)]
struct App {
    instance: Rc<instance::Instance>,
    device:   Rc<device  ::Device>,
    data:     AppData,

    frame:    usize,
    resized:  bool,
    start:    Instant,

}

/// The Vulkan handles and associated properties used by our Vulkan app.
// #[derive(Clone, Debug, Default)]
#[derive(Debug)]
struct AppData {
    messenger:                 vk::DebugUtilsMessengerEXT,
    physical_device:           vk::PhysicalDevice,
    graphics_queue:            vk::Queue,
    present_queue:             vk::Queue,
    surface:                   vk::SurfaceKHR,

    swapchain:                 swapchain::Swapchain,


    descriptor_set_layout:     vk::DescriptorSetLayout,

    frame_buffers:             Vec<vk::Framebuffer>,
    command_pool:              vk::CommandPool,

    texture_image:             TextureImage,

    vertices:                  Vec<Vertex>,
    indices:                   Vec<Index>,
    vertex_buffer:             Buffer,
    index_buffer:              Buffer,

    sync_objects:              SyncObjects,

}



impl App {
    unsafe fn create(window: &Window, entry: Rc<Entry>) -> Result<Self> {

        let (instance, messenger) = instance::Instance::new(window, entry.clone())?;

        let surface         = vk_window::create_surface(&instance, &window, &window)?;
        let physical_device = pick_physical_device     (&instance, surface)?;

        let device            = device::Device::new(entry.clone(), instance.clone(), physical_device, surface)?;
        let swapchain_support = SwapchainSupport::get(&instance, surface, device.physical_device)?;
        let extent            = get_swapchain_extent(window, &swapchain_support.capabilities);


        let graphics_queue             = device.graphics_queue;
        let present_queue              = device.present_queue;
        let descriptor_set_layout      = create_descriptor_set_layout(&device)?;
        let command_pool               = create_command_pool         (&device, &instance,   surface)?;
        let texture_image              = TextureImage        ::new   (device.clone(), instance.clone(), command_pool, graphics_queue)?;
        let (vertices, indices)        = load_model                  ()?;
        let vertex_buffer              = create_vertex_buffer        (&device, &instance, command_pool, graphics_queue, &vertices)?;
        let index_buffer               = create_index_buffer         (&device, &instance, command_pool, graphics_queue, &indices)?;
        let (swapchain, frame_buffers) = swapchain::Swapchain::create(device.clone(), instance.clone(), surface, descriptor_set_layout, &texture_image, command_pool, &vertex_buffer, &index_buffer, &indices, swapchain_support, extent)?;
        let sync_objects               = create_sync_objects         (&device, &swapchain)?;

        Ok(Self {
            frame:    0,
            resized:  false,
            start:    Instant::now(),

            instance: instance,
            device:   device,

            data:     AppData {
                messenger,
                physical_device,
                graphics_queue,
                present_queue,
                surface,
                swapchain,
                descriptor_set_layout,
                frame_buffers,
                command_pool,
                texture_image,
                vertices,
                indices,
                vertex_buffer,
                index_buffer,
                sync_objects,
            }
        })
    }


    unsafe fn destroy(&mut self) {
        println!("start destroy");
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();
        self.device.destroy_sampler              (self.data.texture_image.sampler, ALLOCATOR);
        self.device.destroy_image_view           (self.data.texture_image.view,    ALLOCATOR);
        self.device.destroy_image                (self.data.texture_image.image,   ALLOCATOR);
        self.device.free_memory                  (self.data.texture_image.memory,  ALLOCATOR);
        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, ALLOCATOR);

        self.device.destroy_buffer(self.data.vertex_buffer.buffer, ALLOCATOR);
        self.device.free_memory   (self.data.vertex_buffer.memory, ALLOCATOR);
        self.device.destroy_buffer(self.data.index_buffer .buffer, ALLOCATOR);
        self.device.free_memory   (self.data.index_buffer .memory, ALLOCATOR);

        self.data.sync_objects.in_flight_fences         .iter().for_each(|f| self.device.destroy_fence    (*f, ALLOCATOR));
        self.data.sync_objects.render_finished_sempahore.iter().for_each(|s| self.device.destroy_semaphore(*s, ALLOCATOR));
        self.data.sync_objects.image_available_sempahore.iter().for_each(|s| self.device.destroy_semaphore(*s, ALLOCATOR));

        self.device.destroy_command_pool(self.data.command_pool, ALLOCATOR);

        // self.device.destroy_device(ALLOCATOR);

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, ALLOCATOR);
        }

        self.instance.destroy_surface_khr(self.data.surface, ALLOCATOR);
        println!("done destroy");
        // self.instance.destroy_instance   (ALLOCATOR);
    }

    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {

        self.device.device_wait_idle()?;
        self.destroy_swapchain();


        let support = mem::take(&mut self.data.swapchain.support);
        let extent  = mem::take(&mut self.data.swapchain.extent);

        self.data.swapchain = swapchain::Swapchain::new(
             self.device  .clone(),
             self.instance.clone(),
             self.data.surface,
             self.data.descriptor_set_layout,
            &self.data.texture_image,
            support,
            extent,
        )?;

        Ok(())
    }


    // todo: move this to swapchain's drop
    unsafe fn destroy_swapchain(&mut self) {
        self.device.destroy_image_view(self.data.swapchain.depth_image.view,   ALLOCATOR);
        self.device.free_memory       (self.data.swapchain.depth_image.memory, ALLOCATOR);
        self.device.destroy_image     (self.data.swapchain.depth_image.image,  ALLOCATOR);

        self.device.destroy_descriptor_pool(self.data.swapchain.descriptor_pool, ALLOCATOR);

        self.data.swapchain.uniform_buffers.iter().for_each(|b| {
            self.device.destroy_buffer     (*&b.buffer, ALLOCATOR);
            self.device.free_memory        (*&b.memory, ALLOCATOR)
        });

        self.data.frame_buffers.iter().for_each(|f| self.device.destroy_framebuffer(*f, ALLOCATOR));

        self.device.free_command_buffers(self.data.command_pool, &self.data.swapchain.command_buffers);

        self.device.destroy_pipeline       (self.data.swapchain.pipeline.pipeline,       ALLOCATOR);
        self.device.destroy_pipeline_layout(self.data.swapchain.pipeline.layout,         ALLOCATOR);
        self.device.destroy_render_pass    (self.data.swapchain.render_pass.render_pass, ALLOCATOR);

        self.data.swapchain.image_views.iter().for_each(|v| self.device.destroy_image_view(*v, ALLOCATOR));

        self.device.destroy_swapchain_khr(self.data.swapchain.swapchain, ALLOCATOR);
    }
}


impl App {

    unsafe fn render(&mut self, window: &Window) -> Result<()> {

        self.device.wait_for_fences(
            &[self.data.sync_objects.in_flight_fences[self.frame]],
            true,
            u64::MAX
        )?;

        let result = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain.swapchain,
                u64::MAX,
                self.data.sync_objects.image_available_sempahore[self.frame],
                vk::Fence::null()
            )
        ;

        let image_index = match result {
            Ok ((image_index, _))               => image_index as usize,

            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e)                              => return Err(anyhow!(e)),
        };


        if !self.data.sync_objects.images_in_flight[image_index].is_null() {
            self.device.wait_for_fences(
                &[self.data.sync_objects.images_in_flight[image_index]],
                true,
                u64::MAX
            )?;
        }

        self.data.sync_objects.images_in_flight[image_index as usize] = self.data.sync_objects.in_flight_fences[self.frame];

        self.update_uniform_buffer(image_index)?;

        let wait_semaphores   = &[self.data.sync_objects.image_available_sempahore[self.frame]];
        let wait_stages       = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers   = &[self.data.swapchain.command_buffers[image_index]];
        let signal_semaphores = &[self.data.sync_objects.render_finished_sempahore[self.frame]];
        let submit_info       = vk::SubmitInfo::builder()
            .wait_semaphores    (wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers    (command_buffers)
            .signal_semaphores  (signal_semaphores)
        ;

        self.device.reset_fences(&[self.data.sync_objects.in_flight_fences[self.frame]])?;

        self.device.queue_submit(
            self.data.graphics_queue,
            &[submit_info],
            self.data.sync_objects.in_flight_fences[self.frame]
        )?;

        let swapchains    = &[self.data.swapchain.swapchain];
        let image_indices = &[image_index as u32];
        let present_info  = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains     (swapchains)
            .image_indices  (image_indices)
        ;

        let result  = self.device.queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);

        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain(window)?;
        }
        else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        self.frame = (self.frame +1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {

        let time = self.start.elapsed().as_secs_f32();

        let model = Mat4::from_axis_angle(
            vec3(0.0, 0.0, 1.0),
            Deg(90.0) * time,
        );

        let view = Mat4::look_at_rh(
            point3(2.0, 2.0, 2.0),
            point3(0.0, 0.0, 0.0),
            vec3  (0.0, 0.0, 1.0),
        );

        // correct cgmath's output from openGL to vulkan
        let correction = Mat4::new(
            1.0,  0.0,       0.0, 0.0,
            0.0, -1.0,       0.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 0.0,
            0.0,  0.0, 1.0 / 2.0, 1.0,
        );

        let proj = correction * cgmath::perspective(
            Deg(45.0),
            self.data.swapchain.extent.width as f32 / self.data.swapchain.extent.height as f32,
            0.1,
            10.0,
        );


        let ubo = UniformBufferObject { model, view, proj };


        let memory = self.device.map_memory(
            self.data.swapchain.uniform_buffers[image_index].memory,
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device.unmap_memory(self.data.swapchain.uniform_buffers[image_index].memory);

        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos:        Vec3,
    color:      Vec3,
    tex_coord:  Vec2,
}

type Index = u32;

impl Vertex {
    const fn new(pos: Vec3, color: Vec3, tex_coord: Vec2) -> Self {
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


#[derive(Clone, Copy, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present:  u32,

}

#[derive(Clone, Debug, Default)]
struct SwapchainSupport {
    capabilities:  vk::SurfaceCapabilitiesKHR,
    formats:       Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}


#[derive(Debug, Error)]
#[error("Missing {0}")]
struct SuitabilityError(&'static str);



unsafe fn pick_physical_device(instance: &Instance, surface: vk::SurfaceKHR) -> Result<vk::PhysicalDevice> {

    for physical_device in instance.enumerate_physical_devices()? {

        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, surface, physical_device) {
            warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
        }
        else {
            info!("Selected physical device (`{}`)", properties.device_name);
            return Ok(physical_device);
        }
    }

    Err(anyhow!("Failed to find suitable physical device"))
}

unsafe fn check_physical_device(
    instance:        &Instance,
    surface:         vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
)
    -> Result<()>
{
    QueueFamilyIndices::get(instance, surface, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, surface, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support")));
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.sampler_anisotropy != vk::TRUE {
        return Err(anyhow!(SuitabilityError("No sampler anisotropy")));
    }


    Ok(())
}


unsafe fn check_physical_device_extensions(
    instance:        &Instance,
    physical_device: vk::PhysicalDevice,
)
    -> Result<()>
{
    let extensions: HashSet<_> = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect()
    ;

    if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    }
    else {
        Err(anyhow!(SuitabilityError("Missing required device extensions")))
    }
}






unsafe fn create_descriptor_set_layout(
    device: &Device,
) -> Result<DescriptorSetLayout> {

    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding         (0)
        .descriptor_type (vk::DescriptorType  ::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags     (vk::ShaderStageFlags::VERTEX)
    ;

    let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding         (1)
        .descriptor_type (vk::DescriptorType  ::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags     (vk::ShaderStageFlags::FRAGMENT)
    ;


    let bindings = &[ubo_binding, sampler_binding];
    let info     = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(bindings)
    ;

    Ok(device.create_descriptor_set_layout(&info, ALLOCATOR)?)
}

unsafe fn create_command_pool(
    device:   &device::Device,
    instance: &Instance,
    surface:  vk::SurfaceKHR
)
    -> Result<vk::CommandPool>
{

    let indicies = QueueFamilyIndices::get(instance, surface, device.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags             (vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indicies.graphics)
    ;

    Ok(device.create_command_pool(&info, ALLOCATOR)?)
}


fn load_model() -> Result<(Vec<Vertex>, Vec<u32>)> {

    let mut reader = BufReader::new(File::open("resources/viking_room.obj")?);

    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions { triangulate: true, ..Default::default() },
        |_| Ok(Default::default()),
    )?;

    let mut unique_vertices = HashMap::new();
    let mut vertices        = Vec::new();
    let mut indices         = Vec::new();

    for model in &models {
        for index in &model.mesh.indices {
            let pos_offset       = (3 * index) as usize;
            let tex_coord_offset = (2 * index) as usize;

            let vertex = Vertex {
                pos: vec3(
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                color: vec3(1.0, 1.0, 1.0),
                tex_coord: vec2(
                          model.mesh.texcoords[tex_coord_offset],
                    1.0 - model.mesh.texcoords[tex_coord_offset +1],
                ),
            };

            if let Some(index) = unique_vertices.get(&vertex) {
                indices.push(*index as u32);
            }
            else {
                let index = vertices.len();

                unique_vertices.insert(vertex, index);
                vertices       .push  (vertex);
                indices        .push  (index as u32);
            }
        }
    }

    Ok((vertices, indices))
}


unsafe fn create_image(
    instance:   &Instance,
    device:     &device::Device,
    width:      u32,
    height:     u32,
    format:     vk::Format,
    tiling:     vk::ImageTiling,
    usage:      vk::ImageUsageFlags,
    properties: vk::MemoryPropertyFlags,
)
    -> Result<(vk::Image, vk::DeviceMemory)>
{
    let info = vk::ImageCreateInfo::builder()
        .image_type    (vk::ImageType::_2D)
        .extent        (vk::Extent3D { width, height, depth: 1 })
        .mip_levels    (1)
        .array_layers  (1)
        .format        (format)
        .tiling        (tiling)
        .initial_layout(vk::ImageLayout     ::UNDEFINED)
        .usage         (usage)
        .sharing_mode  (vk::SharingMode     ::EXCLUSIVE)
        .samples       (vk::SampleCountFlags::_1)
        .flags         (vk::ImageCreateFlags::empty())
    ;

    let image = device.create_image(&info, ALLOCATOR)?;

    let requirements = device.get_image_memory_requirements(image);
    let info         = vk::MemoryAllocateInfo::builder()
        .allocation_size  (requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            device.physical_device,
            properties,
            requirements
        )?)
    ;

    let image_memory = device.allocate_memory(&info, ALLOCATOR)?;

    device.bind_image_memory(image, image_memory, 0)?;

    Ok((image, image_memory))

}

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

    let command_buffer = device.allocate_command_buffers(&info)?[0];

    let info = vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
    ;

    device.begin_command_buffer(command_buffer, &info)?;

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
    device.end_command_buffer(command_buffer)?;

    let command_buffers = &[command_buffer];
    let info            = vk::SubmitInfo::builder()
        .command_buffers(command_buffers)
    ;

    device.queue_submit        (graphics_queue, &[info], vk::Fence::null())?;
    device.queue_wait_idle     (graphics_queue)?;

    device.free_command_buffers(command_pool, &[command_buffer]);

    Ok(())
}



unsafe fn create_vertex_buffer(
    device:         &device::Device,
    instance:       &Instance,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    vertices:       &[Vertex],
)
    -> Result<Buffer>
{

    let size = (size_of::<Vertex>() * vertices.len()) as u64;

    let staging_buffer = create_buffer(
        instance,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer.memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(vertices.as_ptr(), memory.cast(), vertices.len());

    device.unmap_memory(staging_buffer.memory);

    let vertex_buffer = create_buffer(
        instance,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(
        device,
        command_pool,
        graphics_queue,
        staging_buffer.buffer,
        vertex_buffer .buffer,
        size
    )?;

    device.destroy_buffer(staging_buffer.buffer, ALLOCATOR);
    device.free_memory   (staging_buffer.memory, ALLOCATOR);

    Ok(vertex_buffer)
}


unsafe fn create_index_buffer(
    device:         &device::Device,
    instance:       &Instance,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    indices:        &[Index],

) -> Result<Buffer> {

    let size = (size_of::<Index>() * indices.len()) as u64;

    let staging_buffer = create_buffer(
        instance,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer.memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(indices.as_ptr(), memory.cast(), indices.len());

    device.unmap_memory(staging_buffer.memory);

    let index_buffer = create_buffer(
        instance,
        device,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(device, command_pool, graphics_queue, staging_buffer.buffer, index_buffer.buffer, size)?;

    device.destroy_buffer(staging_buffer.buffer, ALLOCATOR);
    device.free_memory   (staging_buffer.memory, ALLOCATOR);

    Ok(index_buffer)
}



unsafe fn copy_buffer(
    device:         &Device,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    source:         vk::Buffer,
    destination:    vk::Buffer,
    size:           vk::DeviceSize,
) -> Result<()> {

    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, command_pool, command_buffer, graphics_queue)?;

    Ok(())
}


unsafe fn create_buffer(
    instance:   &Instance,
    device:     &device::Device,
    size:       vk::DeviceSize,
    usage:      vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,

) -> Result<Buffer> {

    let buffer_info = vk::BufferCreateInfo::builder()
        .size        (size)
        .usage       (usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
    ;


    let buffer       = device.create_buffer(&buffer_info, ALLOCATOR)?;
    let requirements = device.get_buffer_memory_requirements(buffer);

    let memory_info  = vk::MemoryAllocateInfo::builder()
        .allocation_size  (requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            device.physical_device,
            properties,
            requirements,
        )?)
    ;

    let buffer_memory = device.allocate_memory(&memory_info, ALLOCATOR)?;

    device.bind_buffer_memory(buffer, buffer_memory, 0)?;

    Ok(Buffer {
        buffer,
        memory: buffer_memory,
    })
}

unsafe fn transition_image_layout(
    device:         &Device,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    image:          vk::Image,
    _format:        vk::Format,
    old_layout:     vk::ImageLayout,
    new_layout:     vk::ImageLayout,

)
    -> Result<()>
{

    let (
        src_access_mask,
        dst_access_mask,
        src_stage_mask,
        dst_stage_mask,
    ) = match (old_layout, new_layout) {
        (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
            vk::AccessFlags       ::empty(),
            vk::AccessFlags       ::TRANSFER_WRITE,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
        ),
        (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
            vk::AccessFlags       ::TRANSFER_WRITE,
            vk::AccessFlags       ::SHADER_READ,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
        ),
        _ => return Err(anyhow!("Unsupported image layout transition!")),
    };


    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let subresource = vk::ImageSubresourceRange::builder()
        .aspect_mask     (vk::ImageAspectFlags::COLOR)
        .base_mip_level  (0)
        .level_count     (1)
        .base_array_layer(0)
        .layer_count     (1)
    ;


    let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout            (old_layout)
        .new_layout            (new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image                 (image)
        .subresource_range     (subresource)
        .src_access_mask       (src_access_mask)
        .dst_access_mask       (dst_access_mask)
    ;

    device.cmd_pipeline_barrier(
        command_buffer,
        src_stage_mask,
        dst_stage_mask,
        vk::DependencyFlags   ::empty(),
        &[] as &[vk::MemoryBarrier],
        &[] as &[vk::BufferMemoryBarrier],
        &[barrier],
    );


    end_single_time_commands(device, command_pool, command_buffer, graphics_queue)?;

    Ok(())
}

unsafe fn copy_buffer_to_image(
    device:         &Device,
    command_pool:   vk::CommandPool,
    graphics_queue: vk::Queue,
    buffer:         vk::Buffer,
    image:          vk::Image,
    width:          u32,
    height:         u32,
)
    -> Result<()>
{
    let command_buffer = begin_single_time_commands(device, command_pool)?;

    let subresource = vk::ImageSubresourceLayers::builder()
        .aspect_mask     (vk::ImageAspectFlags::COLOR)
        .mip_level       (0)
        .base_array_layer(0)
        .layer_count     (1)
    ;

    let region = vk::BufferImageCopy::builder()
        .buffer_offset      (0)
        .buffer_row_length  (0)
        .buffer_image_height(0)
        .image_subresource  (subresource)
        .image_offset       (vk::Offset3D { x: 0,  y: 0,   z: 0 })
        .image_extent       (vk::Extent3D { width, height, depth: 1 })
    ;

    device.cmd_copy_buffer_to_image(
        command_buffer,
        buffer,
        image,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        &[region],
    );


    end_single_time_commands(device, command_pool, command_buffer, graphics_queue)?;

    Ok(())
}


unsafe fn get_memory_type_index(
    instance:        &Instance,
    physical_device: vk::PhysicalDevice,
    properties:      vk::MemoryPropertyFlags,
    requirements:    vk::MemoryRequirements,
) -> Result<u32> {

    let memory = instance.get_physical_device_memory_properties(physical_device);

    (0..memory.memory_heap_count)
        .find      (|i| {
            let suitable    = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];

            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type"))
}

#[derive(Clone, Debug, Default)]
struct SyncObjects {
    pub image_available_sempahore: Vec<vk::Semaphore>,
    pub render_finished_sempahore: Vec<vk::Semaphore>,

    pub in_flight_fences:          Vec<vk::Fence>,
    pub images_in_flight:          Vec<vk::Fence>,
}

unsafe fn create_sync_objects(device: &Device, swapchain: &swapchain::Swapchain)
    -> Result<SyncObjects>
{
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info     = vk::FenceCreateInfo    ::builder()
        .flags(vk::FenceCreateFlags::SIGNALED)
    ;

    let mut sync_objects = SyncObjects::default();

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        sync_objects.image_available_sempahore.push(device.create_semaphore(&semaphore_info, ALLOCATOR)?);
        sync_objects.render_finished_sempahore.push(device.create_semaphore(&semaphore_info, ALLOCATOR)?);

        sync_objects.in_flight_fences         .push(device.create_fence    (&fence_info,     ALLOCATOR)?);
    }

    sync_objects.images_in_flight = swapchain.images
        .iter   ()
        .map    (|_| vk::Fence::null())
        .collect()
    ;

    Ok(sync_objects)
}



impl QueueFamilyIndices {
    unsafe fn get(
        instance:        &Instance,
        surface:         vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<Self>
    {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);
        let graphics   = properties
            .iter    ()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map     (|i| i as u32)
        ;

        let mut present = None;
        for (index, _properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(physical_device, index as u32, surface)? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        }
        else {
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }

    }
}


impl SwapchainSupport {
    unsafe fn get(
        instance:        &Instance,
        surface:         vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<Self>
    {
        Ok(Self {
            capabilities:  instance.get_physical_device_surface_capabilities_khr (physical_device, surface)?,
            formats:       instance.get_physical_device_surface_formats_khr      (physical_device, surface)?,
            present_modes: instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?,
        })
    }
}
