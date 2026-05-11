use std::{collections::HashSet, ffi::{CStr, c_void}, marker::PhantomData, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DeviceV1_0, EntryV1_0, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{device::{self, Device}, instance::{self, Instance}, swapchain::Swapchain}};


pub struct ImageView {
    device:    Rc<Device>,
    view:      vk::ImageView,
}


impl ImageView {

    pub fn new(
        device:  Rc<Device>,
        image:   vk::Image,
        format:  vk::Format,
        aspects: vk::ImageAspectFlags
    )
        -> Result<Self>
    {

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask     (aspects)
            .base_mip_level  (0)
            .level_count     (1)
            .base_array_layer(0)
            .layer_count     (1)
        ;

        let info = vk::ImageViewCreateInfo::builder()
            .image            (image)
            .view_type        (vk::ImageViewType::_2D)
            .format           (format)
            .subresource_range(subresource_range)
        ;

        let view = unsafe {
            device.device().create_image_view(&info, None)?
        };

        Ok(Self {
            device,
            view,
        })
    }

    pub fn view(&self) -> vk::ImageView {
        self.view
    }

}

impl Drop for ImageView {

    fn drop(&mut self) {
        debug!("Dropping ImageView");

        unsafe {
            self.device.device().destroy_image_view(self.view, None);
        }

        debug!("/Dropping ImageView");
    }
}
