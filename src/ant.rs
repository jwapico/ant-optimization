use libm::*;
use nannou::prelude::*;

use crate::model::Model;
use crate::Nannou;
use crate::WINDOW_SIZE;

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    size: Vec2,
    mass: f32,
    pos: Vec2,
    current_vel: Vec2,
    desired_vel: Vec2,
    max_vel: f32,
    max_force: f32,
    texture: wgpu::Texture,
}

impl Ant {
    pub fn new(texture: wgpu::Texture) -> Self {
        Self {
            size: Vec2::new(15.0, 15.0),
            pos: Vec2::new(0.0, 0.0),
            current_vel: Vec2::new(1.0, 1.0),
            desired_vel: Vec2::new(0.0, 0.0),
            mass: 1.2,
            max_vel: 1.0,
            max_force: 0.017,
            texture,
        }
    }

    // if the ant goes out of bounds, change its direction to a valid random angle
    fn fix_wall_collision(&mut self) {
        // let lower_bound = -(WINDOW_SIZE as f32 / 2.0);
        // let upper_bound = WINDOW_SIZE as f32 / 2.0;

        // TODO:
    }

    fn seek(&mut self, pos: Vec2) {
        self.desired_vel = pos - self.pos;
        self.desired_vel = self.desired_vel.normalize() * self.max_vel;

        // steering is desired velocity - current velocity scaled by a force and a mass
        let mut steering = self.desired_vel - self.current_vel;
        truncate(&mut steering, self.max_force);
        steering /= self.mass;

        self.current_vel += steering;
        truncate(&mut self.current_vel, self.max_vel);
        self.pos += self.current_vel;
    }

    fn wander(&mut self) {
        // TODO:
    }
}

impl Nannou for Ant {
    // draws the Ant to the screen
    fn display(&self, app: &App, _model: &Model) {
        let draw = app.draw();
        let angle = self.current_vel.y.atan2(self.current_vel.x);

        draw.texture(&self.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.x, self.size.y)
            .rotate(angle - PI / 2.0);
    }

    // updates the Ant
    fn update(&mut self, mouse_pos: Vec2) {
        self.seek(mouse_pos);
    }
}

fn truncate(vec: &mut Vec2, max_length: f32) {
    let length = vec.length();
    if length > max_length {
        let scale = max_length / length;
        vec.x *= scale;
        vec.y *= scale;
    }
}
