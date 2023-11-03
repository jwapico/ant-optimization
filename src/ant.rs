use nannou::prelude::*;
use rand::Rng;

use crate::Nannou;

struct Bounds {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    edge_threshold: f32,
}

impl Bounds {
    fn new(left: f32, right: f32, bottom: f32, top: f32, edge_threshold: f32) -> Self {
        let left_bound = left + edge_threshold;
        let right_bound = right - edge_threshold;

        let bottom_bound = bottom + edge_threshold;
        let top_bound = top - edge_threshold;

        Self {
            left: left_bound,
            right: right_bound,
            bottom: bottom_bound,
            top: top_bound,
            edge_threshold,
        }
    }
}

// an ant is made up of a position, direction, size, and texture (ant png).
pub struct Ant {
    size: Vec2,
    pos: Vec2,
    current_vel: Vec2,
    desired_vel: Vec2,
    max_force: f32,
    mass: f32,
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
            max_force: 0.02,
            mass: 2.0,
            wander_angle: 1.0,
            texture,
        }
    }

    // moves the ant towards a target
    fn seek(&mut self, target: Vec2, slowing_radius: f32, bounds: &Bounds) {
        let new_target = self.validate_target(target, bounds);

        self.desired_vel = new_target - self.pos;

        // if the ant enters a specefied radius, slow it down so it stops at its target
        let distance = self.desired_vel.length();
        if distance < slowing_radius {
            self.desired_vel = self.desired_vel.normalize() * (distance / slowing_radius);
        } else {
            self.desired_vel = self.desired_vel.normalize();
        }

        // steering is desired velocity - current velocity scaled by a force and a mass
        let mut steering = self.desired_vel - self.current_vel;
        truncate(&mut steering, self.max_force);
        steering /= self.mass;

        self.current_vel += steering;
        self.pos += self.current_vel;
    }

    // makes the ant randomly wander
    fn wander(&mut self, window_dimensions: Vec2) {
        const WANDER_DISTANCE: f32 = 30.0; // distance from ant to wandering circle center
        const WANDER_RADIUS: f32 = 15.0; // radius of wandering circle
        const ANGLE_CHANGE: f32 = PI / 16.0; // max change in wander angle
        const EDGE_THRESHOLD: f32 = 10.0; // max distance from the edge

        let bounds = Bounds::new(
            -(window_dimensions.x / 2.0),
            window_dimensions.x / 2.0,
            -(window_dimensions.y / 2.0),
            window_dimensions.y / 2.0,
            EDGE_THRESHOLD,
        );

        let circle_center = self.current_vel.normalize() * WANDER_DISTANCE;

        self.wander_angle += self.generate_wander_angle(ANGLE_CHANGE, &bounds);

        let displacement = Vec2::new(
            self.wander_angle.cos() * WANDER_RADIUS,
            self.wander_angle.sin() * WANDER_RADIUS,
        );

        let target = self.pos + circle_center + displacement;
        self.seek(target, 0.0, &bounds);
    }

    // generates a random wander angle. If the ants are near the edge they are nudged towards the center
    fn generate_wander_angle(&self, angle_change: f32, bounds: &Bounds) -> f32 {
        let new_threshold = bounds.edge_threshold * 2.0;

        // the ants tend to hug the edges, this nudges them away from the edge
        if self.pos.x <= bounds.left + new_threshold {
            return deg_to_rad(rand::thread_rng().gen_range(90..=270) as f32);
        }

        if self.pos.x >= bounds.right - new_threshold {
            return deg_to_rad(rand::thread_rng().gen_range(-270..=-90) as f32);
        }
        if self.pos.y <= bounds.bottom + new_threshold {
            return deg_to_rad(rand::thread_rng().gen_range(0..=180) as f32);
        }

        if self.pos.y >= bounds.top - new_threshold {
            return deg_to_rad(rand::thread_rng().gen_range(-180..=0) as f32);
        }

        rand::thread_rng().gen_range(-angle_change..angle_change)
    }

    // if the target is out of bounds, change it to point towards the origin
    fn validate_target(&self, target: Vec2, bounds: &Bounds) -> Vec2 {
        let mut new_target = target;

        if target.x < bounds.left || target.x > bounds.right {
            new_target.x = 0.0;
        }

        if target.y < bounds.bottom || target.y > bounds.top {
            new_target.y = 0.0;
        }

        new_target
    }
}

impl Nannou for Ant {
    // draws the Ant to the screen
    fn display(&self, draw: &nannou::Draw) {
        let angle = self.current_vel.y.atan2(self.current_vel.x);

        draw.texture(&self.texture)
            .x_y(self.pos.x, self.pos.y)
            .w_h(self.size.x, self.size.y)
            .rotate(angle - PI / 2.0);
    }

    // updates the Ant
    fn update(&mut self, window_dimensions: Vec2) {
        self.wander(window_dimensions);
    }
}

// truncates a vector to max_length
fn truncate(vec: &mut Vec2, max_length: f32) {
    let length = vec.length();
    if length > max_length {
        let scale = max_length / length;
        vec.x *= scale;
        vec.y *= scale;
    }
}
