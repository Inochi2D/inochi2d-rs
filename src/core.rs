/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use crate::puppet::Inochi2DPuppet;

mod binding {
    /* Types */
    extern "C" {
        pub type InRenderable;
    }

    pub type InTimingFunc = extern fn() -> f64;

    /* Functions */
    extern "C" {
        pub fn inCleanup();
        pub fn inInit(timing: InTimingFunc);
        pub fn inViewportSet(width: i32, height: i32);
        pub fn inViewportGet(width: *mut i32, height: *mut i32);
    }
}

pub struct Inochi2D {
    pub puppets: Vec<Inochi2DPuppet>,
}

impl Inochi2D {
    pub fn new(timing: binding::InTimingFunc) -> Self {
        unsafe {
            binding::inInit(timing);

            Inochi2D {
                puppets: Vec::new(),
            }
        }
    }
}

impl Drop for Inochi2D {
    fn drop(&mut self) {
        unsafe {
            binding::inCleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Inochi2D;

    extern fn timing_func() -> f64 {
        0.0
    }

    #[test]
    fn test_initialization() {
        let pup = Inochi2D::new(timing_func);
    }
}
