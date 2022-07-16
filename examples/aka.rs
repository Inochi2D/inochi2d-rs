/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

extern crate gl;
extern crate inochi2d_rs;
#[cfg(feature = "logging")]
extern crate tracing_subscriber;

use std::path::PathBuf;

use glutin::dpi::LogicalSize;
use glutin::event::{
    ElementState, Event, KeyboardInput, MouseScrollDelta, StartCause, VirtualKeyCode, WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlProfile, GlRequest};
use inochi2d_rs::{
    camera::Inochi2DCamera, core::Inochi2D, puppet::Inochi2DPuppet, scene::Inochi2DScene,
    MONOTONIC_CLOCK,
};

#[cfg(feature = "logging")]
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*};

fn main() {
    /* You can ignore this, it's used for debug logging and is optional */
    #[cfg(feature = "logging")]
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::DEBUG)
        .init();

    let el = EventLoop::new();

    /* Create a new window for us to render in */
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_title("inochi2d-rs");
    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .expect("Unable to create window");

    /* Ensure it's the current window */
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let gl_context = windowed_context.context();

    /* Make sure GL and glutin can talk to each other */
    gl::load_with(|s| gl_context.get_proc_address(s) as *const _);

    /* Create a new Inochi2D context */
    let mut ctx = Inochi2D::new(MONOTONIC_CLOCK, 800, 800);
    /* Create a new Inochi2D puppet from a file */
    let mut puppet = Inochi2DPuppet::new(PathBuf::from("./examples/models/Aka.inx")).unwrap();

    /* Setup the camera and zoom */
    let mut zoom: f64 = 0.15;
    let mut cam = Inochi2DCamera::new(Some(zoom as f32), Some(0.0), Some(0.0));

    /* Setup the Inochi2D scene to draw */
    let mut scene = Inochi2DScene::new();

    /* Main Window loop */
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    /* Handle window resizing */
                    windowed_context.resize(physical_size);

                    ctx.set_viewport(physical_size.width as i32, physical_size.height as i32);
                    unsafe {
                        gl::Viewport(
                            0,
                            0,
                            physical_size.width as i32,
                            physical_size.height as i32,
                        );
                    }
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, vert),
                    ..
                } => {
                    /* Allow us to zoom with the scroll wheel */
                    zoom += f64::from(vert) * 0.01;
                    zoom = zoom.clamp(0.01, 10.0);
                    cam.set_zoom(zoom as f32);
                }
                _ => (),
            },
            Event::NewEvents(StartCause::Poll) | Event::RedrawRequested(_) => {
                /* Clear the Framebuffer so we don't get ghosting and garbage */
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                /* Update and then draw the puppet */
                puppet.update();
                puppet.draw();
                /* Draw the scene */
                scene.draw(0.0, 0.0, ctx.view_width as f32, ctx.view_height as f32);

                windowed_context.swap_buffers().unwrap();
                windowed_context.window().request_redraw();
            }
            _ => (),
        }
    });
}
