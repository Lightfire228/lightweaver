
use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Index, Vertex, buffer::Buffer, depth_image::DepthImage, device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::RenderPass, surface::{self, Surface}, swapchain}};


pub struct CommandPool {
    device:       Rc<Device>,

    command_pool: vk::CommandPool,

}


impl CommandPool {

    pub fn new(
        device:   Rc<Device>,
        instance: &Instance,
        surface:  &Surface,
    )
        -> Result<Rc<Self>>
    {
        let indicies = instance.get_queue_family_indices(surface, device.physical_device)?;

        let info = vk::CommandPoolCreateInfo::builder()
            .flags             (vk::CommandPoolCreateFlags::empty())
            .queue_family_index(indicies.graphics)
        ;

        let command_pool = unsafe {
            device.device().create_command_pool(&info, None)?
        };

        Ok(Rc::new(Self {
            device,
            command_pool,
        }))
    }

    pub fn command_pool(&self) -> vk::CommandPool {
        self.command_pool
    }
}

impl Drop for CommandPool {

    fn drop(&mut self) {
        debug!("Dropping Command Pool");

        unsafe {
            self.device.device().destroy_command_pool(self.command_pool, None);
        }

        debug!("/Dropping Command Pool");
    }
}
