/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

use crate::ffi::{
    inSceneBegin,
    inSceneEnd,
    inSceneDraw
};

pub struct Inochi2DScene {}

impl Inochi2DScene {
    pub fn new() -> Self {
        unsafe {
            inSceneBegin();
        }

        Inochi2DScene {}
    }

    pub fn draw(&mut self, x: f32, y: f32, width: f32, height: f32){
        unsafe {
            inSceneEnd();
            inSceneDraw(x, y, width, height);
            inSceneBegin();
        }
    }
}

impl Drop for Inochi2DScene {
    fn drop(&mut self) {
        unsafe {
            inSceneEnd();
        }
    }
}
