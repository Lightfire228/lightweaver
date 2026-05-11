use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DescriptorPool, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Index, buffer::Buffer, command_buffers::{self, CommandBuffers}, command_pool::{self, CommandPool}, depth_image::{self, DepthImage}, descriptor_set_layout::{self, DescriptorSetLayout}, descriptor_sets::DescriptorSets, device::{self, Device}, framebuffers::{self, Framebuffers}, image_view::{self, ImageView}, instance::{self, Instance, SwapchainSupport}, pipeline::Pipeline, render_pass::{self, RenderPass}, surface::{self, Surface}, swapchain, texture_image::TextureImage, uniform_buffer::{self, UniformBuffers}}};

pub struct Swapchain {
    device:          Rc<Device>,
    extent:          vk::Extent2D,
    format:          vk::Format,
    support:         SwapchainSupport,
    swapchain:       vk::SwapchainKHR,

    images:          Vec<vk::Image>,
    views:           Vec<ImageView>,

    descriptor_sets: DescriptorSets,
    render_pass:     RenderPass,
    pipeline:        Pipeline,
    depth_image:     DepthImage,
    frame_buffers:   Framebuffers,
    command_buffers: CommandBuffers,
    uniform_buffers: UniformBuffers,
}

pub struct SwapchainOpts<'a> {
    pub window:                &'a Window,
    pub instance:              &'a Instance,
    pub surface:               &'a Surface,
    pub texture_image:         &'a TextureImage,
    pub descriptor_set_layout: &'a DescriptorSetLayout,
    pub vertex_buffer:         &'a Buffer,
    pub index_buffer:          &'a Buffer,
    pub indices:               &'a [Index],
}


impl Swapchain {
    pub fn new(
        device:       Rc<Device>,
        command_pool: Rc<CommandPool>,
        opts:         SwapchainOpts,

    )
        -> Result<Self>
    {

        let SwapchainOpts { window, instance, surface, texture_image, descriptor_set_layout, vertex_buffer, index_buffer, indices } = opts;

        let support    = instance.get_swapchain_support   (&surface, device.physical_device)?;
        let qf_indices = instance.get_queue_family_indices(&surface, device.physical_device)?;
        let extent     = get_swapchain_extent             (&window, &support.capabilities);
        let format     = get_swapchain_surface_format     (&support.formats);
        let present    = get_swapchain_present_mode       (&support.present_modes);

        let mut image_count = support.capabilities.min_image_count +1;
        if
               support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }


        let mut queue_family_indices = vec![];

        let image_sharing_mode = if qf_indices.graphics != qf_indices.present {
            queue_family_indices.push(qf_indices.graphics);
            queue_family_indices.push(qf_indices.present);

            vk::SharingMode::CONCURRENT
        }
        else {
            vk::SharingMode::EXCLUSIVE
        };


        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface             (surface.surface())
            .min_image_count     (image_count)
            .image_format        (format.format)
            .image_color_space   (format.color_space)
            .image_extent        (extent)
            .image_array_layers  (1)
            .image_usage         (vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode  (image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform       (support.capabilities.current_transform)
            .composite_alpha     (vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode        (present)
            .clipped             (true)
            .old_swapchain       (vk::SwapchainKHR::null())
        ;

        let swapchain = unsafe {
            device.device().create_swapchain_khr(&info, None)?
        };

        let images = unsafe {
            device.device().get_swapchain_images_khr(swapchain)?
        };

        let views = images
            .iter()
            .map (|i| ImageView::new(device.clone(), *i, format.format, vk::ImageAspectFlags::COLOR))
            .collect
                ::<Result<Vec<_>, _>>
            ()?
        ;

        let render_pass     = RenderPass    ::new(device.clone(), &instance, format.format)?;
        let pipeline        = Pipeline      ::new(device.clone(), extent, descriptor_set_layout, &render_pass)?;

        let uniform_buffers = UniformBuffers::new(device.clone(), &instance, &images)?;
        let descriptor_sets = DescriptorSets::new(device.clone(), &images, descriptor_set_layout, &uniform_buffers, texture_image)?;

        let depth_image     = DepthImage    ::new(device.clone(), &instance, extent)?;
        let frame_buffers   = Framebuffers  ::new(device.clone(), &depth_image, &views, &render_pass, extent)?;
        let command_buffers = CommandBuffers::new(device.clone(), command_pool, &render_pass, &pipeline, &descriptor_sets, extent, vertex_buffer, index_buffer, indices, &frame_buffers)?;

        Ok(Self {
            device,
            extent,
            format: format.format,
            support,
            swapchain,

            images,
            views,

            render_pass,
            pipeline,
            depth_image,
            frame_buffers,
            command_buffers,
            uniform_buffers,
            descriptor_sets,
        })
    }

    pub fn images(&self) -> &[vk::Image] {
        &self.images
    }

    pub fn extent(&self) -> vk::Extent2D {
        self.extent
    }

    pub fn swapchain(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    pub fn command_buffers(&self) -> &CommandBuffers {
        &self.command_buffers
    }

    pub fn uniform_buffers(&self) -> &UniformBuffers {
        &self.uniform_buffers
    }

    pub fn take_support(&mut self) -> SwapchainSupport {
        mem::take(&mut self.support)
    }

}

impl Drop for Swapchain {

    fn drop(&mut self) {
        debug!("Dropping swapchain");

        self.views.clear();

        unsafe {
            self.device.device().destroy_swapchain_khr(self.swapchain, None);
        }

        debug!("/Dropping swapchain");
    }
}


fn get_swapchain_surface_format(
    formats: &[vk::SurfaceFormatKHR],
)
    -> vk::SurfaceFormatKHR
{
    formats
        .iter  ()
        .cloned()
        .find  (|f| {
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


pub fn get_swapchain_extent(
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
