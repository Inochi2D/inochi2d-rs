/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

extern crate gl;
extern crate glfw;
extern crate inochi2d_rs;
#[cfg(feature = "logging")]
extern crate tracing_subscriber;

use std::path::PathBuf;

use glfw::Context;
use inochi2d_rs::{
    camera::Inochi2DCamera, core::Inochi2D, puppet::Inochi2DPuppet, scene::Inochi2DScene,
};

#[cfg(feature = "logging")]
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

use std::sync::Mutex;
use std::time::Instant;

/* Cursed Monotonic timer */
extern "C" fn get_time() -> f64 {
    static mut START: Option<Mutex<Instant>> = None;
    if let Some(mutex) = unsafe { &START } {
        let start = mutex.lock().unwrap();
        Instant::now().duration_since(*start).as_secs_f64()
    } else {
        unsafe { START.replace(Mutex::new(Instant::now())) };
        0.0_f64
    }
}

fn main() {
    /* You can ignore this, it's used for debug logging and is optional */
    #[cfg(feature = "logging")]
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::DEBUG)
        .init();

    /* Setup a new GLFW context */
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    /* Set OpenGL window hints */
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    /* Create a new window for us to render in */
    let (mut win, events) = glfw
        .create_window(800, 600, "inochi2d-rs", glfw::WindowMode::Windowed)
        .expect("Unable to create window");

    /* Ensure it's the current window */
    win.make_current();

    /* Make sure GL and GLFW can talk to each other */
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    /* Setup polling for the window events we care about */
    win.set_key_polling(true);
    win.set_framebuffer_size_polling(true);
    win.set_scroll_polling(true);

    /* Create a new Inochi2D context */
    let mut ctx = Inochi2D::new(get_time, 800, 800);
    /* Create a new Inochi2D puppet from a file */
    let mut puppet = Inochi2DPuppet::new(PathBuf::from("./examples/models/Aka.inx"));

    /* Setup the camera and zoom */
    let mut zoom: f64 = 0.15;
    let mut cam = Inochi2DCamera::new(Some(zoom as f32), Some(0.0), Some(0.0));

    /* Setup the Inochi2D scene to draw */
    let mut scene = Inochi2DScene::new();

    /* Main Window loop */
    while !win.should_close() {
        /* Clear the Framebuffer so we don't get ghosting and garbage */
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        /* Pull events and handle the ones we care about*/
        glfw.poll_events();
        for (_, e) in glfw::flush_messages(&events) {
            match e {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    /* Let us exit the application */
                    win.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    /* Handle window resizing */
                    ctx.set_viewport(width, height);
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                glfw::WindowEvent::Scroll(_, vert) => {
                    /* Allow us to zoom with the scroll wheel */
                    zoom += vert * 0.01;
                    zoom = zoom.clamp(0.01, 10.0);
                    cam.set_zoom(zoom as f32);
                }
                _ => {}
            }
        }
        /* Update and then draw the puppet */
        puppet.update();
        puppet.draw();
        /* Draw the scene */
        scene.draw(0.0, 0.0, ctx.view_width as f32, ctx.view_height as f32);
        /* Swap the buffers and continue */
        win.swap_buffers();
    }
}
