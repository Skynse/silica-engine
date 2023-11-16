use std::collections::HashSet;

use variant_type::{variant_type, VariantProperty};

use crate::{
    api::API,
    particle::{self, Particle},
    variant::{Variant, EMPTY_CELL},
    variant_type,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Wind {
    pub dx: i32,
    pub dy: i32,
    pressure: u8,
    density: u8,
}
pub struct World {
    pub(crate) particles: Vec<Particle>,
    pub wind: Vec<Wind>,
    pub width: i32,
    pub height: i32,
    pub running: bool,
    pub generation: u8,
    pub modified_indices: HashSet<usize>,
    pub cleared: bool,
}

impl Default for World {
    fn default() -> Self {
        Self::new(256, 256)
    }
}

impl World {
    pub fn tick(&mut self) {
        self.cleared = false;
        self.modified_indices = HashSet::new();
        if self.running {
            //wind
            self.paint_variants();
            for x in 0..self.width {
                for y in 0..self.height {
                    let particle = self.get_particle(x as i32, y as i32);
                    let wind = self.get_wind(x as i32, y as i32);
                    World::blow_wind(
                        particle,
                        wind,
                        API {
                            world: self,
                            x: x as i32,
                            y: y as i32,
                        },
                    );
                }
            }

            /*
            for x in 0..self.width {
                let scanx = if self.generation % 2 == 0 {
                    self.width - (x + 1)
                } else {
                    x
                };
                for y in 0..self.height {
                    let idx = self.get_idx(scanx as i32, y as i32);
                    let particle = self.get_particle(scanx as i32, y as i32);
                    World::update_particle(
                        particle,
                        API {
                            world: self,
                            x: scanx as i32,
                            y: y as i32,
                        },
                    );
                }
            }
            */

            // take iterator instead

            // iterate in reverse order
            for x in (0..self.width).rev() {
                for y in (0..self.height).rev() {
                    let idx = self.get_idx(x as i32, y as i32);
                    let left_to_right: bool = rand::random();
                    let particle = self.get_particle(x as i32, y as i32);

                    World::update_particle(
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

        self.generation = self.generation.wrapping_add(1);
        self.modified_indices.clear();
    }

    pub fn needs_update(&self) -> bool {
        self.cleared || !self.modified_indices.is_empty()
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn resume(&mut self) {
        self.running = true;
    }

    fn update_particle(particle: Particle, mut api: API) {
        if particle.variant == Variant::Empty {
            return;
        }
        println!("{:?}", particle.variant);
        match variant_type::variant_type(particle.variant).variant_property {
            VariantProperty::Powder => {
                // super basic falling sand impl
                let below = api.get(0, 1);
                let left = api.get(-1, 1);
                let right = api.get(1, 1);
                if below.variant == Variant::Empty {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }

                if below.variant == Variant::Water {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }

                if below.variant == Variant::SaltWater {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }
            }

            VariantProperty::Liquid => {
                // super basic falling sand impl
                let below = api.get(0, 1);
                let left = api.get(-1, 1);
                let right = api.get(1, 1);
                if below.variant == Variant::Empty {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }

                if below.variant == Variant::Water {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }

                if below.variant == Variant::SaltWater {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return;
                }
            }

            _ => (),
        }
        particle.variant.update(particle, api);
    }

    fn paint_variants(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_idx(x as i32, y as i32);
                let particle = self.get_particle(x as i32, y as i32);
                let variant = particle.variant;
                let color = particle::particle_to_color(variant);
                self.particles[idx].ra = color.0;
                self.particles[idx].rb = color.1;
                self.particles[idx].clock = self.generation;
            }
        }
    }
    pub fn new(width: i32, height: i32) -> World {
        let particles = (0..width * height).map(|_| EMPTY_CELL).collect();
        let wind: Vec<Wind> = (0..width * height)
            .map(|_| Wind {
                dx: 0,
                dy: 0,
                pressure: 0,
                density: 0,
            })
            .collect();
        World {
            particles,
            wind: wind,
            width: width,
            height: height,
            running: true,
            generation: 0,
            modified_indices: HashSet::new(),
            cleared: false,
        }
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn reset(&mut self) {
        for particle in self.particles.iter_mut() {
            *particle = Particle::new(Variant::Empty, 0, 0);
        }
        self.cleared = true;
        self.modified_indices.clear();
    }
}

impl World {
    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width as i32) as usize
    }

    pub fn get_wind(&self, x: i32, y: i32) -> Wind {
        let idx = self.get_idx(x, y);
        return self.wind[idx];
    }

    pub fn get_particle(&self, x: i32, y: i32) -> Particle {
        let idx = self.get_idx(x, y);
        self.particles[idx]
    }

    fn blow_wind(particle: Particle, wind: Wind, mut api: API) {
        if particle.clock == api.world.generation {
            return;
        }

        if particle.variant == Variant::Empty {
            return;
        }

        let mut dx = 0;
        let mut dy = 0;

        let thresh = match particle.variant {
            Variant::Empty => 500,
            Variant::Wall => 500,
            Variant::Sand => 30,
            Variant::Fire => 5,
            Variant::Smoke => 3,
            _ => 40,
        };

        let wx = (wind.dy as i32) - 126;
        let wy = (wind.dx as i32) - 126;
        if wx > thresh {
            dx = 1;
        }

        if wy > thresh {
            dy = 1;
        }

        if wx < -thresh {
            dx = -1;
        }

        if wy < -thresh {
            dy = -1;
        }

        if (dx != 0 || dy != 0) && api.get(dx, dy).variant == Variant::Empty {
            api.set(0, 0, EMPTY_CELL);
            if dy == -1
                && api.get(dx, -2).variant == Variant::Empty
                && (particle.variant == Variant::Sand || particle.variant == Variant::Water)
            {
                dy = -2;
            }

            api.set(dx, dy, particle);
        }
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
        self.modified_indices.insert(idx);
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
