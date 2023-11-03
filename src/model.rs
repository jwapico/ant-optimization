use nannou::prelude::*;

use crate::ant::Ant;
use crate::Nannou;

// describes the state of the app, a window and a vector of ants
pub struct Model {
    ants: Vec<Ant>,
}

// add a way to make a new Model
impl Model {
    pub fn new(ant_texture: wgpu::Texture, num_ants: u32) -> Self {
        let ants = (0..num_ants)
            .map(|_| Ant::new(ant_texture.clone()))
            .collect();

        Self { ants }
    }
}

// call display and update on all the ants
impl Nannou for Model {
    fn display(&self, draw: &nannou::Draw) {
        self.ants.iter().for_each(|ant| ant.display(draw));
    }

    fn update(&mut self, window_dimensions: Vec2) {
        self.ants
            .iter_mut()
            .for_each(|ant| ant.update(window_dimensions));
    }
}
