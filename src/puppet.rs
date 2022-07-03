/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

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

        #[cfg(opengl)]
        pub fn inPuppetDraw(puppet: InPuppetPtr);
    }
}

pub struct Inochi2DPuppet {
    handle: binding::InPuppetPtr,
    pub name: String,
}

impl Inochi2DPuppet {
	pub fn from(buffer: *const u8, size: usize, name: Option<String>) -> Self {
		unsafe {
			let hndl = binding::inPuppetLoadFromMemory(buffer, size);
			Inochi2DPuppet {
				handle: hndl,
				name: name.unwrap_or(String::from("<in-memory-puppet>"))
			}
		}
	}

    pub fn new(puppet: String) -> Self {
        unsafe {
            let hndl = binding::inPuppetLoadEx(puppet.as_ptr(), puppet.len());
            Inochi2DPuppet {
                handle: hndl,
                name: puppet
            }
        }
    }

	pub fn update(&mut self) {
		unsafe {
			binding::inPuppetUpdate(self.handle);
		}
	}

	#[cfg(opengl)]
	pub fn draw(&mut self) {
		unsafe {
			binding::inPuppetUpdate(self.handle);
			binding::inPuppetDraw(self.handle);
		}
	}
}

impl Drop for Inochi2DPuppet {
    fn drop(&mut self) {
        unsafe {
            binding::inPuppetDestroy(self.handle);
        }
    }
}
