use std::{collections::HashSet, ffi::{CStr, c_void}, rc::Rc};
use anyhow::{Ok, Result, anyhow};

use log::*;
use thiserror::Error;
use vulkanalia::{Entry, vk::{self, DeviceV1_0, EntryV1_0, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands}, window as vk_window};
use winit::window::{self, Window};
use vulkanalia::Version;
use vulkanalia::Instance as VkInstance;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use crate::{app::instance::{self, Instance}, rendering::{DEVICE_EXTENSIONS, PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER}};


pub struct Surface {
    pub instance: Rc<Instance>,
        surface:  vk::SurfaceKHR,
}


impl Surface {
    pub fn new(
        instance: Rc<Instance>,
        window:   &Window
    )
        -> Result<Self>
    {
        let surface = unsafe {
            vk_window::create_surface(instance.instance(), window, window)?
        };

        Ok(Self {
            instance,
            surface,

        })
    }

    /// # safety
    /// the caller must ensure the surface isn't dropped while this handle is valid
    pub unsafe fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }
}

impl Drop for Surface {

    fn drop(&mut self) {
        unsafe {
            self.instance.instance().destroy_surface_khr(self.surface, None);
        }
    }
}
