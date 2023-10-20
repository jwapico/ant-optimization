use nannou::prelude::*;

// Things that can be drawn to the screen
trait Nannou {
    fn display(&self, app: &App, model: &Model);
    fn update(&mut self);
}

// A position on the screen
struct Pos {
    x: f32,
    y: f32,
}

// The dimensions of a rectangle
struct Dimensions {
    w: i32,
    h: i32,
}

// A direction of movement, represented in polar coordinates
struct Direction {
    speed: f32,
    angle: f32,
}

// A single ant
struct Ant {
    pos: Pos,
    size: Dimensions,
    dir: Direction,
}

impl Nannou for Ant {
    fn display(&self, app: &App, model: &Model) {
        let draw = app.draw();

        draw.texture(&model.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.w as f32, self.size.h as f32);
    }

    fn update(&mut self) {}
}

// The model of the nannou app
struct Model {
    texture: wgpu::Texture,
    last_update_time: std::time::Instant,
    update_interval: std::time::Duration,
    ant: Ant,
}

impl Nannou for Model {
    fn display(&self, app: &App, model: &Model) {}

    fn update(&mut self) {}
}

fn main() {
    nannou::app(model).update(update).run();
}

// Initialize the Model and perform setup tasks. This is run once on startup
fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("red_ant.png");
    let ant_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    let ant = Ant {
        pos: Pos { x: 0.0, y: 0.0 },
        size: Dimensions { w: 30, h: 30 },
        dir: Direction {
            speed: 10.0,
            angle: 0.0,
        },
    };

    Model {
        texture: ant_texture,
        ant,
        update_interval: std::time::Duration::from_millis(100), // Adjust the interval as needed
        last_update_time: std::time::Instant::now(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let current_time = std::time::Instant::now();

    if current_time - model.last_update_time >= model.update_interval {
        model.last_update_time = current_time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let background_color = rgb(34.0 / 255.0, 40.0 / 255.0, 49.0 / 255.0);

    frame.clear(background_color);

    draw.texture(&model.texture)
        .x_y(model.ant.pos.x, model.ant.pos.y)
        .w_h(model.ant.size.w as f32, model.ant.size.h as f32);

    draw.to_frame(app, &frame).unwrap();
}
