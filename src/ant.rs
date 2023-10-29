use libm::*;
use nannou::prelude::*;

use crate::model::Model;
use crate::Nannou;
use crate::WINDOW_SIZE;

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    texture: wgpu::Texture,
}

impl Ant {
    pub fn new(texture: wgpu::Texture) -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(1.0, 1.0),
            size: Vec2::new(15.0, 15.0),
            texture,
        }
    }

    // if the ant goes out of bounds, change its direction to a valid random angle
    fn fix_wall_collision(&mut self) {
        let lower_bound = -(WINDOW_SIZE as f32 / 2.0);
        let upper_bound = WINDOW_SIZE as f32 / 2.0;

        // TODO:
    }

    // TODO: Make wandering smoother
    fn wander(&mut self, mouse_pos: Vec2) {
        let direction = mouse_pos - self.pos;
        let direction_unit = direction.normalize();

        self.vel = direction_unit * 1.5;
        self.pos += self.vel;

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
            .rotate(atan2f(self.pos.y, self.pos.x) - PI / 2.0);
    }

    // updates the Ant
    fn update(&mut self, mouse_pos: Vec2) {
        self.wander(mouse_pos);
    }
}
