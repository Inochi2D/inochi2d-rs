/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/


/* Types */
extern "C" {
    pub type InPuppet;
    pub type InRenderable;
}


pub type InPuppetPtr = *mut InPuppet;
pub type InTimingFunc = fn() -> f64;

/* Functions */
extern "C" {
    pub fn inCleanup();
    pub fn inInit(timing: InTimingFunc);
    pub fn inLoadPuppet(path: *const u8) -> InPuppetPtr;
    pub fn inLoadPuppetEx(path: *const u8, len: usize) -> InPuppetPtr;
    pub fn inLoadPuppetFromMemory(data: *const u8, len: usize) -> InPuppetPtr;
    pub fn inDestroyPuppet(puppet: InPuppetPtr);
}
