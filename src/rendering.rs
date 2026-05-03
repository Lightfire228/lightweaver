#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use std::mem::{ManuallyDrop, MaybeUninit, size_of};
use cgmath::{vec2, vec3};
use winit::application::ApplicationHandler;
use std::result::Result::Ok;
use anyhow::{Result, anyhow};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};
use log::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::{window as vk_window};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::u64;
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
use vulkanalia::vk::{ExtDebugUtilsExtensionInstanceCommands};

// Note: This trait was called `KhrSurfaceExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;

// Note: This trait was called `KhrSwapchainExtension` in versions of `vulkanalia` prior to `v0.31.0`.
use vulkanalia::vk::KhrSwapchainExtensionDeviceCommands;


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
        entry:    &entry,
    };

    event_loop.run_app(&mut app)?;

    Ok(())
}

struct AppWindow<'a> {
    window:   Option<Window>,
    app:      Option<App<'a>>,
    minimize: bool,

    entry:    &'a Entry,
}

impl<'a> ApplicationHandler for AppWindow<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title     ("Lightweaver")
                .with_inner_size(LogicalSize::new(1024, 768))
        ).unwrap();

        let app = unsafe { App::create(&window, &self.entry) }.unwrap();

        self.window = Some(window);
        self.app    = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id:   winit::window    ::WindowId,
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




#[derive(Debug)]
struct App<'a> {
    instance: ManuallyDrop<instance::Instance<'a>>,
    device:   ManuallyDrop<device  ::Device  <'a>>,
    data:     AppData,

    frame:    usize,
    resized:  bool,
    start:    Instant,

}

/// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
    messenger:                 vk::DebugUtilsMessengerEXT,
    physical_device:           vk::PhysicalDevice,
    graphics_queue:            vk::Queue,
    present_queue:             vk::Queue,
    surface:                   vk::SurfaceKHR,

    swapchain:                 vk::SwapchainKHR,
    swapchain_images:          Vec<vk::Image>,
    swapchain_image_views:     Vec<vk::ImageView>,
    swapchain_format:          vk::Format,
    swapchain_extent:          vk::Extent2D,

    render_pass:               vk::RenderPass,
    descriptor_set_layout:     vk::DescriptorSetLayout,
    pipeline_layout:           vk::PipelineLayout,
    pipeline:                  vk::Pipeline,

    framebuffers:              Vec<vk::Framebuffer>,
    command_pool:              vk::CommandPool,
    command_buffers:           Vec<vk::CommandBuffer>,

    image_available_sempahore: Vec<vk::Semaphore>,
    render_finished_sempahore: Vec<vk::Semaphore>,

    in_flight_fences:          Vec<vk::Fence>,
    images_in_flight:          Vec<vk::Fence>,

    vertices:                  Vec<Vertex>,
    indices:                   Vec<u32>,
    vertex_buffer:             vk::Buffer,
    vertex_buffer_memory:      vk::DeviceMemory,
    index_buffer:              vk::Buffer,
    index_buffer_memory:       vk::DeviceMemory,

    uniform_buffers:           Vec<vk::Buffer>,
    uniform_buffers_memory:    Vec<vk::DeviceMemory>,
    descriptor_pool:           vk::DescriptorPool,
    descriptor_sets:           Vec<vk::DescriptorSet>,

    texture_image:             vk::Image,
    texture_image_memory:      vk::DeviceMemory,
    texture_image_view:        vk::ImageView,
    texture_sampler:           vk::Sampler,

    depth_image:               vk::Image,
    depth_image_memory:        vk::DeviceMemory,
    depth_image_view:          vk::ImageView,
}


