use crate::model::Model;
use nannou::prelude::*;

mod ant;
mod model;

const WINDOW_SIZE: u32 = 1024;
const NUM_ANTS: u32 = 225;

// for types that need to be drawn to the screen
trait Nannou {
    fn display(&self, draw: &nannou::Draw);
    fn update(&mut self, window_dimensions: Vec2);
}

// make and run the nannou app with the model and update functions
fn main() {
    nannou::app(model).update(update).run();
}

// make a new model. this is ran once when the nannou app is started
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();

    let background_path = assets.join("map.png");
    let background_texture = wgpu::Texture::from_path(app, background_path).unwrap();

    let img_path = assets.join("red_ant.png");
    let ant_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model::new(background_texture, ant_texture, NUM_ANTS)
}

// required function for the nannou app; is run 60 times per second
fn update(_app: &App, model: &mut Model, _update: Update) {
    let window_dimensions = _app.window_rect().wh();

    model.update(window_dimensions)
}

// required function for the nannou app; draws to the screen
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let background_color = rgb(155.0 / 255.0, 118.0 / 255.0, 83.0 / 255.0);

    draw.texture(&model.background_texture)
        .x_y(0.0, 0.0)
        .w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32);

    frame.clear(background_color);

    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
