/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use crate::puppet::Inochi2DPuppet;

use std::path::PathBuf;

#[cfg(feature = "logging")]
use tracing::debug;

use crate::ffi::{inCleanup, inInit, inViewportGet, inViewportSet, types::InTimingFunc};

pub struct Inochi2D {
    pub puppets: Vec<Inochi2DPuppet>,

    pub view_width: i32,
    pub view_height: i32,
}

impl Inochi2D {
    /// Add a new puppet to the Inochi2D context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// ctx.add_puppet("./puppets/Ada.inx");
    ///
    /// ~~~
    ///
    pub fn add_puppet(&mut self, puppet: PathBuf) {
        self.puppets.push(Inochi2DPuppet::new(puppet))
    }

    /// Update all puppets in the current context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// ctx.update_puppets();
    ///
    /// ~~~
    ///
    pub fn update_puppets(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Updating all puppets");
        for p in self.puppets.iter_mut() {
            p.update()
        }
    }

    /// Draw all puppets in the current context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// ctx.draw_puppets();
    ///
    /// ~~~
    ///
    #[cfg(feature = "opengl")]
    pub fn draw_puppets(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Drawing all puppets");
        for p in self.puppets.iter_mut() {
            p.draw()
        }
    }

    /// Set the viewport geometry for the current context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// ctx.set_viewport(800, 600);
    ///
    /// ~~~
    ///
    pub fn set_viewport(&mut self, w: i32, h: i32) {
        #[cfg(feature = "logging")]
        debug!("Setting viewport to {}x{}", w, h);
        unsafe {
            inViewportSet(w, h);
        }
        self.view_width = w;
        self.view_height = h;
    }

    /// Get the viewport geometry for the current context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// let viewport = ctx.get_viewport();
    ///
    /// println!("Viewport is {}x{}", viewport.0, viewport.1);
    ///
    /// ~~~
    ///
    /// # Returns
    ///
    /// A tuple `(i32, i32)` with item 0 being the width and item 1 being the height.
    ///
    pub fn get_viewport(&mut self) -> (i32, i32) {
        let mut viewport_width: i32 = 0;
        let mut viewport_height: i32 = 0;

        unsafe {
            inViewportGet(&mut viewport_width, &mut viewport_height);
        }

        self.view_width = viewport_width;
        self.view_height = viewport_height;

        (viewport_width, viewport_height)
    }

    /// Initialize a new Inochi2D context.
    ///
    /// # Example
    ///
    /// ~~~no_run
    /// let mut ctx = Inochi2D::new(/* ... */);
    ///
    /// ~~~
    ///
    /// # Returns
    ///
    /// A new `Inochi2D` context.
    ///
    pub fn new(timing: InTimingFunc, w: i32, h: i32) -> Self {
        #[cfg(feature = "logging")]
        debug!("Constructing Inochi2D");

        unsafe {
            inInit(timing);
            inViewportSet(w, h);

            Inochi2D {
                puppets: Vec::new(),

                view_width: w,
                view_height: h,
            }
        }
    }
}

impl Drop for Inochi2D {
    fn drop(&mut self) {
        #[cfg(feature = "logging")]
        debug!("Disposing of Inochi2D");
        self.puppets.clear();

        unsafe {
            inCleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Inochi2D;

    extern "C" fn timing_func() -> f64 {
        0.0
    }

    #[test]
    fn test_initialization() {
        let _ctx = Inochi2D::new(timing_func, 800, 600);
    }
}
