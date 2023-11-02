use nannou::prelude::*;

use crate::ant::Ant;
use crate::Nannou;

// describes the state of the app, a window and a vector of ants
pub struct Model {
    _window: window::Id,
    ants: Vec<Ant>,
}

// add a way to make a new Model
impl Model {
    pub fn new(_window: window::Id, ant_texture: wgpu::Texture, num_ants: u32) -> Self {
        let ants = (0..num_ants)
            .map(|_| Ant::new(ant_texture.clone()))
            .collect();

        Self { _window, ants }
    }
}

// call display and update on all the ants
impl Nannou for Model {
    fn display(&self, _model: &Model, draw: &nannou::Draw) {
        self.ants.iter().for_each(|ant| ant.display(_model, draw));
    }

    fn update(&mut self) {
        self.ants.iter_mut().for_each(|ant| ant.update());
    }
}
