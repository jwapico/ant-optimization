use nannou::prelude::*;
use rand::Rng;

use crate::model::Model;
use crate::Nannou;

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    size: Vec2,
    mass: f32,
    pos: Vec2,
    current_vel: Vec2,
    desired_vel: Vec2,
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
            wander_angle: 1.0,
            max_force: 0.02,
            texture,
        }
    }

    // moves the ant towards a target
    fn seek(&mut self, target: Vec2, slowing_radius: f32) {
        self.desired_vel = target - self.pos;

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
        const ANGLE_CHANGE: f32 = 0.25; // max change in wander angle

        let circle_center = self.current_vel.normalize() * WANDER_DISTANCE;

        self.wander_angle += rand::thread_rng().gen_range(-ANGLE_CHANGE..ANGLE_CHANGE);

        let displacement = Vec2::new(
            self.wander_angle.cos() * WANDER_RADIUS,
            self.wander_angle.sin() * WANDER_RADIUS,
        );

        let target = self.pos + circle_center + displacement;
        self.seek(target, 0.0);
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

fn set_angle(vec: &mut Vec2, angle: f32) {
    let len = vec.length();
    vec.x = angle.cos() * len;
    vec.y = angle.sin() * len;
}
