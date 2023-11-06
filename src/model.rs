use image::GenericImageView;
use nannou::prelude::*;
use std::collections::HashMap;

use crate::ant::Ant;
use crate::Nannou;

// possible states of a pixel on the map.
enum MapPixel {
    Open,
    Wall,
}

// describes the state of the app, a window and a vector of ants
pub struct Model {
    ants: Vec<Ant>,
    map: HashMap<(i32, i32), MapPixel>,
    pub background_texture: wgpu::Texture,
}

// add a way to make a new Model
impl Model {
    pub fn new(
        background_texture: wgpu::Texture,
        ant_texture: wgpu::Texture,
        num_ants: u32,
    ) -> Self {
        let ants = (0..num_ants)
            .map(|_| Ant::new(ant_texture.clone()))
            .collect();

        let map = generate_hashmap("assets/map.png");

        Self {
            ants,
            map,
            background_texture,
        }
    }
}

// call display and update on all the ants
impl Nannou for Model {
    fn display(&self, draw: &nannou::Draw) {
        self.ants.iter().for_each(|ant| ant.display(draw));
    }

    fn update(&mut self, window_dimensions: Vec2) {
        self.ants
            .iter_mut()
            .for_each(|ant| ant.update(window_dimensions));
    }
}

// reads in a picture of the game map, and based off pixel color builds a hashmap
// where pixel coordinates are the keys and MapPixel types are the values
fn generate_hashmap(img_path: &str) -> HashMap<(i32, i32), MapPixel> {
    let mut map = HashMap::new();
    let img = image::open(img_path).expect("Failed to open file.");

    for pixel in img.pixels() {
        // normalize the coords so (0, 0) is the middle of the screen
        let coords = (
            (pixel.0 as i32) - (img.width() as i32 / 2),
            (pixel.1 as i32) - (img.height() as i32 / 2),
        );

        let open_color = [155, 118, 83, 255];
        let wall_color = [99, 69, 44, 255];

        let pixel_color = [pixel.2[0], pixel.2[1], pixel.2[2], pixel.2[3]];

        if pixel_color == open_color {
            map.insert(coords, MapPixel::Open);
        } else if pixel_color == wall_color {
            map.insert(coords, MapPixel::Wall);
        }
    }

    map
}
