/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

#[cfg(feature = "logging")]
use tracing::{debug, error, info,warn};

use crate::ffi::{
    Types::InCameraPtr,
    inCameraDestroy,
    inCameraGetCenterOffset,
    inCameraGetCurrent,
    inCameraGetMatrix,
    inCameraGetPosition,
    inCameraGetRealSize,
    inCameraGetZoom,
    inCameraSetPosition,
    inCameraSetZoom
};

pub struct Inochi2DCamera {
    handle: InCameraPtr,
    zoom: f32,
    x: f32,
    y: f32,
}

impl Inochi2DCamera {
    pub fn set_zoom(&mut self, zoom: f32) {
        #[cfg(feature = "logging")]
        debug!("Setting camera zoom to: {}", zoom);
        unsafe {
            inCameraSetZoom(self.handle, zoom);
        }

        self.zoom = zoom;
    }

    pub fn get_zoom(&mut self) -> f32 {
        let mut _z: f32 = 0.0;
        unsafe {
            inCameraGetZoom(self.handle, &mut _z);
        }

        self.zoom = _z;
        _z
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        #[cfg(feature = "logging")]
        debug!("Setting camera position to: ({}, {})", x, y);
        unsafe {
            inCameraSetPosition(self.handle, x, y);
        }

        self.x = x;
        self.y = y;
    }

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
            debug!("Creating new camera (x: {} y: {} zoom: {})", cam_x, cam_y, cam_zoom);


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
