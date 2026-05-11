use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Vertex, device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, render_pass::RenderPass, surface::{self, Surface}, swapchain}};


pub struct DescriptorSetLayout {
    device: Rc<Device>,

    layout: vk::DescriptorSetLayout,
}


impl DescriptorSetLayout {

    pub fn new(
        device: Rc<Device>,
    )
        -> Result<Self>
    {

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

        let layout = unsafe {
            device.device().create_descriptor_set_layout(&info, None)?
        };

        Ok(Self {
            device,
            layout,
        })
    }

    pub fn layout(&self) -> vk::DescriptorSetLayout {
        self.layout
    }

}

impl Drop for DescriptorSetLayout {

    fn drop(&mut self) {
        debug!("Dropping DescriptorSetLayout");

        unsafe {
            self.device.device().destroy_descriptor_set_layout(self.layout, None);
        }

        debug!("/Dropping DescriptorSetLayout");
    }
}
