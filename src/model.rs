use crate::ant::Ant;
use crate::Nannou;
use nannou::prelude::*;

// describes the state of the app
pub struct Model {
    _window: window::Id,
    ant: Ant,
}

// add a way to make a new Model
impl Model {
    pub fn new(_window: window::Id, ant_texture: wgpu::Texture) -> Self {
        Self {
            _window,
            ant: Ant::new(ant_texture),
        }
    }
}

// call appropriate Nannou methods for the ant
impl Nannou for Model {
    fn display(&self, app: &App, _model: &Model) {
        self.ant.display(app, _model)
    }

    fn update(&mut self) {
        self.ant.update();
    }
}
