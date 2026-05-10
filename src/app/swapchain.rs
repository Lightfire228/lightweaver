use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{depth_image::{self, DepthImage}, descriptor_set_layout::{self, DescriptorSetLayout}, device::{self, Device}, framebuffers::{self, Framebuffers}, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::{self, RenderPass}, surface::{self, Surface}, swapchain}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};

// TODO: do we need to store an instance ref?
pub struct Swapchain {
    instance:      Rc<Instance>,
    device:        Rc<Device>,
    surface:       Surface,
    extent:        vk::Extent2D,
    format:        vk::Format,
    swapchain:     vk::SwapchainKHR,

    images:        Vec<vk::Image>,
    views:         Vec<ImageView>,

    pipeline:      Pipeline,
    depth_image:   DepthImage,
    frame_buffers: Framebuffers,
}


impl Swapchain {
    pub fn new(
        instance:              Rc<Instance>,
        device:                Rc<Device>,
        window:                &Window,
        surface:               Surface,
        descriptor_set_layout: &DescriptorSetLayout,

    )
        -> Result<(Self, RenderPass)>
    {
        let support = instance.get_swapchain_support   (&surface, device.physical_device)?;
        let indices = instance.get_queue_family_indices(&surface, device.physical_device)?;
        let extent  = get_swapchain_extent             (&window, &support.capabilities);
        let format  = get_swapchain_surface_format     (&support.formats);
        let present = get_swapchain_present_mode       (&support.present_modes);

        let mut image_count = support.capabilities.min_image_count +1;
        if
               support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }


        let mut queue_family_indices = vec![];

        let image_sharing_mode = if indices.graphics != indices.present {
            queue_family_indices.push(indices.graphics);
            queue_family_indices.push(indices.present);

            vk::SharingMode::CONCURRENT
        }
        else {
            vk::SharingMode::EXCLUSIVE
        };


        let info = vk::SwapchainCreateInfoKHR::builder()
            .surface             (unsafe { surface.surface() })
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

        let views = unsafe {
            images
                .iter()
                .map (|i| ImageView::new(device.clone(), *i, format.format, vk::ImageAspectFlags::COLOR))
                .collect
                    ::<Result<Vec<_>, _>>
                ()?
        };

        let render_pass   = RenderPass  ::new(device.clone(), &instance, format.format)?;
        let pipeline      = Pipeline    ::new(device.clone(), extent, descriptor_set_layout, &render_pass)?;

        let depth_image   = DepthImage  ::new(device.clone(), &instance, extent)?;
        let frame_buffers = Framebuffers::new(device.clone(), &depth_image, &views, &render_pass, extent)?;


        Ok((
            Self {
                device,
                instance,
                surface,
                extent,
                format: format.format,
                swapchain,

                images,
                views,

                pipeline,
                depth_image,
                frame_buffers,
            },
            render_pass
        ))
    }

    pub fn extent(&self) -> vk::Extent2D {
        self.extent
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
