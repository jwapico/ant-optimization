use nannou::prelude::*;

const WINDOW_SIZE: u32 = 1080;

// for types that need to be drawn to the screen
trait Nannou {
    fn display(&self, app: &App, _model: &Model);
    fn update(&mut self);
}

// position on the screen
#[derive(Default)]
struct Pos {
    x: f32,
    y: f32,
}

// direction in polar coordinates
#[derive(Default)]
struct Direction {
    speed: f32,
    degrees: f32,
}

// dimensions of a rectangle, defaults to 25px
struct Dimensions {
    w: i32,
    h: i32,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self { w: 15, h: 15 }
    }
}

// an ant is made up of a position, direction, size, and texture (ant png).
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
    // draws the Ant to the screen
    fn display(&self, app: &App, _model: &Model) {
        let draw = app.draw();

        draw.texture(&self.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.w as f32, self.size.h as f32);
    }

    // updates the Ant
    fn update(&mut self) {
        // change the x position by the x component of direction
        self.pos.x += deg_to_rad(self.dir.degrees).cos() * self.dir.speed;

        // change the y position by the y component of direction
        self.pos.y += deg_to_rad(self.dir.degrees).sin() * self.dir.speed;
    }
}

// describes the state of the app
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

//
// Nannou Interface
//

fn main() {
    nannou::app(model).update(update).run();
}

// make a new model. this is run once when the nannou app is started
fn model(app: &App) -> Model {
    // create the window
    let _window = app
        .new_window()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .build()
        .unwrap();

    // load the ant texture
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("red_ant.png");
    let ant_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    // return a new Model with the window and ant texture
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
