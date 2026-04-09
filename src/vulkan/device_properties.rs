

use thiserror::Error;
use anyhow::{Result, Ok, anyhow};
use vulkanalia::{prelude::v1_4::*};
use log::*;

use crate::vulkan::AppData;

#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    graphics: u32,
}


impl AppData {
    pub unsafe fn pick_physical_device(
        &self,
        instance: &Instance
    ) 
        -> Result<vk::PhysicalDevice>
    {
        let devices = unsafe {
            instance.enumerate_physical_devices()?
        };

        for physical_device in devices {

            let properties = unsafe {
                instance.get_physical_device_properties(physical_device)
            };


            let res = unsafe { self.check_physical_device(instance, physical_device) };

            if let Err(error) = res {
                warn!("Skipping physical device (`{}`): {}", properties.device_name, error);
            } 
            else {
                info!("Selected physical device (`{}`).", properties.device_name);
                return Ok(physical_device);
            }
        }

        Err(anyhow!("Failed to find suitable physical device."))
    }

    unsafe fn check_physical_device(
        &self,
        instance:        &Instance,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<()> 
    {
        unsafe { 
            QueueFamilyIndices::get(instance, self, physical_device)?;
        };


        Ok(())
    }

}

impl QueueFamilyIndices {
    unsafe fn get(
        instance:        &Instance,
        data:            &AppData,
        physical_device: vk::PhysicalDevice,
    )
        -> Result<Self> 
    {
        let properties = unsafe {
            instance
                .get_physical_device_queue_family_properties(physical_device)
        };

        let graphics = properties
            .iter    ()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map     (|i| i as u32)
        ;

        if let Some(graphics) = graphics {
            Ok(Self { graphics })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}
