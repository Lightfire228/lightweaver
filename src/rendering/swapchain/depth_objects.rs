use vulkanalia::{bytecode::Bytecode, vk::{self, DeviceV1_0, Handle, HasBuilder, InstanceV1_0}};
use crate::rendering::{ALLOCATOR, Vertex, create_image, device::Device, instance::Instance, swapchain::{self, create_image_view, render_pass::{RenderPass, get_depth_format}} };

use std::{rc::Rc, result::Result::Ok};
use anyhow::{anyhow, Result};

use super::device;


#[derive(Debug)]
pub struct DepthImage {
    instance: Rc<Instance>,
    device:   Rc<device::Device>,

    pub image:   vk::Image,
    pub memory:  vk::DeviceMemory,
    pub view:    vk::ImageView,
}


impl DepthImage {
    pub unsafe fn new(
        device:    Rc<device::Device>,
        instance:  Rc<Instance>,
        extent:    vk::Extent2D,
    )
        -> Result<Self>
    {

        let format = get_depth_format(&instance, device.physical_device)?;

        let (depth_image, depth_image_memory) = create_image(
            &instance,
            &device,
            extent.width,
            extent.height,
            format,
            vk::ImageTiling        ::OPTIMAL,
            vk::ImageUsageFlags    ::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        let depth_image_view = create_image_view(
            &device,
            depth_image,
            format,
            vk::ImageAspectFlags::DEPTH,
        )?;

        Ok(Self {
            device,
            instance,
            image: depth_image,
            memory: depth_image_memory,
            view: depth_image_view,

        })
    }

}

impl Drop for DepthImage {
    fn drop(&mut self) {
    }
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
