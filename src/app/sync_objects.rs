

use log::debug;
use vulkanalia::vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands};

use std::{fs::File, ptr::copy_nonoverlapping as memcpy, rc::Rc, result::Result::Ok};
use anyhow::{anyhow, Result};
use winit::window::Window;

use crate::{app::{MAX_FRAMES_IN_FLIGHT, buffer::Buffer, command_pool::CommandPool, device::Device, image::{Image, ImageOpts}, image_view::ImageView, instance::Instance}, script::vm::debug};

use super::device;

pub struct SyncObjects {
    device: Rc<Device>,

    pub image_available_sempahore: Vec<vk::Semaphore>,
    pub render_finished_sempahore: Vec<vk::Semaphore>,

    pub in_flight_fences:          Vec<vk::Fence>,
    pub images_in_flight:          Vec<vk::Fence>,
}

impl SyncObjects {
    pub fn new(device: Rc<Device>, images: &[vk::Image])
        -> Result<Self>
    {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info     = vk::FenceCreateInfo    ::builder()
            .flags(vk::FenceCreateFlags::SIGNALED)
        ;

        let mut sync_objects = Self {
            device:                    device.clone(),
            image_available_sempahore: Vec::new(),
            render_finished_sempahore: Vec::new(),
            in_flight_fences:          Vec::new(),
            images_in_flight:          Vec::new(),
        };

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                sync_objects.image_available_sempahore.push(device.device().create_semaphore(&semaphore_info, None)?);
                sync_objects.render_finished_sempahore.push(device.device().create_semaphore(&semaphore_info, None)?);

                sync_objects.in_flight_fences         .push(device.device().create_fence    (&fence_info,     None)?);
            }
        }

        sync_objects.images_in_flight = images
            .iter   ()
            .map    (|_| vk::Fence::null())
            .collect()
        ;

        Ok(sync_objects)
    }
}

impl Drop for SyncObjects {
    fn drop(&mut self) {
        debug!("Dropping Sync Objects");

        unsafe {
            self.in_flight_fences         .iter().for_each(|f| self.device.device().destroy_fence    (*f, None));
            self.render_finished_sempahore.iter().for_each(|s| self.device.device().destroy_semaphore(*s, None));
            self.image_available_sempahore.iter().for_each(|s| self.device.device().destroy_semaphore(*s, None));

        }

        debug!("/Dropping Sync Objects");
    }
}
