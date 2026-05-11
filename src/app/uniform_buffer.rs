
use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc, ops::Index};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{UniformBufferObject, Vertex, buffer::Buffer, depth_image::DepthImage, descriptor_set_layout::DescriptorSetLayout, device::{self, Device}, image::Image, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::RenderPass, surface::{self, Surface}, swapchain, texture_image::TextureImage}};


pub struct UniformBuffers {
    device: Rc<Device>,

    buffers: Vec<Buffer>,
}


impl UniformBuffers {

    pub fn new(
        device:   Rc<Device>,
        instance: &Instance,
        images:   &[vk::Image],
    )
        -> Result<Self>
    {
        let mut buffers = Self {
            device,
            buffers: Vec::new(),
        };

        buffers.clear(instance, images)?;

        Ok(buffers)
    }

    pub fn clear(
        &mut self,
        instance: &Instance,
        images:   &[vk::Image]
    )
        -> Result<()>
    {

        self.buffers.clear();

        for _ in images.iter() {

            self.buffers.push(Buffer::create_buffer(
                self.device.clone(),
                instance,
                size_of::<UniformBufferObject>() as u64,
                vk::BufferUsageFlags   ::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
                "Uniform"
            )?);
        }

        Ok(())

    }
}

impl Index<usize> for UniformBuffers {
    type Output = Buffer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffers[index]
    }
}
