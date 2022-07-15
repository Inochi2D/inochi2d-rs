/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use std::path::PathBuf;

use crate::ffi::{
    Types::InPuppetPtr,
    inPuppetLoad,
    inPuppetLoadEx,
    inPuppetLoadFromMemory,
    inPuppetDestroy,
    inPuppetGetName,
    inPuppetUpdate
};
#[cfg(feature = "opengl")]
use crate::ffi::inPuppetDraw;


pub struct Inochi2DPuppet {
    handle: InPuppetPtr,
    pub name: String,
}

impl Inochi2DPuppet {
    pub fn from(buffer: *const u8, size: usize, name: Option<String>) -> Self {
        unsafe {
            let hndl = inPuppetLoadFromMemory(buffer, size);
            Inochi2DPuppet {
                handle: hndl,
                name: name.unwrap_or(String::from("<in-memory-puppet>")),
            }
        }
    }

    pub fn new(mut puppet: PathBuf) -> Self {
        let puppet_path = String::from(puppet.to_str().expect("Unable to get puppet path"));

        unsafe {
            let hndl = inPuppetLoadEx(puppet_path.as_ptr(), puppet_path.len());

            Inochi2DPuppet {
                handle: hndl,
                name: puppet_path,
            }
        }
    }

    pub fn update(&mut self) {
        unsafe {
            inPuppetUpdate(self.handle);
        }
    }

    #[cfg(feature = "opengl")]
    pub fn draw(&mut self) {
        unsafe {
            inPuppetDraw(self.handle);
        }
    }
}

impl Drop for Inochi2DPuppet {
    fn drop(&mut self) {
        unsafe {
            inPuppetDestroy(self.handle);
        }
    }
}
