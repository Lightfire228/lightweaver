use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{anyhow, Result};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DeviceV1_0, EntryV1_0, HasBuilder, InstanceV1_0}, window as vk_window};
use winit::window::Window;
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::{instance::{self, Instance}, surface::Surface}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};


pub struct Device {
    device:   vulkanalia::Device,
    instance: Rc<Instance>,

    pub graphics_queue:  vk::Queue,
    pub present_queue:   vk::Queue,
    pub physical_device: vk::PhysicalDevice,
}


#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);



impl Device {
    pub fn new(
        instance: Rc<Instance>,
        surface:  &Surface,

    )
        -> Result<Rc<Self>>
    {
        let physical_device = instance.pick_physical_device(surface)?;

        let indices = instance.get_queue_family_indices(surface, physical_device)?;

        let mut unique_indices = HashSet::new();

        unique_indices.insert(indices.graphics);
        unique_indices.insert(indices.present);

        let queue_priorities   = &[1.0];
        let queue_info: Vec<_> = unique_indices
            .iter()
            .map(|i| vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities  (queue_priorities)
            )
            .collect()
        ;

        // set layers for compatibility with older vulkan versions
        let layers = if VALIDATION_ENABLED {
            vec![VALIDATION_LAYER.as_ptr()]
        }
        else {
            vec![]
        };


        let mut extensions: Vec<_> = DEVICE_EXTENSIONS
            .iter   ()
            .map    (|n| n.as_ptr())
            .collect()
        ;

        // Required by Vulkan on macOS since 1.3.216
        if cfg!(target_os = "macos") && instance.entry.version()? >= PORTABILITY_MACOS_VERSION {
            extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
        }

        let features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
        ;

        let info     = vk::DeviceCreateInfo::builder()
            .queue_create_infos     (&queue_info)
            .enabled_layer_names    (&layers)
            .enabled_extension_names(&extensions)
            .enabled_features       (&features)
        ;

        let device = unsafe {
            instance.instance().create_device(physical_device, &info, None)?
        };

        let graphics_queue = unsafe { device.get_device_queue(indices.graphics, 0) };
        let present_queue  = unsafe { device.get_device_queue(indices.present,  0) };

        info!("ye");

        Ok(Rc::new(Self {
            device,
            instance,
            physical_device,
            graphics_queue,
            present_queue,
        }))
    }

    pub unsafe fn device(&self) -> &vulkanalia::Device {
        &self.device
    }
}

impl Drop for Device {

    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
