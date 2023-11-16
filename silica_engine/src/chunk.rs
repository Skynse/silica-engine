use crate::{
    particle,
    prelude::{Particle, Variant, Wind, World, API, EMPTY_CELL},
};

// The point of chunks is to not process a chunk if
// particles in it haven't changed since the last tick.
#[derive(Clone)]
pub struct Chunk {
    pub particles: Vec<Particle>,
    pub wind: Vec<Wind>,
    pub dirty: bool,
    pub width: i32,
    pub height: i32,
}

impl Chunk {
    pub fn new(chunk_width: i32, chunk_height: i32) -> Self {
        let mut particles = Vec::new();
        let mut wind = Vec::new();

        // Iterate over each cell in the chunk
        for _ in 0..chunk_width {
            for _ in 0..chunk_height {
                particles.push(EMPTY_CELL);
                wind.push(Wind {
                    dx: 0,
                    dy: 0,
                    pressure: 0,
                    density: 0,
                });
            }
        }

        Self {
            particles,
            wind,
            dirty: false,
            width: chunk_width,
            height: chunk_height,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        if self.dirty {
            for x in 0..self.width {
                for y in 0..self.height {
                    let particle = self.get_particle(x as i32, y as i32);
                    let wind = self.get_wind(x as i32, y as i32);
                    World::blow_wind(
                        particle,
                        wind,
                        API {
                            x: x as i32,
                            y: y as i32,
                            chunk: self,
                            world,
                        },
                    );

                    Chunk::update_particle(
                        particle,
                        API {
                            x: x as i32,
                            y: y as i32,
                            chunk: self,
                            world,
                        },
                    );
                }
            }
            self.dirty = false;
        }
    }

    pub fn get_wind(&self, x: i32, y: i32) -> Wind {
        let idx = self.get_idx(x, y);
        self.wind[idx]
    }

    pub fn get_particle(&self, x: i32, y: i32) -> Particle {
        let idx = self.get_idx(x, y);
        self.particles[idx]
    }

    pub fn update_particle(particle: Particle, api: API) {
        // Your existing logic for updating particles
        particle.variant.update(particle, api);
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) {
        let idx = self.get_idx(x, y);
        self.particles[idx] = particle;
        self.dirty = true;
    }

    fn blow_wind(particle: Particle, wind: Wind, mut api: API) {
        // Your existing logic for updating particles based on wind
    }

    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        // get index of particle in chunk assuming 2d
        (y * self.width + x) as usize
    }
}
