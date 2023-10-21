use nannou::prelude::*;
use rand::Rng;

use crate::model::Model;
use crate::Nannou;
use crate::WINDOW_SIZE;

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    pos: Vec2,
    size: Vec2,
    dir: Direction,
    texture: wgpu::Texture,
}

impl Ant {
    pub fn new(texture: wgpu::Texture) -> Self {
        Self {
            pos: Vec2::default(),
            size: Vec2::new(15.0, 15.0),
            dir: Direction::default(),
            texture,
        }
    }

    // if the ant goes out of bounds, change its direction to a valid random angle
    fn fix_wall_collision(&mut self) {
        let mut rng = rand::thread_rng();

        let lower_bound = -(WINDOW_SIZE as f32 / 2.0);
        let upper_bound = WINDOW_SIZE as f32 / 2.0;

        if self.pos.x < lower_bound {
            self.pos.x = lower_bound;
            self.dir.degrees = rng.gen_range(0.0..=180.0);
        } else if self.pos.y < lower_bound {
            self.pos.y = lower_bound;
            self.dir.degrees = rng.gen_range(-90.0..=90.0);
        } else if self.pos.x > upper_bound {
            self.pos.x = upper_bound;
            self.dir.degrees = rng.gen_range(-180.0..=0.0);
        } else if self.pos.y > upper_bound {
            self.pos.y = upper_bound;
            self.dir.degrees = rng.gen_range(90.0..=270.0);
        }
    }

    // TODO: Make wandering smoother
    fn wander(&mut self) {
        let mut rng = rand::thread_rng();

        let d_angle = rng.gen_range(-5.0..=5.0);
        self.dir.degrees += d_angle;

        let dx = deg_to_rad(self.dir.degrees).cos() * self.dir.speed;
        let dy = deg_to_rad(self.dir.degrees).sin() * self.dir.speed;
        self.pos.x += dx;
        self.pos.y += dy;

        self.fix_wall_collision();
    }
}

impl Nannou for Ant {
    // draws the Ant to the screen
    fn display(&self, app: &App, _model: &Model) {
        let draw = app.draw();

        draw.texture(&self.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.x, self.size.y)
            .rotate(deg_to_rad(self.dir.degrees) + 3.0 * PI / 2.0);
    }

    // updates the Ant
    fn update(&mut self) {
        self.wander();
    }
}

// direction in polar coordinates
struct Direction {
    speed: f32,
    degrees: f32,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            speed: 0.5,
            degrees: 0.0,
        }
    }
}
