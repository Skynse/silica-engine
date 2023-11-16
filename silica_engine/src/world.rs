use crate::{
    api::API,
    particle::Particle,
    variant::{Variant, EMPTY_CELL},
};

pub struct World {
    pub(crate) particles: Vec<Particle>,
    pub(crate) ambient_heat: u8,
    pub(crate) ambient_pressure: u8,
    pub(crate) ambient_wind: u8,

    pub width: usize,
    pub height: usize,
    pub running: bool,
}

impl Default for World {
    fn default() -> Self {
        Self::new(256, 256)
    }
}

impl World {
    pub fn tick(&mut self) {
        if self.running {
            for x in 0..self.width {
                for y in 0..self.height {
                    let particle = self.get(x, y);
                    particle.variant.update(
                        particle,
                        API {
                            world: self,
                            x: x as i32,
                            y: y as i32,
                        },
                    );
                }
            }
        }
    }
    pub fn new(width: usize, height: usize) -> World {
        let mut particles = vec![EMPTY_CELL; width * height];
        World {
            particles,
            ambient_heat: 0,
            ambient_pressure: 0,
            ambient_wind: 0,
            width: width,
            height: height,
            running: true,
        }
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return EMPTY_CELL;
        }
        self.particles[x as usize + y as usize * self.width]
    }

    pub fn reset(&mut self) {
        for particle in self.particles.iter_mut() {
            *particle = Particle::new(Variant::Empty, 0, 0);
        }
    }
}

impl World {
    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width as i32) as usize
    }

    pub fn get_particle(&self, x: i32, y: i32) -> Particle {
        if x >= self.width as i32 || y >= self.height as i32 {
            return Particle::new(Variant::Empty, 0, 0);
        }
        self.particles[self.width * y as usize + x as usize]
    }

    pub fn get_particle_mut(&mut self, x: i32, y: i32) -> &mut Particle {
        let idx = self.get_idx(x, y);
        &mut self.particles[idx]
    }

    pub fn swap_particles(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let idx1 = self.get_idx(x1, y1);
        let idx2 = self.get_idx(x2, y2);
        self.particles.swap(idx1, idx2);
    }

    pub fn set_particle(&mut self, x: i32, y: i32, variant: Variant) {
        let idx = self.get_idx(x, y);
        if idx >= self.particles.len() {
            return;
        }
        self.particles[idx] = Particle::new(variant, 0, 0);
    }

    pub fn set(&mut self, x: i32, y: i32, particle: Particle) {
        let idx = self.get_idx(x, y);
        self.particles[idx] = particle;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_set() {
        let world = World::new(100, 100);
        assert_eq!(world.get_particle(0, 0).get_variant(), Variant::Empty);
    }
}
