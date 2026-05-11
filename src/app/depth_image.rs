use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Vertex, device::{self, Device}, image::{Image, ImageOpts}, image_view::{self, ImageView}, instance::{self, Instance}, render_pass::RenderPass, surface::{self, Surface}, swapchain}};

pub struct DepthImage {
    device:   Rc<device::Device>,

    image:   Image,
    view:    ImageView,
}


impl DepthImage {
    pub fn new(
        device:    Rc<Device>,
        instance:  &Instance,
        extent:    vk::Extent2D,
    )
        -> Result<Self>
    {
        unsafe {
            let format = get_depth_format(&instance, device.physical_device)?;

            let opts = ImageOpts {
                width:      extent.width,
                height:     extent.height,
                format,
                tiling:     vk::ImageTiling        ::OPTIMAL,
                usage:      vk::ImageUsageFlags    ::DEPTH_STENCIL_ATTACHMENT,
                properties: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            };

            let image = Image::new(device.clone(), &instance, opts)?;

            let view = ImageView::new(device.clone(), image.image(), format, vk::ImageAspectFlags::DEPTH)?;

            Ok(Self {
                device,
                image,
                view,

            })

        }

    }

    pub fn image (&self) -> &Image {
        &self.image
    }

    pub fn view  (&self) -> &ImageView {
        &self.view
    }

}

impl Drop for DepthImage {
    fn drop(&mut self) {
    }
}


pub unsafe fn get_depth_format(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<vk::Format> {
    let candidates = &[
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    get_supported_format(
        instance,
        physical_device,
        candidates,
        vk::ImageTiling       ::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}


unsafe fn get_supported_format(
    instance:        &Instance,
    physical_device: vk::PhysicalDevice,
    candidates:      &[vk::Format],
    tiling:          vk::ImageTiling,
    features:        vk::FormatFeatureFlags,
)
    -> Result<vk::Format>
{
    candidates
        .iter  ()
        .cloned()
        .find  (|f| {
            let properties = instance.instance().get_physical_device_format_properties(
                physical_device,
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
