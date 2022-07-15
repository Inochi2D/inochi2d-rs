/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

#[cfg(feature = "logging")]
use tracing::debug;

use crate::ffi::{
    inCameraDestroy, inCameraGetCenterOffset, inCameraGetCurrent, inCameraGetMatrix,
    inCameraGetPosition, inCameraGetRealSize, inCameraGetZoom, inCameraSetPosition,
    inCameraSetZoom, types::InCameraPtr,
};

pub struct Inochi2DCamera {
    handle: InCameraPtr,
    zoom: f32,
    x: f32,
    y: f32,
}

impl Inochi2DCamera {
    /// Set Inochi2D Camera Zoom
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// camera.set_zoom(0.15);
    /// ~~~
    ///
    pub fn set_zoom(&mut self, zoom: f32) {
        #[cfg(feature = "logging")]
        debug!("Setting camera zoom to: {}", zoom);
        unsafe {
            inCameraSetZoom(self.handle, zoom);
        }

        self.zoom = zoom;
    }

    /// Gets the Inochi2D Camera Zoom
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// let zoom: f32 = camera.get_zoom();
    /// ~~~
    ///
    /// # Returns
    ///
    /// A single `f32` which describes the cameras zoom.
    ///
    pub fn get_zoom(&mut self) -> f32 {
        let mut _z: f32 = 0.0;
        unsafe {
            inCameraGetZoom(self.handle, &mut _z);
        }

        self.zoom = _z;
        _z
    }

    /// Set Inochi2D Camera Position
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// camera.set_pos(0.50, 0.25);
    /// ~~~
    ///
    pub fn set_pos(&mut self, x: f32, y: f32) {
        #[cfg(feature = "logging")]
        debug!("Setting camera position to: ({}, {})", x, y);
        unsafe {
            inCameraSetPosition(self.handle, x, y);
        }

        self.x = x;
        self.y = y;
    }

    /// Get the Inochi2D Camera Position
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// let pos = camera.get_pos();
    ///
    /// println!("Camera is at {} {}", pos.0, pos.1);
    ///
    /// ~~~
    ///
    /// # Returns
    ///
    /// A tuple with two `f32` elements describing the cameras X and Y
    ///
    pub fn get_pos(&mut self) -> (f32, f32) {
        let mut _x: f32 = 0.0;
        let mut _y: f32 = 0.0;

        unsafe {
            inCameraGetPosition(self.handle, &mut _x, &mut _y);
        }

        self.x = _x;
        self.y = _y;

        (_x, _y)
    }

    /// Get the Inochi2D Camera position offset
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// let pos = camera.get_offset();
    ///
    /// println!("Camera's offset is {} {}", pos.0, pos.1);
    ///
    /// ~~~
    ///
    /// # Returns
    ///
    /// A tuple with two `f32` elements describing the cameras X and Y offset
    ///
    pub fn get_offset(&mut self) -> (f32, f32) {
        let mut _x: f32 = 0.0;
        let mut _y: f32 = 0.0;

        unsafe {
            inCameraGetCenterOffset(self.handle, &mut _x, &mut _y);
        }

        (_x, _y)
    }

    /// Get the Inochi2D Camera's real size
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// let sz = camera.get_real_size();
    ///
    /// println!("Camera's real size is {} {}", pos.0, pos.1);
    ///
    /// ~~~
    ///
    /// # Returns
    ///
    /// A tuple with two `f32` elements describing the cameras real size in X and Y
    ///
    pub fn get_real_size(&mut self) -> (f32, f32) {
        let mut _x: f32 = 0.0;
        let mut _y: f32 = 0.0;

        unsafe {
            inCameraGetRealSize(self.handle, &mut _x, &mut _y);
        }

        (_x, _y)
    }

    /// Get the Inochi2D Camera's matrix
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(/* ... */);
    ///
    /// let matrix = camera.get_matrix();
    /// ~~~
    ///
    /// # Returns
    ///
    /// A an array with 16 `f32` elements describing the cameras matrix
    ///
    pub fn get_matrix(&mut self) -> [f32; 16] {
        let mut matrix: [f32; 16] = [0.0; 16];

        unsafe {
            inCameraGetMatrix(self.handle, &mut matrix);
        }

        matrix
    }

    /// Get the current Inochi2D camera and optionally set it's zoom and position
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut camera = Inochi2DCamera::new(
    ///     Some(0.15), None, None
    /// );
    ///
    /// ~~~
    ///
    ///
    /// # Returns
    ///
    /// A new `Inochi2DCamera`
    ///
    ///
    pub fn new(zoom: Option<f32>, x: Option<f32>, y: Option<f32>) -> Self {
        unsafe {
            let hndl = inCameraGetCurrent();
            let cam_zoom = zoom.unwrap_or_else(|| {
                let mut _z: f32 = 0.0;
                inCameraGetZoom(hndl, &mut _z);
                _z
            });
            // TODO: find a better way to do this
            let cam_x = x.unwrap_or_else(|| {
                let mut _x: f32 = 0.0;
                let mut _y: f32 = 0.0;
                inCameraGetPosition(hndl, &mut _x, &mut _y);
                _x
            });
            let cam_y = y.unwrap_or_else(|| {
                let mut _x: f32 = 0.0;
                let mut _y: f32 = 0.0;
                inCameraGetPosition(hndl, &mut _x, &mut _y);
                _y
            });
            #[cfg(feature = "logging")]
            debug!(
                "Creating new camera (x: {} y: {} zoom: {})",
                cam_x, cam_y, cam_zoom
            );

            inCameraSetZoom(hndl, cam_zoom);
            inCameraSetPosition(hndl, cam_x, cam_y);

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
        #[cfg(feature = "logging")]
        debug!("Destroying camera");
        unsafe {
            inCameraDestroy(self.handle);
        }
    }
}
