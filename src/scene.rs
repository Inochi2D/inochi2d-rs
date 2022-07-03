/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

mod binding {
    extern "C" {
        pub fn inSceneBegin();
        pub fn inSceneEnd();
        pub fn inSceneDraw(x: f32, y: f32, width: f32, height: f32);
    }
}


pub struct Inochi2DScene {}

impl Inochi2DScene {

}