impl<'a> App<'a> {
    unsafe fn create(window: &Window, entry: &'a Entry) -> Result<Self> {

        let mut this = MaybeUninit::<Self>::uninit();
        let     ptr  = this.as_mut_ptr();

        macro_rules! init {
            ($x:ident, $val:expr) => {{
                (&raw mut (*ptr).$x).write($val);
                &(*ptr).$x
            }};
        }
        macro_rules! init_mut {
            ($x:ident, $val:expr) => {{
                (&raw mut (*ptr).$x).write($val);
                &mut (*ptr).$x
            }};
        }

        let (instance, messenger) = instance::Instance::new(window, &entry)?;

        let     instance = init!    (instance, ManuallyDrop::new(instance));
        let mut data     = init_mut!(data,     AppData::default());

        data.messenger = messenger;


        data.surface = vk_window::create_surface(instance,    &window, &window)?;
        data.physical_device = pick_physical_device(instance, &mut data)?;

        // let device = create_logical_device(&entry, &instance, &mut data)?;
        let (device, graphics_queue, present_queue) = device::Device::new(&entry, instance, data.physical_device, data.surface)?;
        data.graphics_queue = graphics_queue;
        data.present_queue  = present_queue;

        let device = init!(device, ManuallyDrop::new(device));


        create_swapchain            (window,  instance, device, &mut data)?;
        create_swapchain_image_views(                   device, &mut data)?;
        create_render_pass          (         instance, device, &mut data)?;
        create_descriptor_set_layout(                   device, &mut data)?;
        create_pipeline             (                   device, &mut data)?;
        create_depth_objects        (         instance, device, &mut data)?;
        create_framebuffers         (                   device, &mut data)?;
        create_command_pool         (         instance, device, &mut data)?;
        create_texture_image        (         instance, device, &mut data)?;
        create_texture_image_view   (                   device, &mut data)?;
        create_texture_sampler      (                   device, &mut data)?;
        load_model                  (                           &mut data)?;
        create_vertex_buffer        (         instance, device, &mut data)?;
        create_index_buffer         (         instance, device, &mut data)?;
        create_uniform_buffers      (         instance, device, &mut data)?;
        create_descriptor_pool      (                   device, &mut data)?;
        create_descriptor_sets      (                   device, &mut data)?;
        create_command_buffers      (                   device, &mut data)?;
        create_sync_objects         (                   device, &mut data)?;

        _ = init!(frame,   0);
        _ = init!(resized, false);
        _ = init!(start,   Instant::now());

        println!("hi again");

        Ok(this.assume_init())
    }


    unsafe fn destroy(&mut self) {
        println!("start destroy");
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();
        self.device.destroy_sampler              (self.data.texture_sampler,       ALLOCATOR);
        self.device.destroy_image_view           (self.data.texture_image_view,    ALLOCATOR);
        self.device.destroy_image                (self.data.texture_image,         ALLOCATOR);
        self.device.free_memory                  (self.data.texture_image_memory,  ALLOCATOR);
        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, ALLOCATOR);

        self.device.destroy_buffer(self.data.vertex_buffer,        ALLOCATOR);
        self.device.free_memory   (self.data.vertex_buffer_memory, ALLOCATOR);
        self.device.destroy_buffer(self.data.index_buffer,         ALLOCATOR);
        self.device.free_memory   (self.data.index_buffer_memory,  ALLOCATOR);

        self.data.in_flight_fences         .iter().for_each(|f| self.device.destroy_fence    (*f, ALLOCATOR));

        self.data.render_finished_sempahore.iter().for_each(|s| self.device.destroy_semaphore(*s, ALLOCATOR));
        self.data.image_available_sempahore.iter().for_each(|s| self.device.destroy_semaphore(*s, ALLOCATOR));

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

        create_swapchain            (window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(                        &self.device, &mut self.data)?;
        create_render_pass          (        &self.instance, &self.device, &mut self.data)?;
        create_pipeline             (                        &self.device, &mut self.data)?;
        create_depth_objects        (        &self.instance, &self.device, &mut self.data)?;
        create_framebuffers         (                        &self.device, &mut self.data)?;
        create_uniform_buffers      (        &self.instance, &self.device, &mut self.data)?;
        create_descriptor_pool      (                        &self.device, &mut self.data)?;
        create_descriptor_sets      (                        &self.device, &mut self.data)?;
        create_command_buffers      (                        &self.device, &mut self.data)?;

        self.data
            .images_in_flight
            .resize(self.data.swapchain_images.len(), vk::Fence::null())
        ;

        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) {
        self.device.destroy_image_view(self.data.depth_image_view,   ALLOCATOR);
        self.device.free_memory       (self.data.depth_image_memory, ALLOCATOR);
        self.device.destroy_image     (self.data.depth_image,        ALLOCATOR);

        self.device.destroy_descriptor_pool(self.data.descriptor_pool, ALLOCATOR);

        self.data.uniform_buffers       .iter().for_each(|b| self.device.destroy_buffer     (*b, ALLOCATOR));
        self.data.uniform_buffers_memory.iter().for_each(|b| self.device.free_memory        (*b, ALLOCATOR));
        self.data.framebuffers          .iter().for_each(|f| self.device.destroy_framebuffer(*f, ALLOCATOR));

        self.device.free_command_buffers(self.data.command_pool, &self.data.command_buffers);

        self.device.destroy_pipeline       (self.data.pipeline,        ALLOCATOR);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, ALLOCATOR);
        self.device.destroy_render_pass    (self.data.render_pass,     ALLOCATOR);

        self.data.swapchain_image_views.iter().for_each(|v| self.device.destroy_image_view(*v, ALLOCATOR));

