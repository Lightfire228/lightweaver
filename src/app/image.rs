use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Vertex, device::{self, Device}, get_memory_type_index, image_view::{self, ImageView}, instance::{self, Instance}, render_pass::RenderPass, surface::{self, Surface}, swapchain}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};


pub struct Image {
    device: Rc<Device>,

    opts:   ImageOpts,
    image:  vk::Image,
    memory: vk::DeviceMemory,
}

#[derive(Clone, Copy)]
pub struct ImageOpts {
    pub width:      u32,
    pub height:     u32,
    pub format:     vk::Format,
    pub tiling:     vk::ImageTiling,
    pub usage:      vk::ImageUsageFlags,
    pub properties: vk::MemoryPropertyFlags,
}

impl Image {

    pub fn new(
        device:     Rc<Device>,
        instance:   &Instance,
        opts:       ImageOpts,

    )
        -> Result<Self>
    {

        let d = unsafe {
            device.device()
        };

        let ImageOpts { width, height, format, tiling, usage, properties } = opts;

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

        let image = unsafe {
            device.device().create_image(&info, None)?
        };

        let requirements = unsafe { d.get_image_memory_requirements(image) };
        let info         = vk::MemoryAllocateInfo::builder()
            .allocation_size  (requirements.size)
            .memory_type_index(unsafe { get_memory_type_index(
                instance,
                device.physical_device,
                properties,
                requirements
                )?})
        ;

        let memory = unsafe {
            let memory = d.allocate_memory(&info, None)?;
            d.bind_image_memory(image, memory, 0)?;

            memory
        };

        Ok(Self {
            device,
            opts,
            image,
            memory,
        })
    }

    pub unsafe fn image (&self) -> vk::Image {
        self.image
    }

    pub unsafe fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

}

impl Drop for Image {

    fn drop(&mut self) {
        debug!("Dropping Image");

        unsafe {
            self.device.device().free_memory  (self.memory, None);
            self.device.device().destroy_image(self.image,  None);
        }

        debug!("/Dropping Image");
    }
}
