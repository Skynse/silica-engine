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

#[derive(Clone, Copy, PartialEq)]
pub struct Environment {
    pub pressure: f32,
    pub ambient_temperature: f32,
    pub ambient_pressure: f32,
}
pub struct World {
    pub(crate) particles: Vec<Particle>,
    pub wind: Vec<Wind>,
    pub environment: Vec<Environment>,
    pub width: i32,
    pub height: i32,
    pub running: bool,
    pub generation: u8,
    pub modified_indices: HashSet<usize>,
    pub cleared: bool,
    pub modified_state: bool,
}

impl Default for World {
    fn default() -> Self {
        Self::new(256, 256)
    }
}

impl World {
    pub fn tick(&mut self) {
        self.cleared = false;
        if self.running {
            /*
                        for x in 0..self.width {
                            for y in 0..self.height {
                                let temp = self.get_temperature(x, y);
                                // share heat with neighbors divided by 4
                                // DO NOT USE WIND OR DENSITY
                                let mut heat = temp / 4.;
                                let pressure = self.get_pressure(x, y);

                                let nbr = self.get_particle(x, y);
                                let nbb = self.get_particle(x, y + 1);
                                let nbl = self.get_particle(x - 1, y);
                                let nbr = self.get_particle(x + 1, y);

                                // if a particle is above, take average its temperature
                                // set temeprature for this location to that average
                                // only do for above for now
                                // decrease environment heat over time
                                self.set_temperature(x, y, temp - 0.1);
                                // clamp temp
                                if temp < 0. {
                                    self.set_temperature(x, y, 0.);
                                }

                                if nbr.variant != Variant::Empty {
                                    let nbr_temp = self.get_temperature(x + 1, y);
                                    heat += nbr_temp / 4.;
                                }

                                let nbr_type = &variant_type::variant_type(nbr.variant).variant_property;
                                let base_temp = variant_type::variant_type(nbr.variant).base_temperature;

                                self.set_pressure(x, y, pressure);
                            }
                        }
            */
            //wind
            /*
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
            */
            //  self.generation = self.generation.wrapping_add(1);
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
            for x in (0..self.width - 1).rev() {
                let scanx = if self.generation % 2 == 0 {
                    self.width - (x + 1)
                } else {
                    x
                };
                for y in (0..self.height).rev() {
                    let idx = self.get_idx(scanx as i32, y as i32);

                    let particle = self.get_particle(scanx as i32, y as i32);
                    if particle.modified {
                        self.modified_indices.insert(idx);
                    }

                    self.modified_state = World::update_particle(
                        particle,
                        API {
                            world: self,
                            x: scanx as i32,
                            y: y as i32,
                        },
                    );

                    let idx = self.get_idx(x as i32, y as i32);
                    let mut temperature = self.get_temperature(x, y);

                    // Adjust temperature towards the baseline value
                    let baseline_temperature = 22.0;
                    let temperature_decay_rate = 0.1; // Adjust this rate based on your preference

                    temperature += (baseline_temperature - temperature) * temperature_decay_rate;

                    // Share temperature with neighbors
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx != 0 || dy != 0 {
                                let nbr_x = (x as i32 + dx).clamp(0, self.width - 1) as i32;
                                let nbr_y = (y as i32 + dy).clamp(0, self.height - 1) as i32;

                                let nbr_temperature = self.get_temperature(nbr_x, nbr_y);
                                temperature += (nbr_temperature - temperature) * 0.25;
                                // Adjust this rate based on your preference
                            }
                        }
                    }

                    self.set_temperature(x, y, temperature);
                }
            }

