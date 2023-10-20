use nannou::prelude::*;

// Things that can be drawn to the screen
trait Nannou {
    fn display(&self, app: &App, _model: &Model);
    fn update(&mut self);
}

// A position on the screen
struct Pos {
    x: f32,
    y: f32,
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

// The dimensions of a rectangle
struct Dimensions {
    w: i32,
    h: i32,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self { w: 25, h: 25 }
    }
}

// A direction of movement, represented in polar coordinates
struct Direction {
    speed: f32,
    degrees: f32,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            speed: 0.0,
            degrees: 0.0,
        }
    }
}

// A single ant
struct Ant {
    pos: Pos,
    size: Dimensions,
    dir: Direction,
    texture: wgpu::Texture,
}

impl Ant {
    fn new(texture: wgpu::Texture) -> Self {
        Self {
            pos: Pos::default(),
            size: Dimensions::default(),
            dir: Direction::default(),
            texture,
        }
    }
}

impl Nannou for Ant {
    fn display(&self, app: &App, _model: &Model) {
        let draw = app.draw();

        draw.texture(&self.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.w as f32, self.size.h as f32);
    }

    fn update(&mut self) {
        self.pos.x += deg_to_rad(self.dir.degrees).cos() * self.dir.speed;
        self.pos.y += deg_to_rad(self.dir.degrees).sin() * self.dir.speed;
    }
}

// The model of the nannou app
struct Model {
    _window: window::Id,
    ant: Ant,
}

impl Model {
    fn new(_window: window::Id, ant_texture: wgpu::Texture) -> Self {
        Self {
            _window,
            ant: Ant::new(ant_texture),
        }
    }
}

impl Nannou for Model {
    fn display(&self, app: &App, _model: &Model) {
        self.ant.display(app, _model)
    }

    fn update(&mut self) {
        self.ant.update();
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

//
// Nannou Interface
//

// Initialize the Model and perform setup tasks. This is run once on startup
fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("red_ant.png");
    let ant_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model::new(_window, ant_texture)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let background_color = rgb(34.0 / 255.0, 40.0 / 255.0, 49.0 / 255.0);

    frame.clear(background_color);

    model.display(app, model);

    draw.to_frame(app, &frame).unwrap();
}
