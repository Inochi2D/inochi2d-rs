/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use std::path::PathBuf;

#[cfg(feature = "logging")]
use tracing::debug;

#[cfg(feature = "opengl")]
use crate::ffi::inPuppetDraw;
use crate::ffi::{
    inPuppetDestroy, inPuppetGetName, inPuppetLoad, inPuppetLoadEx, inPuppetLoadFromMemory,
    inPuppetUpdate, types::InPuppetPtr,
};

pub struct Inochi2DPuppet {
    handle: InPuppetPtr,
    pub name: String,
}

impl Inochi2DPuppet {
    pub fn from(buffer: *const u8, size: usize, name: Option<String>) -> Self {
        #[cfg(feature = "logging")]
        debug!("Constructing puppet from {} bytes", size);
        unsafe {
            let hndl = inPuppetLoadFromMemory(buffer, size);
            Inochi2DPuppet {
                handle: hndl,
                name: name.unwrap_or(String::from("<in-memory-puppet>")),
            }
        }
    }

    pub fn new(puppet: PathBuf) -> Self {
        let puppet_path = String::from(puppet.to_str().expect("Unable to get puppet path"));
        #[cfg(feature = "logging")]
        debug!("Constructing puppet from file {}", puppet_path);

        unsafe {
            let hndl = inPuppetLoadEx(puppet_path.as_ptr(), puppet_path.len());

            Inochi2DPuppet {
                handle: hndl,
                name: puppet_path,
            }
        }
    }

    pub fn update(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Updating puppet {}", self.name);

        unsafe {
            inPuppetUpdate(self.handle);
        }
    }

    #[cfg(feature = "opengl")]
    pub fn draw(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Drawing puppet {}", self.name);

        unsafe {
            inPuppetDraw(self.handle);
        }
    }
}

impl Drop for Inochi2DPuppet {
    fn drop(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Disposing of puppet {}", self.name);

        unsafe {
            inPuppetDestroy(self.handle);
        }
    }
}