        self.device.destroy_swapchain_khr(self.data.swapchain, ALLOCATOR);
    }
}

impl<'a> Drop for App<'a> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self.instance);
        }
    }
}


impl<'a> App<'a> {

    unsafe fn render(&mut self, window: &Window) -> Result<()> {

        self.device.wait_for_fences(
            &[self.data.in_flight_fences[self.frame]],
            true,
            u64::MAX
        )?;

        let result = self
            .device
            .acquire_next_image_khr(
                self.data.swapchain,
                u64::MAX,
                self.data.image_available_sempahore[self.frame],
                vk::Fence::null()
            )
        ;

        let image_index = match result {
            Ok ((image_index, _))               => image_index as usize,

            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e)                              => return Err(anyhow!(e)),
        };


        if !self.data.images_in_flight[image_index].is_null() {
            self.device.wait_for_fences(
                &[self.data.images_in_flight[image_index]],
                true,
                u64::MAX
            )?;
        }

        self.data.images_in_flight[image_index as usize] = self.data.in_flight_fences[self.frame];

        self.update_uniform_buffer(image_index)?;

        let wait_semaphores   = &[self.data.image_available_sempahore[self.frame]];
        let wait_stages       = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers   = &[self.data.command_buffers[image_index]];
        let signal_semaphores = &[self.data.render_finished_sempahore[self.frame]];
        let submit_info       = vk::SubmitInfo::builder()
            .wait_semaphores    (wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers    (command_buffers)
            .signal_semaphores  (signal_semaphores)
        ;

        self.device.reset_fences(&[self.data.in_flight_fences[self.frame]])?;

        self.device.queue_submit(
            self.data.graphics_queue,
            &[submit_info],
            self.data.in_flight_fences[self.frame]
        )?;

        let swapchains    = &[self.data.swapchain];
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
            self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
            0.1,
            10.0,
        );


        let ubo = UniformBufferObject { model, view, proj };


