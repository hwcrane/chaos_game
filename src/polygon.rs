use crate::vec2::Vec2;
use color_space::{FromColor, Rgb};
use rand::prelude::*;

#[derive(Debug)]
pub struct Polygon {
    n: usize,
    pub verticies: Vec<Vec2>,
    pub colours: Vec<Rgb>,
    last_vertex: usize,
    twice: bool,
}

impl Polygon {
    pub fn new(n: usize) -> Polygon {
        let interval = 360. / n as f64;
        let verticies: Vec<Vec2> = (0..n).map(|i| Vec2::unit(interval * i as f64)).collect();
        let colours: Vec<color_space::Rgb> = (0..n)
            .map(|i| {
                color_space::Rgb::from_color(&color_space::Hsv::new(i as f64 * interval, 1., 1.))
            })
            .collect();
        println!("{:?}", colours);
        Polygon {
            verticies,
            colours,
            n,
            last_vertex: usize::MAX,
            twice: false,
        }
    }

    pub fn random_vertex(&mut self) -> usize {
        let mut rng = thread_rng();

        let mut idx = rng.gen_range(0..self.n);
        // if self.twice {
        //     while (idx + 1) % self.n == self.last_vertex ||  (self.last_vertex + 1) % self.n == idx {
        //         idx = rng.gen_range(0..self.n);
        //     }
        //     self.twice = false
        // }
        // if idx == self.last_vertex {
        //     self.twice = true
        // }
        // while idx == self.last_vertex {
        //     idx = rng.gen_range(0..self.n);
        // }
        self.last_vertex = idx;
        idx
    }
}
