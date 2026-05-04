

use std::{collections::HashSet,  ops::{Deref, DerefMut}, rc::Rc};

use vulkanalia::vk::{DeviceV1_0, HasBuilder, Queue};
use vulkanalia::{Entry, vk::{self}};
use crate::rendering::{ALLOCATOR, DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, QueueFamilyIndices, instance::Instance};

use super::{VALIDATION_ENABLED, VALIDATION_LAYER};

use anyhow::{Result};
use log::*;


#[derive(Debug)]
pub struct Device {
    device:   vulkanalia::Device,
    instance: Rc<Instance>,

    pub graphics_queue: Queue,
    pub present_queue:  Queue,
    pub physical_device: vk::PhysicalDevice,
}

impl Device {
    pub unsafe fn new(
        entry:           Rc<Entry>,
        instance:        Rc<Instance>,
        physical_device: vk::PhysicalDevice,
        surface:         vk::SurfaceKHR,
    )
        -> Result<Rc<Self>>
    {
        let indices = QueueFamilyIndices::get(instance.as_ref(), surface, physical_device)?;

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
            .iter()
            .map(|n| n.as_ptr())
            .collect()
        ;

        // Required by Vulkan on macOS since 1.3.216
        if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
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

        let device = instance.create_device(physical_device, &info, ALLOCATOR)?;

        let device = Rc::new(Self {
            physical_device,
            graphics_queue: device.get_device_queue(indices.graphics, 0),
            present_queue:  device.get_device_queue(indices.present,  0),
            instance,
            device,
        });

        Ok(device)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        trace!("dropping device");
        unsafe {
            self.device.destroy_device(ALLOCATOR);
        }
        trace!("dropped device");
    }
}

impl DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

impl Deref for Device {
    type Target = vulkanalia::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
