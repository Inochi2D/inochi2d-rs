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

    pub view_width: i32,
    pub view_height: i32,
}

impl Inochi2D {
    pub fn set_viewport(&mut self, w: i32, h: i32) {
        unsafe {
            binding::inViewportSet(w, h);
        }
        self.view_width = w;
        self.view_height = h;
    }

    pub fn get_viewport(&mut self) -> (i32, i32) {
        let mut viewport_width: i32 = 0;
        let mut viewport_height: i32 = 0;

        unsafe {
            binding::inViewportGet(&mut viewport_width, &mut viewport_height);
        }

        self.view_width = viewport_width;
        self.view_height = viewport_height;

        (viewport_width, viewport_height)
    }

    pub fn new(timing: binding::InTimingFunc, w: Option<i32>, h: Option<i32>) -> Self {
        let viewport_width = w.unwrap_or(800);
        let viewport_height = h.unwrap_or(600);

        unsafe {
            binding::inInit(timing);
            binding::inViewportSet(viewport_width, viewport_height);

            Inochi2D {
                puppets: Vec::new(),

                view_width: viewport_width,
                view_height: viewport_height,
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
        let pup = Inochi2D::new(timing_func, None, None);
    }
}
