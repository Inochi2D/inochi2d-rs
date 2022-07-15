/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

pub mod types {
    #[repr(C)]
    pub struct InError {
        len: usize,
        msg: *const u8,
    }

    create_opaque_type!(InPuppet);
    pub type InPuppetPtr = *mut InPuppet;

    create_opaque_type!(InCamera);
    pub type InCameraPtr = *mut InCamera;

    create_opaque_type!(InRenderable);
    pub type InRenderablePtr = *mut InRenderable;

    pub type InTimingFunc = extern "C" fn() -> f64;
}

extern "C" {
    /* Core Functionality */
    pub fn inInit(timing: types::InTimingFunc);
    pub fn inCleanup();
    pub fn inViewportSet(width: i32, height: i32);
    pub fn inViewportGet(width: *mut i32, height: *mut i32);

    #[cfg(feature = "opengl")]
    pub fn inSceneBegin();
    #[cfg(feature = "opengl")]
    pub fn inSceneEnd();
    #[cfg(feature = "opengl")]
    pub fn inSceneDraw(x: f32, y: f32, width: f32, height: f32);

    /* Cameras */
    pub fn inCameraGetCurrent() -> types::InCameraPtr;
    pub fn inCameraDestroy(camera: types::InCameraPtr);
    pub fn inCameraGetPosition(camera: types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraSetPosition(camera: types::InCameraPtr, x: f32, y: f32);
    pub fn inCameraGetZoom(camera: types::InCameraPtr, zoom: *mut f32);
    pub fn inCameraSetZoom(camera: types::InCameraPtr, zoom: f32);
    pub fn inCameraGetCenterOffset(camera: types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraGetRealSize(camera: types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraGetMatrix(camera: types::InCameraPtr, mat4: *mut f32);

    /* Puppets */
    pub fn inPuppetLoad(path: *const u8) -> types::InPuppetPtr;
    pub fn inPuppetLoadEx(path: *const u8, len: usize) -> types::InPuppetPtr;
    pub fn inPuppetLoadFromMemory(data: *const u8, len: usize) -> types::InPuppetPtr;
    pub fn inPuppetDestroy(puppet: types::InPuppetPtr);
    pub fn inPuppetGetName(puppet: types::InPuppetPtr, name: *const u8, len: *const usize);
    pub fn inPuppetUpdate(puppet: types::InPuppetPtr);
    #[cfg(feature = "opengl")]
    pub fn inPuppetDraw(puppet: types::InPuppetPtr);
}
