/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use std::path::PathBuf;

mod binding {
    extern "C" {
        pub type InPuppet;
    }
    pub type InPuppetPtr = *mut InPuppet;

    extern "C" {
        pub fn inPuppetUpdate(puppet: InPuppetPtr);
        pub fn inPuppetLoad(path: *const u8) -> InPuppetPtr;
        pub fn inPuppetLoadEx(path: *const u8, len: usize) -> InPuppetPtr;
        pub fn inPuppetLoadFromMemory(data: *const u8, len: usize) -> InPuppetPtr;
        pub fn inPuppetDestroy(puppet: InPuppetPtr);

        #[cfg(feature = "opengl")]
        pub fn inPuppetDraw(puppet: InPuppetPtr);
    }
}

pub struct Inochi2DPuppet<'a> {
    handle: binding::InPuppetPtr,
    pub name: &'a str,
}

impl<'a> Inochi2DPuppet<'a> {
	pub fn from(buffer: *const u8, size: usize, name: Option<&'a str>) -> Self {
		unsafe {
			let hndl = binding::inPuppetLoadFromMemory(buffer, size);
			Inochi2DPuppet {
				handle: hndl,
				name: name.unwrap_or("<in-memory-puppet>")
			}
		}
	}

    pub fn new(puppet: PathBuf) -> Self {
        let puppet_path = puppet.to_str().expect("Unable to get puppet path");
        unsafe {
            let hndl = binding::inPuppetLoadEx(puppet_path.as_ptr(), puppet_path.len());
            Inochi2DPuppet {
                handle: hndl,
                name: puppet_path
            }
        }
    }

	pub fn update(&mut self) {
		unsafe {
			binding::inPuppetUpdate(self.handle);
		}
	}

	#[cfg(feature = "opengl")]
	pub fn draw(&mut self) {
		unsafe {
			binding::inPuppetUpdate(self.handle);
			binding::inPuppetDraw(self.handle);
		}
	}
}

impl<'a> Drop for Inochi2DPuppet<'a> {
    fn drop(&mut self) {
        unsafe {
            binding::inPuppetDestroy(self.handle);
        }
    }
}
