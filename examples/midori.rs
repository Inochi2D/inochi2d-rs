/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

extern crate gl;
extern crate glfw;
extern crate inochi2d_rs;

use std::path::PathBuf;

use glfw::Context;
use inochi2d_rs::{
    core::Inochi2D,
    puppet::{self, Inochi2DPuppet},
    Inochi2DBuilder,
};

extern "C" fn get_time() -> f64 {
    0.0
}

fn main() {
    let mut time: f64 = 0.0;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut win, events) = glfw
        .create_window(800, 600, "inochi2d-rs", glfw::WindowMode::Windowed)
        .expect("Unable to create window");

    win.make_current();
    win.set_key_polling(true);

    let mut ctx = Inochi2DBuilder::new()
        .viewport(800, 800)
        .timing(get_time)
        .puppet(PathBuf::from("./models/Midori.inx"))
        .build()
        .unwrap();

    while !win.should_close() {
        win.swap_buffers();
        glfw.poll_events();
        for (_, e) in glfw::flush_messages(&events) {
            match e {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    win.set_should_close(true)
                }
                _ => {}
            }
        }
        // ctx.draw_puppets();
    }
}
