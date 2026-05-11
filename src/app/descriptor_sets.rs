
use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc, ops::Index};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{UniformBufferObject, Vertex, buffer::Buffer, depth_image::DepthImage, descriptor_set_layout::DescriptorSetLayout, device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, pipeline::Pipeline, render_pass::RenderPass, surface::{self, Surface}, swapchain, texture_image::TextureImage, uniform_buffer::UniformBuffers}};


pub struct DescriptorSets {
    device: Rc<Device>,

    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
}


impl DescriptorSets {

    pub fn new(
        device:                Rc<Device>,
        images:                &[vk::Image],
        descriptor_set_layout: &DescriptorSetLayout,
        uniform_buffers:       &UniformBuffers,
        texture_image:         &TextureImage,
    )
        -> Result<Self>
    {

        let descriptor_pool = unsafe { create_descriptor_pool(&device, images)? };

        let layouts = vec![descriptor_set_layout.layout(); images.len()];
        let info    = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool)
            .set_layouts    (&layouts)
        ;

        let descriptor_sets = unsafe { device.device().allocate_descriptor_sets(&info)? };

        for (i, _) in images.iter().enumerate() {
            let info = vk::DescriptorBufferInfo::builder()
                .buffer(uniform_buffers[i].buffer())
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64)
            ;

            let buffer_info = &[info];
            let ubo_write   = vk::WriteDescriptorSet::builder()
                .dst_set          (descriptor_sets[i])
                .dst_binding      (0)
                .dst_array_element(0)
                .descriptor_type  (vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info      (buffer_info)
            ;

            let info = vk::DescriptorImageInfo::builder()
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image_view  (texture_image.view().view())
                .sampler     (texture_image.sampler())
            ;

            let image_info    = &[info];
            let sampler_write = vk::WriteDescriptorSet::builder()
                .dst_set          (descriptor_sets[i])
                .dst_binding      (1)
                .dst_array_element(0)
                .descriptor_type  (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info       (image_info)
            ;

            unsafe {
                device.device().update_descriptor_sets(
                    &[ubo_write, sampler_write],
                    &[] as &[vk::CopyDescriptorSet]
                );
            }
        }

        Ok(Self {
            device,
            descriptor_pool,
            descriptor_sets,
        })
    }
}

impl Drop for DescriptorSets {

    fn drop(&mut self) {
        debug!("Dropping Descriptor Sets");

        unsafe {
            self.device.device().destroy_descriptor_pool(self.descriptor_pool, None);
        }

        debug!("/Dropping Descriptor Sets");
    }
}

impl Index<usize> for DescriptorSets {
    type Output = vk::DescriptorSet;

    fn index(&self, index: usize) -> &Self::Output {
        &self.descriptor_sets[index]
    }
}


unsafe fn create_descriptor_pool(device: &Device, images: &[vk::Image]) -> Result<vk::DescriptorPool> {

    let ubo_size = vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(images.len() as u32)
    ;

    let sampler_size =  vk::DescriptorPoolSize::builder()
        .type_           (vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(images.len() as u32)
    ;

    let pool_sizes = &[ubo_size, sampler_size];
    let info       = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(pool_sizes)
        .max_sets  (images.len() as u32)
    ;

    Ok(device.device().create_descriptor_pool(&info, None)?)
}
