/*
    Copyright © 2022, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Aki "lethalbit" Van Ness
*/

#![feature(extern_types)]
pub mod binding;

pub struct Inochi2D {
    puppet: binding::InPuppetPtr,
}

impl Inochi2D {
    pub fn new(puppet: String, timing: binding::InTimingFunc) -> Self {
        unsafe {
            binding::inInit(timing);
            let pup = binding::inLoadPuppetEx(puppet.as_ptr(), puppet.len());
            Inochi2D{
                puppet: pup
            }
        }
    }
}

impl Drop for Inochi2D {
    fn drop(&mut self) {
        unsafe {
            binding::inDestroyPuppet(self.puppet);
            binding::inCleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Inochi2D;

    fn timing_func() -> f64 {
        0.0
    }

    #[test]
    fn test_initialization() {
        let pup = Inochi2D::new(String::from("./Midori.inx"), timing_func);
    }
}
