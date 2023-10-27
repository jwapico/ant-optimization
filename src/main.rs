use nannou::prelude::*;

mod ant;
mod model;

use crate::model::Model;

pub const WINDOW_SIZE: u32 = 512;

// for types that need to be drawn to the screen
trait Nannou {
    fn display(&self, app: &App, _model: &Model);
    fn update(&mut self);
}

// make and run the nannou app with the model and update functions
fn main() {
    nannou::app(model).update(update).run();
}

// make a new model. this is run once when the nannou app is started
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("red_ant.png");
    let ant_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model::new(_window, ant_texture)
}

// this is run on timed updates, 60 times per second
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update()
}

// draw to the screen
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let background_color = rgb(34.0 / 255.0, 40.0 / 255.0, 49.0 / 255.0);

    frame.clear(background_color);

    model.display(app, model);

    draw.to_frame(app, &frame).unwrap();
}
