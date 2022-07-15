/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

pub mod Types {
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
    pub fn inInit(timing: Types::InTimingFunc);
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
    pub fn inCameraGetCurrent() -> Types::InCameraPtr;
    pub fn inCameraDestroy(camera: Types::InCameraPtr);
    pub fn inCameraGetPosition(camera: Types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraSetPosition(camera: Types::InCameraPtr, x: f32, y: f32);
    pub fn inCameraGetZoom(camera: Types::InCameraPtr, zoom: *mut f32);
    pub fn inCameraSetZoom(camera: Types::InCameraPtr, zoom: f32);
    pub fn inCameraGetCenterOffset(camera: Types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraGetRealSize(camera: Types::InCameraPtr, x: *mut f32, y: *mut f32);
    pub fn inCameraGetMatrix(camera: Types::InCameraPtr, mat4: *mut f32);

    /* Puppets */
    pub fn inPuppetLoad(path: *const u8) -> Types::InPuppetPtr;
    pub fn inPuppetLoadEx(path: *const u8, len: usize) -> Types::InPuppetPtr;
    pub fn inPuppetLoadFromMemory(data: *const u8, len: usize) -> Types::InPuppetPtr;
    pub fn inPuppetDestroy(puppet: Types::InPuppetPtr);
    pub fn inPuppetGetName(puppet: Types::InPuppetPtr, name: *const u8, len: *const usize);
    pub fn inPuppetUpdate(puppet: Types::InPuppetPtr);
    #[cfg(feature = "opengl")]
    pub fn inPuppetDraw(puppet: Types::InPuppetPtr);
}
