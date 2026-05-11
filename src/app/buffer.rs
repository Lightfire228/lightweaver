
use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Index, Vertex, depth_image::DepthImage, device::{self, Device}, get_memory_type_index, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::RenderPass, surface::{self, Surface}, swapchain}};


pub struct Buffer {
    device: Rc<Device>,

    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    name:   &'static str,
}


impl Buffer {

    pub fn new(
        device: Rc<Device>,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        name:   &'static str,
    )
        -> Self
    {
        Self {
            device,
            buffer,
            memory,
            name,
        }
    }


    pub fn create_buffer(
        device:     Rc<Device>,
        instance:   &Instance,
        size:       vk::DeviceSize,
        usage:      vk::BufferUsageFlags,
        properties: vk::MemoryPropertyFlags,
        name:       &'static str,
    )
        -> Result<Buffer>
    {

        let d = device.device();

        let buffer_info = vk::BufferCreateInfo::builder()
            .size        (size)
            .usage       (usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
        ;


        let buffer       = unsafe { d.create_buffer(&buffer_info, None)? };
        let requirements = unsafe { d.get_buffer_memory_requirements(buffer)  };

        let memory_info  = vk::MemoryAllocateInfo::builder()
            .allocation_size  (requirements.size)
            .memory_type_index(unsafe { get_memory_type_index(
                instance,
                device.physical_device,
                properties,
                requirements,
            )?})
        ;

        let memory = unsafe {
            let memory = d.allocate_memory(&memory_info, None)?;
            d.bind_buffer_memory(buffer, memory, 0)?;

            memory
        };

        Ok(Self {
            device,
            buffer,
            memory,
            name,
        })
    }


    pub fn buffer(&self) -> vk::Buffer {
        self.buffer
    }

    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }
}

impl Drop for Buffer {

    fn drop(&mut self) {
        debug!("Dropping {} Buffer", self.name);

        unsafe {
            self.device.device().free_memory   (self.memory, None);
            self.device.device().destroy_buffer(self.buffer, None);
        }

        debug!("/Dropping {} Buffer", self.name);
    }
}
