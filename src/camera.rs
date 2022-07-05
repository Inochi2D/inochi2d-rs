/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

mod binding {
    create_opaque_type!(InCamera);

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
	zoom: f32,
	x: f32,
	y: f32,
}


impl Inochi2DCamera {
	pub fn set_zoom(&mut self, zoom: f32) {
		unsafe {
			binding::inCameraSetZoom(self.handle, zoom);
		}

		self.zoom = zoom;
	}

	pub fn get_zoom(&mut self) -> f32 {
		let mut _z: f32 = 0.0;
		unsafe {
			binding::inCameraGetZoom(self.handle, &mut _z);
		}

		self.zoom = _z;
		_z
	}

	pub fn sett_pos(&mut self, x: f32, y: f32) {
		unsafe {
			binding::inCameraSetPosition(self.handle, x, y);
		}

		self.x = x;
		self.y = y;
	}

	pub fn get_pos(&mut self) -> (f32, f32) {
		let mut _x: f32 = 0.0;
		let mut _y: f32 = 0.0;

		unsafe {
			binding::inCameraGetPosition(self.handle, &mut _x, &mut _y);
		}

		self.x = _x;
		self.y = _y;

		(_x, _y)
	}

    pub fn new(zoom: Option<f32>, x: Option<f32>, y: Option<f32>) -> Self {
		unsafe {
            let hndl = binding::inCameraGetCurrent();
			let cam_zoom = zoom.unwrap_or_else(|| {
				let mut _z: f32 = 0.0;
				binding::inCameraGetZoom(hndl, &mut _z);
				_z
			});
			// TODO: find a better way to do this
			let cam_x = x.unwrap_or_else(|| {
				let mut _x: f32 = 0.0;
				let mut _y: f32 = 0.0;
				binding::inCameraGetPosition(hndl, &mut _x, &mut _y);
				_x
			});
			let cam_y = y.unwrap_or_else(|| {
				let mut _x: f32 = 0.0;
				let mut _y: f32 = 0.0;
				binding::inCameraGetPosition(hndl, &mut _x, &mut _y);
				_y
			});

			Inochi2DCamera {
                handle: hndl,
				zoom: cam_zoom,
				x: cam_x,
				y: cam_y,
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
