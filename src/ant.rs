use nannou::prelude::*;
use rand::Rng;

use crate::model::Model;
use crate::Nannou;
use crate::WINDOW_SIZE;

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    size: Vec2,
    pos: Vec2,
    current_vel: Vec2,
    desired_vel: Vec2,
    mass: f32,
    max_vel: f32,
    max_force: f32,
    wander_angle: f32,
    texture: wgpu::Texture,
}

impl Ant {
    pub fn new(texture: wgpu::Texture) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            size: Vec2::new(15.0, 15.0),
            pos: Vec2::new(0.0, 0.0),
            current_vel: Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)),
            desired_vel: Vec2::new(0.0, 0.0),
            mass: 1.5,
            max_vel: 1.0,
            max_force: 0.02,
            wander_angle: 1.0,
            texture,
        }
    }

    // moves the ant towards a target
    fn seek(&mut self, target: Vec2, slowing_radius: f32) {
        let new_target = self.validate_target(target);

        self.desired_vel = new_target - self.pos;

        // if the ant enters a specefied radius, slow it down so it stops at its target
        let distance = self.desired_vel.length();
        if distance < slowing_radius {
            self.desired_vel =
                self.desired_vel.normalize() * self.max_vel * (distance / slowing_radius);
        } else {
            self.desired_vel = self.desired_vel.normalize() * self.max_vel;
        }

        // steering is desired velocity - current velocity scaled by a force and a mass
        let mut steering = self.desired_vel - self.current_vel;
        truncate(&mut steering, self.max_force);
        steering /= self.mass;

        self.current_vel += steering;
        truncate(&mut self.current_vel, self.max_vel);
        self.pos += self.current_vel;
    }

    // makes the ant randomly wander
    fn wander(&mut self) {
        const WANDER_DISTANCE: f32 = 30.0; // disatnce from ant to wandering circle center
        const WANDER_RADIUS: f32 = 15.0; // radius of wandering circle
        const ANGLE_CHANGE: f32 = PI / 16.0; // max change in wander angle

        let circle_center = self.current_vel.normalize() * WANDER_DISTANCE;

        self.wander_angle += self.generate_wander_angle(ANGLE_CHANGE);

        let displacement = Vec2::new(
            self.wander_angle.cos() * WANDER_RADIUS,
            self.wander_angle.sin() * WANDER_RADIUS,
        );

        let target = self.pos + circle_center + displacement;
        self.seek(target, 0.0);
    }

    // TODO: Figure out why this works??? Why does removing validate_target break it?
    // TODO: Refactor and comment
    fn generate_wander_angle(&self, angle_change: f32) -> f32 {
        const EDGE_THRESHOLD: f32 = 10.0;

        let lower_bound = (-(WINDOW_SIZE as i32) / 2) as f32 + EDGE_THRESHOLD;
        let upper_bound = (WINDOW_SIZE as i32 / 2) as f32 - EDGE_THRESHOLD;

        if self.pos.x <= lower_bound {
            deg_to_rad(rand::thread_rng().gen_range(90..=270) as f32)
        } else if self.pos.x >= upper_bound {
            deg_to_rad(rand::thread_rng().gen_range(-270..=-90) as f32)
        } else if self.pos.y <= lower_bound {
            deg_to_rad(rand::thread_rng().gen_range(0..=180) as f32)
        } else if self.pos.y >= upper_bound {
            deg_to_rad(rand::thread_rng().gen_range(-180..=0) as f32)
        } else {
            rand::thread_rng().gen_range(-angle_change..angle_change)
        }
    }

    fn validate_target(&self, target: Vec2) -> Vec2 {
        let mut new_target = target;

        let lower_bound = (-(WINDOW_SIZE as i32) / 2) as f32;
        let upper_bound = (WINDOW_SIZE as i32 / 2) as f32;

        if target.x < lower_bound || target.x > upper_bound {
            new_target.x = (lower_bound + upper_bound) / 2.0; // Move target towards the center on the x-axis
        }

        if target.y < lower_bound || target.y > upper_bound {
            new_target.y = (lower_bound + upper_bound) / 2.0; // Move target towards the center on the y-axis
        }

        new_target
    }

    // TODO: Spawn a bunch of ants
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
        // self.seek(mouse_pos, 35.0);
        self.wander();
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
