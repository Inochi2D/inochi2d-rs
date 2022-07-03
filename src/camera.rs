/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

mod binding {
    extern "C" {
        pub type InCamera;
    }
    pub type InCameraPtr = *mut InCamera;

    extern "C" {
        pub fn inCameraGetCurrent() -> InCameraPtr;
        pub fn inCameraSetPosition(camera: InCameraPtr, x: f32, y: f32);
        pub fn inCameraGetPosition(camera: InCameraPtr, x: *mut f32, y: *mut f32);
        pub fn inCameraSetZoom(camera: InCameraPtr, zoom: f32);
        pub fn inCameraGetZoom(camera: InCameraPtr, zoom: *mut f32);
        pub fn inCameraDestroy(camera: InCameraPtr);
    }
}

pub struct Inochi2DCamera {
	handle: binding::InCameraPtr,
}


impl Inochi2DCamera {
    pub fn new() -> Self {
        unsafe {
            let hndl = binding::inCameraGetCurrent();

			Inochi2DCamera {
                handle: hndl,
            }
        }
    }
}

impl Drop for Inochi2DCamera {
    fn drop(&mut self) {
        unsafe {
            binding::inCameraDestroy(self.handle);
        }
    }
}
