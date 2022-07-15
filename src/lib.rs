#![cfg_attr(feature = "nightly", feature(extern_types))]
#![allow(unsafe_op_in_unsafe_fn, unused_unsafe)]
/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

/*!
    An attempted idiomatic wrapper for [Inochi2D](https://github.com/Inochi2D/inochi2d).

*/

#[macro_use]
mod macros;

mod ffi;

pub mod camera;
pub mod core;
pub mod puppet;
#[cfg(feature = "opengl")]
pub mod scene;

use crate::core::Inochi2D;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, String>;

pub struct Inochi2DBuilder {
    viewport_size: (i32, i32),
    time_func: Option<extern "C" fn() -> f64>,
    puppets: Vec<PathBuf>,
}

#[cfg(feature = "monotonic")]
use std::sync::Mutex;
#[cfg(feature = "monotonic")]
use std::time::Instant;

/* A hacky Monotonic clock for Inochi2D */
#[cfg(feature = "monotonic")]
pub extern "C" fn MONOTONIC_CLOCK() -> f64 {
    static mut START: Option<Mutex<Instant>> = None;
    if let Some(mutex) = unsafe { &START } {
        let start = mutex.lock().unwrap();
        Instant::now().duration_since(*start).as_secs_f64()
    } else {
        unsafe { START.replace(Mutex::new(Instant::now())) };
        0.0_f64
    }
}

impl<'a> Inochi2DBuilder {
    /// Creates a new Inochi2D context builder.
    ///
    ///
    /// # Example
    /// ~~~no_run
    /// extern crate inochi2d_rs;
    ///
    /// use inochi2d_rs::Inochi2DBuilder;
    ///
    /// fn main() {
    ///     let ctx = Inochi2DBuilder::new()
    ///         .build()
    ///         .expect("Unable to create Inochi2D context");
    /// }
    /// ~~~
    ///
    /// # Returns
    ///
    /// A new instance of `Inochi2DBuilder`.
    ///
    pub fn new() -> Self {
        Inochi2DBuilder {
            viewport_size: (800, 600),
            time_func: None,
            puppets: Vec::new(),
        }
    }

    /// Set Inochi2D viewport size.
    ///
    /// # Example
    /// ~~~no_run
    /// let ctx = Inochi2DBuilder::new()
    ///     .viewport(800, 600)
    ///     .build()
    ///     .expect("Unable to create Inochi2D context");
    /// ~~~
    ///
    /// # Returns
    ///
    /// The current `Inochi2DBuilder` instance.
    ///
    pub fn viewport(mut self, w: i32, h: i32) -> Inochi2DBuilder {
        self.viewport_size = (w, h);
        self
    }

    /// Add a puppet to be loaded.
    ///
    /// # Example
    /// ~~~no_run
    /// let ctx = Inochi2DBuilder::new()
    ///     .puppet("./puppets/Ada.idx")
    ///     .build()
    ///     .expect("Unable to create Inochi2D context");
    /// ~~~
    ///
    /// # Returns
    ///
    /// The current `Inochi2DBuilder` instance.
    ///
    pub fn puppet(mut self, puppet: PathBuf) -> Inochi2DBuilder {
        self.puppets.push(puppet);
        self
    }

    /// Add a puppet to be loaded.
    ///
    /// # Example
    /// ~~~no_run
    /// let ctx = Inochi2DBuilder::new()
    ///     .timing(|| {
    ///         0.0
    ///     })
    ///     .build()
    ///     .expect("Unable to create Inochi2D context");
    /// ~~~
    ///
    /// # Returns
    ///
    /// The current `Inochi2DBuilder` instance.
    ///
    pub fn timing(mut self, func: extern "C" fn() -> f64) -> Inochi2DBuilder {
        self.time_func = Some(func);
        self
    }

    /// Create an Inochi2D context from the current builder.
    ///
    /// # Example
    /// ~~~no_run
    /// let ctx = Inochi2DBuilder::new()
    ///     .build()
    ///     .expect("Unable to create Inochi2D context");
    /// ~~~
    ///
    /// # Returns
    ///
    /// - If initialization was successful a `Inochi2D` context.
    /// - If an error occurred a string indicating the error will be returned.
    ///
    pub fn build(self) -> Result<Inochi2D> {
        if self.time_func.is_none() {
            Err("timing mut be called before build!".into())
        } else {
            let mut ctx = Inochi2D::new(
                self.time_func.unwrap(),
                self.viewport_size.0,
                self.viewport_size.1,
            );

            for p in self.puppets {
                ctx.add_puppet(p)?;
            }

            Ok(ctx)
        }
    }
}