        let memory = self.device.map_memory(
            self.data.uniform_buffers_memory[image_index],
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device.unmap_memory(self.data.uniform_buffers_memory[image_index]);

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

#[derive(Clone, Debug)]
struct SwapchainSupport {
    capabilities:  vk::SurfaceCapabilitiesKHR,
    formats:       Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}


#[derive(Debug, Error)]
#[error("Missing {0}")]
struct SuitabilityError(&'static str);



unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<vk::PhysicalDevice> {

    for physical_device in instance.enumerate_physical_devices()? {

        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
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
    data:            &AppData,
    physical_device: vk::PhysicalDevice,
)
    -> Result<()>
{
    QueueFamilyIndices::get(instance, data.surface, physical_device)?;
    check_physical_device_extensions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
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



fn get_swapchain_surface_format(
    formats: &[vk::SurfaceFormatKHR],
)
    -> vk::SurfaceFormatKHR
{
    formats
        .iter          ()
        .cloned        ()
        .find          (|f| {
                  f.format      == vk::Format       ::B8G8R8A8_SRGB
               && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_swapchain_present_mode(
    present_modes: &[vk::PresentModeKHR]
)
    -> vk::PresentModeKHR
{
    present_modes
        .iter     ()
        .cloned   ()
        .find     (|m| *m == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

fn get_swapchain_extent(
    window:       &Window,
    capabilities: &vk::SurfaceCapabilitiesKHR,
)
    -> vk::Extent2D
{
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    }
    else {
        vk::Extent2D::builder()
            .width (window.inner_size().width .clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ))
            .height(window.inner_size().height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ))
            .build()
    }
}

unsafe fn create_swapchain(
    window:   &Window,
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData
)
    -> Result<()>
{
    let indicies = QueueFamilyIndices::get(instance, data.surface, data.physical_device)?;
    let support  = SwapchainSupport  ::get(instance, data,         data.physical_device)?;

    let surface_format = get_swapchain_surface_format(&support.formats);
    let present_mode   = get_swapchain_present_mode  (&support.present_modes);
    let extent         = get_swapchain_extent        (window, &support.capabilities);


    let mut image_count = support.capabilities.min_image_count +1;
    if
           support.capabilities.max_image_count != 0
        && image_count > support.capabilities.max_image_count
    {
        image_count = support.capabilities.max_image_count;
    }


    let mut queue_family_indices = vec![];

    let image_sharing_mode = if indicies.graphics != indicies.present {
        queue_family_indices.push(indicies.graphics);
        queue_family_indices.push(indicies.present);

        vk::SharingMode::CONCURRENT
    }
    else {
        vk::SharingMode::EXCLUSIVE
    };

    let info = vk::SwapchainCreateInfoKHR::builder()
        .surface             (data.surface)
        .min_image_count     (image_count)
        .image_format        (surface_format.format)
        .image_color_space   (surface_format.color_space)
        .image_extent        (extent)
        .image_array_layers  (1)
        .image_usage         (vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode  (image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform       (support.capabilities.current_transform)
        .composite_alpha     (vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode        (present_mode)
        .clipped             (true)
        .old_swapchain       (vk::SwapchainKHR::null())
    ;

    data.swapchain        = device.create_swapchain_khr(&info, ALLOCATOR)?;
    data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;
    data.swapchain_format = surface_format.format;
    data.swapchain_extent = extent;

    Ok(())
}

unsafe fn create_swapchain_image_views(
    device: &Device,
    data:   &mut AppData,
)
    -> Result<()>
{
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map (|i| create_image_view(device, *i, data.swapchain_format, vk::ImageAspectFlags::COLOR))
        .collect::<Result<Vec<_>, _>>()?
    ;

    Ok(())
}


unsafe fn create_descriptor_set_layout(
    device: &Device,
    data:   &mut AppData,
) -> Result<()> {

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

    data.descriptor_set_layout = device.create_descriptor_set_layout(&info, ALLOCATOR)?;

    Ok(())
}

unsafe fn create_pipeline(device: &Device, data: &mut AppData) -> Result<()> {
    // TODO: compile shaders at runtime
    // https://stackoverflow.com/a/73591683
    let vert = include_bytes!("../shaders/vert.spv");
    let frag = include_bytes!("../shaders/frag.spv");

    let vert_shader_module = create_shader_module(device, &vert[..])?;
    let frag_shader_module = create_shader_module(device, &frag[..])?;

    let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage (vk::ShaderStageFlags::VERTEX)
        .module(vert_shader_module)
        .name  (b"main\0")
    ;
    let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage (vk::ShaderStageFlags::FRAGMENT)
        .module(frag_shader_module)
        .name  (b"main\0")
    ;

    let binding_descriptions   = [Vertex::binding_description()];
    let attribute_descriptions = Vertex::attribute_descriptions();
    let vertex_input_state     = vk::PipelineVertexInputStateCreateInfo  ::builder()
        .vertex_binding_descriptions  (&binding_descriptions)
        .vertex_attribute_descriptions(&attribute_descriptions)
    ;

    let input_assembly_state   = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology                (vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false)
    ;

    let viewport = vk::Viewport::builder()
        .x        (0.0)
        .y        (0.0)
        .width    (data.swapchain_extent.width  as f32)
        .height   (data.swapchain_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
    ;
    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D { x: 0, y: 0 })
        .extent(data.swapchain_extent)
    ;

    let viewports      = &[viewport];
    let scissors       = &[scissor];
    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(viewports)
        .scissors (scissors)
    ;

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable       (false)
        .rasterizer_discard_enable(false)
        .polygon_mode             (vk::PolygonMode::FILL)
        .line_width               (1.0)
        .cull_mode                (vk::CullModeFlags::BACK)
        .front_face               (vk::FrontFace    ::COUNTER_CLOCKWISE)
        .depth_bias_enable        (false)
    ;

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::_1)
    ;

    let attachment = vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask      (vk::ColorComponentFlags::all())
        .blend_enable          (false)

        // Optional
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ZERO)
        .color_blend_op        (vk::BlendOp    ::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
        .alpha_blend_op        (vk::BlendOp    ::ADD)
    ;

    let attachments = &[attachment];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op       (vk::LogicOp::COPY)
        .attachments    (attachments)
        .blend_constants([0.0, 0.0, 0.0, 0.0])
    ;


    let set_layouts = &[data.descriptor_set_layout];
    let layout_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(set_layouts)
    ;

    data.pipeline_layout = device.create_pipeline_layout(&layout_info, ALLOCATOR)?;

    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable       (true)
        .depth_write_enable      (true)
        .depth_compare_op        (vk::CompareOp::LESS)
        .depth_bounds_test_enable(false)
        .min_depth_bounds        (0.0) // Optional.
        .max_depth_bounds        (1.0) // Optional.
    ;

    let stages = &[vert_stage, frag_stage];
    let info   = vk::GraphicsPipelineCreateInfo::builder()
        .stages              (stages)
        .vertex_input_state  (&vertex_input_state)
        .input_assembly_state(&input_assembly_state)
        .viewport_state      (&viewport_state)
        .rasterization_state (&rasterization_state)
        .multisample_state   (&multisample_state)
        .depth_stencil_state (&depth_stencil_state)
        .color_blend_state   (&color_blend_state)
        .layout              (data.pipeline_layout)
        .render_pass         (data.render_pass)
        .subpass             (0)
        .base_pipeline_handle(vk::Pipeline::null())
        .base_pipeline_index (-1)
    ;

    data.pipeline = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], ALLOCATOR)?.0[0];

    device.destroy_shader_module(vert_shader_module, ALLOCATOR);
    device.destroy_shader_module(frag_shader_module, ALLOCATOR);

    Ok(())
}

unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {

    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|i| {
            // The color attachment differs for every swapchain image, but the same depth image can be
            // used by all of them because only a single subpass is running at the same time due to
            // our semaphores.
            let attachments = &[*i, data.depth_image_view];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass)
                .attachments(attachments)
                .width      (data.swapchain_extent.width)
                .height     (data.swapchain_extent.height)
                .layers     (1)
            ;

            device.create_framebuffer(&create_info, ALLOCATOR)
        })
        .collect::<Result<Vec<_>, _>>()?
    ;

    Ok(())
}

unsafe fn create_command_pool(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
)
    -> Result<()>
{

    let indicies = QueueFamilyIndices::get(instance, data.surface, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags             (vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indicies.graphics)
    ;

    data.command_pool = device.create_command_pool(&info, ALLOCATOR)?;

    Ok(())
}


unsafe fn create_depth_objects(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
)
    -> Result<()>
{

    let format = get_depth_format(instance, data)?;

    let (depth_image, depth_image_memory) = create_image(
        instance,
        device,
        data,
        data.swapchain_extent.width,
        data.swapchain_extent.height,
        format,
        vk::ImageTiling        ::OPTIMAL,
        vk::ImageUsageFlags    ::DEPTH_STENCIL_ATTACHMENT,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.depth_image        = depth_image;
    data.depth_image_memory = depth_image_memory;
    data.depth_image_view   = create_image_view(
        device,
        data.depth_image,
        format,
        vk::ImageAspectFlags::DEPTH,
    )?;

    Ok(())
}

unsafe fn get_supported_format(
    instance:   &Instance,
    data:       &AppData,
    candidates: &[vk::Format],
    tiling:     vk::ImageTiling,
    features:   vk::FormatFeatureFlags,
)
    -> Result<vk::Format>
{
    candidates
        .iter  ()
        .cloned()
        .find  (|f| {
            let properties = instance.get_physical_device_format_properties(
                data.physical_device,
                *f
            );

            match tiling {
                vk::ImageTiling::LINEAR  => properties.linear_tiling_features .contains(features),
                vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),

                _ => false
            }
        })
        .ok_or_else(|| anyhow!("Failed to find supported format"))
}

unsafe fn get_depth_format(instance: &Instance, data: &AppData) -> Result<vk::Format> {
    let candidates = &[
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    get_supported_format(
        instance,
        data,
        candidates,
        vk::ImageTiling       ::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}


unsafe fn create_texture_image(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
)
    -> Result<()>
{
    // let image  = File::open("resources/texture.png")?;
    let image  = File::open("resources/viking_room.png")?;

    let     decoder = png::Decoder::new(image);
    let mut reader  = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();

    if width != 1024 || height != 1024 || reader.info().color_type != png::ColorType::Rgba {
        panic!("Invalid texture image.");
    }

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags   ::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

    device.unmap_memory(staging_buffer_memory);

    let (texture_image, texture_image_memory) = create_image(
        instance,
        device,
        data,
        width,
        height,
        vk::Format             ::R8G8B8A8_SRGB,
        vk::ImageTiling        ::OPTIMAL,
        vk::ImageUsageFlags    ::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.texture_image        = texture_image;
    data.texture_image_memory = texture_image_memory;

    transition_image_layout(
        device,
        data,
        data.texture_image,
        vk::Format     ::R8G8B8A8_SRGB,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    )?;

    copy_buffer_to_image(
        device,
        data,
        staging_buffer,
        data.texture_image,
        width,
        height,
    )?;

    transition_image_layout(
        device,
        data,
        data.texture_image,
        vk::Format     ::R8G8B8A8_SRGB,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;

    device.destroy_buffer(staging_buffer,        ALLOCATOR);
    device.free_memory   (staging_buffer_memory, ALLOCATOR);

    Ok(())
}


fn load_model(data: &mut AppData) -> Result<()> {

    let mut reader = BufReader::new(File::open("resources/viking_room.obj")?);

    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions { triangulate: true, ..Default::default() },
        |_| Ok(Default::default()),
    )?;

    let mut unique_vertices = HashMap::new();

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
                data.indices.push(*index as u32);
            }
            else {
                let index = data.vertices.len();
                unique_vertices.insert(vertex, index);
                data.vertices  .push  (vertex);
                data.indices   .push  (index as u32);
            }
        }
    }

    Ok(())
}


unsafe fn create_texture_image_view(device: &Device, data: &mut AppData) -> Result<()> {

    data.texture_image_view = create_image_view(
        device,
        data.texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageAspectFlags::COLOR,
    )?;

    Ok(())
}

unsafe fn create_image_view(
    device:  &Device,
    image:   vk::Image,
    format:  vk::Format,
    aspects: vk::ImageAspectFlags
)
    -> Result<vk::ImageView>
{

    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask     (aspects)
        .base_mip_level  (0)
        .level_count     (1)
        .base_array_layer(0)
        .layer_count     (1)
    ;

    let info = vk::ImageViewCreateInfo::builder()
        .image            (image)
        .view_type        (vk::ImageViewType::_2D)
        .format           (format)
        .subresource_range(subresource_range)
    ;

    Ok(device.create_image_view(&info, ALLOCATOR)?)
}


unsafe fn create_texture_sampler(device: &Device, data: &mut AppData) -> Result<()> {

    let info = vk::SamplerCreateInfo::builder()
        .mag_filter              (vk::Filter            ::LINEAR)
        .min_filter              (vk::Filter            ::LINEAR)
        .address_mode_u          (vk::SamplerAddressMode::REPEAT)
        .address_mode_v          (vk::SamplerAddressMode::REPEAT)
        .address_mode_w          (vk::SamplerAddressMode::REPEAT)
        .anisotropy_enable       (true)
        .max_anisotropy          (16.0)
        .border_color            (vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .compare_enable          (false)
        .compare_op              (vk::CompareOp        ::ALWAYS)
        .mipmap_mode             (vk::SamplerMipmapMode::LINEAR)
        .mip_lod_bias            (0.0)
        .min_lod                 (0.0)
        .max_lod                 (0.0)
    ;

    data.texture_sampler = device.create_sampler(&info, ALLOCATOR)?;

    Ok(())
}


unsafe fn create_image(
    instance:   &Instance,
    device:     &Device,
    data:       &AppData,
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
            data,
            properties,
            requirements
        )?)
    ;

    let image_memory = device.allocate_memory(&info, ALLOCATOR)?;

    device.bind_image_memory(image, image_memory, 0)?;

    Ok((image, image_memory))

}

unsafe fn begin_single_time_commands(
    device: &Device,
    data:   &AppData,
)
    -> Result<vk::CommandBuffer>
{
    let info = vk::CommandBufferAllocateInfo::builder()
        .level               (vk::CommandBufferLevel::PRIMARY)
        .command_pool        (data.command_pool)
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
    data:           &AppData,
    command_buffer: vk::CommandBuffer,
)
    -> Result<()>
{
    device.end_command_buffer(command_buffer)?;

    let command_buffers = &[command_buffer];
    let info            = vk::SubmitInfo::builder()
        .command_buffers(command_buffers)
    ;

    device.queue_submit        (data.graphics_queue, &[info], vk::Fence::null())?;
    device.queue_wait_idle     (data.graphics_queue)?;

    device.free_command_buffers(data.command_pool, &[command_buffer]);

    Ok(())
}


unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {

    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool        (data.command_pool)
        .level               (vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(data.framebuffers.len() as u32)
    ;

    data.command_buffers = device.allocate_command_buffers(&allocate_info)?;

    for (i, command_buffer) in data.command_buffers.iter().enumerate() {
        let inheritance = vk::CommandBufferInheritanceInfo::builder();

        let info = vk::CommandBufferBeginInfo::builder()
            .flags           (vk::CommandBufferUsageFlags::empty())
            .inheritance_info(&inheritance)
        ;

        device.begin_command_buffer(*command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(data.swapchain_extent)
        ;

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            }
        };

        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth:   1.0,
                stencil: 0,
            }
        };

        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass (data.render_pass)
            .framebuffer (data.framebuffers[i])
            .render_area (render_area)
            .clear_values(clear_values)
        ;

        device.cmd_begin_render_pass   (*command_buffer, &info, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline       (*command_buffer, vk::PipelineBindPoint::GRAPHICS, data.pipeline);

        device.cmd_bind_vertex_buffers (*command_buffer, 0, &[data.vertex_buffer], &[0]);
        device.cmd_bind_index_buffer   (*command_buffer, data.index_buffer, 0, vk::IndexType::UINT32);
        device.cmd_bind_descriptor_sets(
            *command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            data.pipeline_layout,
            0,
            &[data.descriptor_sets[i]],
            &[],
        );
        device.cmd_draw_indexed        (*command_buffer, data.indices.len() as u32, 1, 0, 0, 0);

        device.cmd_end_render_pass     (*command_buffer);

        device.end_command_buffer      (*command_buffer)?;
    }

    Ok(())
}

unsafe fn create_vertex_buffer(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
) -> Result<()> {

    let size = (size_of::<Vertex>() * data.vertices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(data.vertices.as_ptr(), memory.cast(), data.vertices.len());

    device.unmap_memory(staging_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.vertex_buffer        = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    device.destroy_buffer(staging_buffer,        ALLOCATOR);
    device.free_memory   (staging_buffer_memory, ALLOCATOR);

    Ok(())
}


unsafe fn create_index_buffer(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
) -> Result<()> {

    let size = (size_of::<u32>() * data.indices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(data.indices.as_ptr(), memory.cast(), data.indices.len());

    device.unmap_memory(staging_buffer_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.index_buffer        = index_buffer;
    data.index_buffer_memory = index_buffer_memory;

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    device.destroy_buffer(staging_buffer,        ALLOCATOR);
    device.free_memory   (staging_buffer_memory, ALLOCATOR);

    Ok(())
}

unsafe fn create_uniform_buffers(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
) -> Result<()> {

    data.uniform_buffers       .clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        data.uniform_buffers       .push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}


unsafe fn create_descriptor_pool(device: &Device, data: &mut AppData) -> Result<()> {

    let ubo_size = vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(data.swapchain_images.len() as u32)
    ;

    let sampler_size =  vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(data.swapchain_images.len() as u32)
    ;

    let pool_sizes = &[ubo_size, sampler_size];
    let info       = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(pool_sizes)
        .max_sets  (data.swapchain_images.len() as u32)
    ;

    data.descriptor_pool = device.create_descriptor_pool(&info, ALLOCATOR)?;

    Ok(())
}

unsafe fn create_descriptor_sets(device: &Device, data: &mut AppData) -> Result<()> {

    let layouts = vec![data.descriptor_set_layout; data.swapchain_images.len()];
    let info    = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(data.descriptor_pool)
        .set_layouts    (&layouts)
    ;

    data.descriptor_sets = device.allocate_descriptor_sets(&info)?;

    for i in 0..data.swapchain_images.len() {
        let info = vk::DescriptorBufferInfo::builder()
            .buffer(data.uniform_buffers[i])
            .offset(0)
            .range(size_of::<UniformBufferObject>() as u64)
        ;

        let buffer_info = &[info];
        let ubo_write   = vk::WriteDescriptorSet::builder()
            .dst_set          (data.descriptor_sets[i])
            .dst_binding      (0)
            .dst_array_element(0)
            .descriptor_type  (vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info      (buffer_info)
        ;

        let info = vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view  (data.texture_image_view)
            .sampler     (data.texture_sampler)
        ;

        let image_info    = &[info];
        let sampler_write = vk::WriteDescriptorSet::builder()
            .dst_set          (data.descriptor_sets[i])
            .dst_binding      (1)
            .dst_array_element(0)
            .descriptor_type  (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info       (image_info)
        ;

        device.update_descriptor_sets(
            &[ubo_write, sampler_write],
            &[] as &[vk::CopyDescriptorSet]
        );
    }

    Ok(())
}


unsafe fn copy_buffer(
    device:      &Device,
    data:        &AppData,
    source:      vk::Buffer,
    destination: vk::Buffer,
    size:        vk::DeviceSize,
) -> Result<()> {

    let command_buffer = begin_single_time_commands(device, data)?;

    let regions = vk::BufferCopy::builder().size(size);
    device.cmd_copy_buffer(command_buffer, source, destination, &[regions]);

    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}


unsafe fn create_buffer(
    instance:   &Instance,
    device:     &Device,
    data:       &mut AppData,
    size:       vk::DeviceSize,
    usage:      vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {

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
            data,
            properties,
            requirements,
        )?)
    ;

    let buffer_memory = device.allocate_memory(&memory_info, ALLOCATOR)?;

    device.bind_buffer_memory(buffer, buffer_memory, 0)?;


    Ok((buffer, buffer_memory))
}

unsafe fn transition_image_layout(
    device:     &Device,
    data:       &AppData,
    image:      vk::Image,
    format:     vk::Format,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
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


    let command_buffer = begin_single_time_commands(device, data)?;

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


    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}

unsafe fn copy_buffer_to_image(
    device: &Device,
    data:   &AppData,
    buffer: vk::Buffer,
    image:  vk::Image,
    width:  u32,
    height: u32,
)
    -> Result<()>
{
    let command_buffer = begin_single_time_commands(device, data)?;

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


    end_single_time_commands(device, data, command_buffer)?;

    Ok(())
}


unsafe fn get_memory_type_index(
    instance:     &Instance,
    data:         &AppData,
    properties:   vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements,
) -> Result<u32> {

    let memory = instance.get_physical_device_memory_properties(data.physical_device);

    (0..memory.memory_heap_count)
        .find      (|i| {
            let suitable    = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];

            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type"))
}


unsafe fn create_sync_objects(device: &Device, data: &mut AppData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info     = vk::FenceCreateInfo    ::builder()
        .flags(vk::FenceCreateFlags::SIGNALED)
    ;

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_sempahore.push(device.create_semaphore(&semaphore_info, ALLOCATOR)?);
        data.render_finished_sempahore.push(device.create_semaphore(&semaphore_info, ALLOCATOR)?);

        data.in_flight_fences         .push(device.create_fence    (&fence_info,     ALLOCATOR)?);
    }

    data.images_in_flight = data.swapchain_images
        .iter   ()
        .map    (|_| vk::Fence::null())
        .collect()
    ;

    Ok(())
}

unsafe fn create_shader_module(
    device:   &Device,
    bytecode: &[u8],
)
    -> Result<vk::ShaderModule>
{
    let bytecode = Bytecode::new(bytecode).unwrap();

    let info = vk::ShaderModuleCreateInfo::builder()
        .code     (bytecode.code     ())
        .code_size(bytecode.code_size())
    ;

    Ok(device.create_shader_module(&info, ALLOCATOR)?)
}


unsafe fn create_render_pass(
    instance: &Instance,
    device:   &Device,
    data:     &mut AppData,
)
    -> Result<()>
{

    let depth_stencil_attachment = vk::AttachmentDescription::builder()
        .format          (get_depth_format(instance, data)?)
        .samples         (vk::SampleCountFlags ::_1)
        .load_op         (vk::AttachmentLoadOp ::CLEAR)
        .store_op        (vk::AttachmentStoreOp::DONT_CARE)
        .stencil_load_op (vk::AttachmentLoadOp ::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout  (vk::ImageLayout      ::UNDEFINED)
        .final_layout    (vk::ImageLayout      ::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
    ;

    let depth_stencil_attachment_ref = vk::AttachmentReference::builder()
        .attachment(1)
        .layout    (vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
    ;

    let color_attachment = vk::AttachmentDescription::builder()
        .format          (data.swapchain_format)
        .samples         (vk::SampleCountFlags ::_1)
        .load_op         (vk::AttachmentLoadOp ::CLEAR)
        .store_op        (vk::AttachmentStoreOp::STORE)
        .stencil_load_op (vk::AttachmentLoadOp ::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout  (vk::ImageLayout      ::UNDEFINED)
        .final_layout    (vk::ImageLayout      ::PRESENT_SRC_KHR)
    ;

    let color_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout    (vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
    ;

    let color_attachments = &[color_attachment_ref];
    let subpass = vk::SubpassDescription::builder()
        .pipeline_bind_point     (vk::PipelineBindPoint::GRAPHICS)
        .color_attachments       (color_attachments)
        .depth_stencil_attachment(&depth_stencil_attachment_ref)
    ;

    let dependency = vk::SubpassDependency::builder()
        .src_subpass    (vk::SUBPASS_EXTERNAL)
        .dst_subpass    (0)
        .src_stage_mask (
              vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
            | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
        )
        .src_access_mask(
            vk::AccessFlags::empty()
        )
        .dst_stage_mask (
              vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
            | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
        )
        .dst_access_mask(
              vk::AccessFlags::COLOR_ATTACHMENT_WRITE
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
        )
    ;


    let attachments  = &[color_attachment, depth_stencil_attachment];
    let subpasses    = &[subpass];
    let dependencies = &[dependency];

    let info = vk::RenderPassCreateInfo::builder()
        .attachments (attachments)
        .subpasses   (subpasses)
        .dependencies(dependencies)
    ;

    data.render_pass = device.create_render_pass(&info, ALLOCATOR)?;



    Ok(())
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
        data:            &AppData,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<Self>
    {
        Ok(Self {
            capabilities:  instance.get_physical_device_surface_capabilities_khr (physical_device, data.surface)?,
            formats:       instance.get_physical_device_surface_formats_khr      (physical_device, data.surface)?,
            present_modes: instance.get_physical_device_surface_present_modes_khr(physical_device, data.surface)?,
        })
    }
}
