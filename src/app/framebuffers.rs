use std::{collections::HashSet, ffi::{CStr, c_void}, mem, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, bytecode::Bytecode, vk::{self, DeviceV1_0, EntryV1_0, Handle, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{Vertex, depth_image::DepthImage, device::{self, Device}, image_view::{self, ImageView}, instance::{self, Instance}, render_pass::RenderPass, surface::{self, Surface}, swapchain}};


pub struct Framebuffers {
    device:       Rc<Device>,

    frame_buffers: Vec<vk::Framebuffer>,
}


impl Framebuffers {

    pub fn new(
        device:      Rc<Device>,
        depth_image: &DepthImage,
        image_views: &[ImageView],
        render_pass: &RenderPass,
        extent:      vk::Extent2D,

    )
        -> Result<Self>
    {
        let frame_buffers = image_views
            .iter()
            .map (|i| i.view() )
            .map (|i| {

                let view = depth_image.view().view();

                // The color attachment differs for every swapchain image, but the same depth image can be
                // used by all of them because only a single subpass is running at the same time due to
                // our semaphores.
                let attachments = &[i, view];

                let create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass.render_pass())
                    .attachments(attachments)
                    .width      (extent.width)
                    .height     (extent.height)
                    .layers     (1)
                ;

                unsafe {
                    device.device().create_framebuffer(&create_info, None)
                }
            })
            .collect
                ::<Result<Vec<_>, _>>
            ()?
        ;

        Ok(Self {
            device,
            frame_buffers,
        })

    }


    pub fn frame_buffers(&self) -> &[vk::Framebuffer] {
        &self.frame_buffers
    }

}

impl Drop for Framebuffers {

    fn drop(&mut self) {
        debug!("Dropping Framebuffers");

        unsafe {
            let buffers = mem::take(&mut self.frame_buffers);

            for f in buffers {
                self.device.device().destroy_framebuffer(f, None);
            }
        }

        debug!("/Dropping Framebuffers");
    }
}