            self.generation = self.generation.wrapping_add(1);
            self.modified_indices.clear();
        }
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

    fn update_particle(mut particle: Particle, mut api: API) -> bool {
        if particle.clock > api.world.generation {
            return false;
        }

        if variant_type::variant_type(particle.variant).has_flag(variant_type::FLAG_IMMUTABLE) {
            return false;
        }

        match variant_type::variant_type(particle.variant).variant_property {
            VariantProperty::Powder => {
                // super basic falling sand impl
                let below = api.get(0, 1);
                let left = api.get(-1, 1);
                let right = api.get(1, 1);
                if below.variant == Variant::Empty {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return true;
                }

                if left.variant == Variant::Empty && right.variant == Variant::Empty {
                    let dir = api.rand_dir();
                    api.set(dir, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return true;
                }
            }

            VariantProperty::Liquid => {
                // super basic falling and spreading water impl
                let below = api.get(0, 1);
                let below_left = api.get(-1, 1);
                let below_right = api.get(1, 1);
                let idx = api.world.get_idx(api.x, api.y);

                // Move down if the cell below is empty
                if below.variant == Variant::Empty {
                    api.set(0, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                    return true;
                }
                // Try to spread horizontally if there's space to the left or right
                else if below_left.variant == Variant::Empty {
                    // swap with below left
                    // swap current index with below left
                    api.world.particles.swap(idx, idx - 1);
                    return true;
                } else if below_right.variant == Variant::Empty {
                    // swap with below right
                    // swap current index with below right
                    api.world.particles.swap(idx, idx + 1);
                    return true;
                }

                // water can also move two cells down if there's space
            }

            VariantProperty::Gas => {
                let (dx, dy) = api.rand_vec();

                let nbr = api.get(dx, dy);
                // api.set_fluid(Wind {
                //     dx: 0,
                //     dy: 0,
                //     pressure: 5,
                //     density: 0,
                // });
                if particle.rb == 0 {
                    api.set(0, 0, Particle { rb: 5, ..particle });
                }

                let nbr_type = &variant_type::variant_type(nbr.variant).variant_property;

                if nbr.variant == Variant::Empty {
                    if particle.rb < 3 {
                        //single molecule
                        api.set(0, 0, EMPTY_CELL);
                        api.set(dx, dy, particle);
                    } else {
                        api.set(0, 0, Particle { rb: 1, ..particle });
                        api.set(
                            dx,
                            dy,
                            Particle {
                                rb: particle.rb - 1,
                                ..particle
                            },
                        );
                    }
                } else if (dx != 0 || dy != 0) && *nbr_type == VariantProperty::Gas && nbr.rb < 4 {
                    // if (cell.rb < 2) {
                    api.set(0, 0, EMPTY_CELL);
                    // }
                    api.set(
                        dx,
                        dy,
                        Particle {
                            rb: nbr.rb + particle.rb,
                            ..particle
                        },
                    );
                }
                // spread opinion
            }

            _ => (),
        }
        let state = &particle.update(api);
        particle.modified = *state;
        *state
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

        let environment: Vec<Environment> = (0..width * height)
            .map(|_| Environment {
                pressure: 0.,
                ambient_temperature: 0.,
                ambient_pressure: 0.,
            })
            .collect();
        World {
            particles,
            wind: wind,
            environment: environment,
            width: width,
            height: height,
            running: true,
            generation: 0,
            modified_indices: HashSet::new(),
            cleared: false,
            modified_state: false,
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

    pub fn get_temperature(&self, x: i32, y: i32) -> f32 {
        let idx = self.get_idx(x, y);
        self.environment[idx].ambient_temperature
    }

    pub fn get_pressure(&self, x: i32, y: i32) -> f32 {
        let idx = self.get_idx(x, y);
        self.environment[idx].ambient_pressure
    }

    pub fn get_wind(&self, x: i32, y: i32) -> Wind {
        let idx = self.get_idx(x, y);
        return self.wind[idx];
    }

    pub fn get_particle(&self, x: i32, y: i32) -> Particle {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return EMPTY_CELL;
        }
        let idx = self.get_idx(x, y);
        self.particles[idx]
    }

    pub fn toggle_modified_state(&mut self) -> bool {
        self.modified_state = !self.modified_state;
        self.modified_state
    }

    pub fn add_heat(&mut self, x: i32, y: i32, heat: f32) {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return;
        }
        let idx = self.get_idx(x, y);
        self.particles[idx].temperature += heat;
    }

    pub fn is_modified(&self) -> bool {
        self.modified_state
    }

    fn blow_wind(particle: Particle, wind: Wind, mut api: API) {
        if particle.clock > api.world.generation {
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

    pub fn set_pressure(&mut self, x: i32, y: i32, pressure: f32) {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return;
        }
        let idx = self.get_idx(x, y);
        self.environment[idx].pressure = pressure;
    }

    pub fn set_temperature(&mut self, x: i32, y: i32, temperature: f32) {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return;
        }

        let idx = self.get_idx(x, y);
        self.environment[idx].ambient_temperature = temperature;
    }

    pub fn set_particle(&mut self, x: i32, y: i32, variant: Variant) {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return;
        }

        // if the particle is already set to the same variant, don't do anything
        if self.get_particle(x, y).variant == variant {
            return;
        }

        // if the particle is already a wall, don't do anything
        if self.get_particle(x, y).variant == Variant::Wall {
            return;
        }

        let idx = self.get_idx(x, y);
        if idx >= self.particles.len() {
            return;
        }
        self.particles[idx] = Particle::new(variant, 0, 0);
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
