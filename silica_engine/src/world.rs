use std::{
    collections::HashSet,
    io::{BufWriter, Read, Write},
    path::Path,
};

use crate::{
    api::API,
    particle::{self, Particle},
    variant::{Variant, EMPTY_CELL},
    variant_type, Serialize,
};

use std::fs::File;
use variant_type::{VariantProperty, VariantType, EMPTY, FLAG_ALIVE};

pub const GRAVITY: f32 = 10f32;
pub const SPREAD_FACTOR: f32 = 0.1f32;

#[derive(Clone, Copy, PartialEq)]
pub struct Environment {
    pub pressure: f32,
    pub ambient_temperature: f32,
    pub ambient_pressure: f32,
}

pub struct World {
    pub(crate) particles: Vec<Particle>,
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

impl Serialize for World {
    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];
        res.extend_from_slice(&self.generation.to_le_bytes());

        for particle in self.particles.iter() {
            res.extend_from_slice(&particle.serialize());
        }
        res
    }

    fn deserialize(bytes: &[u8]) -> Self {
        let mut generation_bytes = [0; 1];

        generation_bytes.copy_from_slice(&bytes[8..9]);

        let generation = u8::from_le_bytes(generation_bytes);

        let mut particles = Vec::new();
        for i in 9..bytes.len() {
            let mut particle_bytes = [0; 36];
            particle_bytes.copy_from_slice(&bytes[i..i + 36]);
            particles.push(Particle::deserialize(&particle_bytes));
        }

        Self {
            particles,
            environment: vec![],
            generation,
            cleared: false,
            ..Default::default()
        }
    }
}

impl World {
    pub fn save_to_slc(&mut self, path: &str) {
        let mut p = String::from(path);

        // add slc extension if not present
        if !p.ends_with(".slc") {
            p.push_str(".slc");
        }
        println!("saving to {}.slc", path);

        let mut w =
            brotli::CompressorWriter::new(BufWriter::new(File::create(p).unwrap()), 4096, 11, 22);
        w.write_all(&self.serialize()).unwrap();
    }

    pub fn load_from_slc(&mut self, path: &str) {
        println!("loaded from {}", path);
        let mut decoder = brotli::Decompressor::new(File::open(path).unwrap(), 4096);
        let mut buf = vec![];
        decoder.read_to_end(buf.as_mut()).unwrap();
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_idx(x, y);
                let particle = Particle::deserialize(&buf[i..i + 36]);
                self.particles[idx] = particle;
                i += 36;
            }
        }
    }
    pub fn save(&self, path: &str) {
        let mut p = String::from(path);

        let mut image = Vec::new();
        for particle in self.particles.iter() {
            let color = particle::particle_to_color(*particle).to_rgb8();
            image.push(color.0);
            image.push(color.1);
            image.push(color.2);
        }

        // add png extension if not present
        if !p.ends_with(".png") {
            p.push_str(".png");
        }

        let ref mut w = BufWriter::new(File::create(p).unwrap());
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&image).unwrap(); // Save
        println!("saving to {}.png", path);
    }

    pub fn load(&mut self, path: &str) {
        println!("loaded from {}", path);
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_idx(x, y);
                let r = buf[i];
                let g = buf[i + 1];
                let b = buf[i + 2];
                self.particles[idx] = particle::color_to_particle((r, g, b));
                i += 3;
            }
        }
    }

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

                                if nbr.get_variant() != Variant::Empty {
                                    let nbr_temp = self.get_temperature(x + 1, y);
                                    heat += nbr_temp / 4.;
                                }

                                let nbr_type = &variant_type::variant_type(nbr.get_variant()).variant_property;
                                let base_temp = variant_type::variant_type(nbr.get_variant()).base_temperature;

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
            for x in (0..self.width).rev() {
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
                        continue;
                    }

                    // self.distribute_cell_temperature(x, y);

                    self.modified_state = World::update_particle(
                        particle,
                        API {
                            world: self,
                            x: scanx as i32,
                            y: y as i32,
                        },
                    );
                    if self.modified_state {
                        self.modified_indices.insert(idx);
                    }
                } //end gen
            }

            // ambient heat calculation
            // take a 3x3 grid and share temp based on center temp until all cells are the same

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
        if particle.modified {
            return false;
        }

        if particle.get_variant() == Variant::Empty && !particle.variant_type.has_flag(FLAG_ALIVE) {
            return false;
        }

        if particle.variant_type.has_flag(variant_type::FLAG_IMMUTABLE) {
            return false;
        }

        // decrease temperature over time to variant base temperature
        let temperature = particle.temperature;
        let base_temperature = particle.variant_type.base_temperature;
        let temperature_decay_rate = 0.01; // Adjust this rate based on your preference

        particle.temperature += (base_temperature - temperature) * temperature_decay_rate;
        let _type = particle.variant_type;
        match particle.variant_type.variant_property {
            VariantProperty::Powder => {
                let dx = api.rand_dir();
                let nbr = api.get(0, 1);

                if nbr.get_variant() == Variant::Empty {
                    api.set(dx, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                } else if api.get(dx, 1).get_variant() == Variant::Empty {
                    api.set(dx, 1, particle);
                    api.set(0, 0, EMPTY_CELL);
                } else if nbr.variant_type.variant_property == VariantProperty::Liquid {
                    api.set(0, 0, nbr);
                    api.set(0, 1, particle);
                } else {
                    let weight_diff = particle
                        .variant_type
                        .weight
                        .wrapping_sub(nbr.variant_type.weight);
                    let max_velocity = 5;
                    let velocity = weight_diff.min(max_velocity);

                    for _ in 0..velocity {
                        if api.get(0, 1).get_variant() == Variant::Empty {
                            api.set(0, 1, particle);
                            api.set(0, 0, EMPTY_CELL);
                            break;
                        } else if api.get(dx, 1).get_variant() == Variant::Empty {
                            api.set(dx, 1, particle);
                            api.set(0, 0, EMPTY_CELL);
                            break;
                        } else if api.get(0, 1).variant_type.variant_property
                            == VariantProperty::Liquid
                        {
                            api.set(0, 0, nbr);
                            api.set(0, 1, particle);
                            break;
                        } else {
                            api.set(0, 0, particle);
                            break;
                        }
                    }
                }
            }

            VariantProperty::Liquid => {
                let mut dx = api.rand_dir();
                let below = api.get(0, 1);
                let dx1 = api.get(dx, 1);
                if particle.variant_type.weight > below.variant_type.weight {
                    //swap
                    api.swap(0, 1);
                }

                // let mut dx0 = api.get(dx, 0);
                //fall down
                if below.get_variant() == Variant::Empty {
                    api.set(0, 0, below);
                    let mut ra = particle.ra;
                    if api.once_in(20) {
                        //randomize direction when falling sometimes
                        ra = 100 + api.rand_int(50) as u8;
                    }
                    api.set(0, 1, Particle { ra, ..particle });

                    return true;
                } else if dx1.get_variant() == Variant::Empty {
                    //fall diagonally
                    api.set(0, 0, dx1);
                    api.set(dx, 1, particle);
                    return true;
                } else if api.get(-dx, 1).get_variant() == Variant::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(-dx, 1, particle);
                    return true;
                }
                let left = particle.ra % 2 == 0;
                dx = if left { 1 } else { -1 };
                let dx0 = api.get(dx, 0);
                let dxd = api.get(dx * 2, 0);

                if dx0.get_variant() == Variant::Empty && dxd.get_variant() == Variant::Empty {
                    // scoot double
                    api.set(0, 0, dxd);
                    api.set(2 * dx, 0, Particle { rb: 6, ..particle });
                    let (dx, dy) = api.rand_vec(); //rand_vec_8
                    let nbr = api.get(dx, dy);

                    // spread opinion
                    if nbr.get_variant() == Variant::Water && nbr.ra % 2 != particle.ra % 2 {
                        api.set(
                            dx,
                            dy,
                            Particle {
                                ra: particle.ra,
                                ..particle
                            },
                        )
                    }
                } else if dx0.get_variant() == Variant::Empty {
                    api.set(0, 0, dx0);
                    api.set(dx, 0, Particle { rb: 3, ..particle });
                    let (dx, dy) = api.rand_vec(); //rand_vec_8
                    let nbr = api.get(dx, dy);
                    if nbr.get_variant() == Variant::Water {
                        if nbr.ra % 2 != particle.ra % 2 {
                            api.set(
                                dx,
                                dy,
                                Particle {
                                    ra: particle.ra,
                                    ..particle
                                },
                            )
                        }
                    }
                } else if particle.rb == 0 {
                    if api.get(-dx, 0).get_variant() == Variant::Empty {
                        // bump
                        api.set(
                            0,
                            0,
                            Particle {
                                ra: ((particle.ra as i32) + dx) as u8,
                                ..particle
                            },
                        );
                    }
                } else {
                    // become less certain (more bumpable)
                    api.set(
                        0,
                        0,
                        Particle {
                            rb: particle.rb - 1,
                            ..particle
                        },
                    );
                }

                // weight distribution
                let dx = api.rand_dir();
                let nbr = api.get(dx, 1);
                let weight = particle.variant_type.weight;
                let nbr_weight = nbr.variant_type.weight;

                if nbr_weight < weight {
                    // swap
                    api.swap(0, 1);
                }
            }

            VariantProperty::Gas => {
                // basic sand behavior but upwards
                let dx = api.rand_dir();
                let nbr = api.get(dx, -1);

                if api.get(dx, -1).get_variant() == Variant::Empty {
                    api.set(dx, -1, particle);
                    api.set(0, 0, EMPTY_CELL);
                } else {
                    api.set(0, 0, particle);
                }
            }
            // spread opinion
            _ => (),
        }
        let state = &particle.update(api);
        particle.modified = *state;
        *state
    }

    fn paint_variants(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_idx(x, y);
                let particle = self.get_particle(x, y);
                let _variant = particle.get_variant();
                let color = particle::particle_to_color(particle).to_rgba8();
                self.particles[idx].ra = color.0;
                self.particles[idx].rb = color.1;
                self.particles[idx].clock = self.generation;
            }
        }
    }
    pub fn new(width: i32, height: i32) -> World {
        let particles = (0..width * height).map(|_| EMPTY_CELL).collect();
        let environment: Vec<Environment> = (0..width * height)
            .map(|_| Environment {
                pressure: 0.,
                ambient_temperature: 22.,
                ambient_pressure: 0.,
            })
            .collect();
        World {
            particles,
            environment,
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
            *particle = Particle::new(EMPTY, 0, 0);
        }

        self.cleared = true;
        self.modified_indices.clear();
    }
}

impl World {
    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return 0;
        }
        (x + y * self.width) as usize
    }

    pub fn get_temperature(&self, x: i32, y: i32) -> f32 {
        let idx = self.get_idx(x, y);
        self.environment[idx].ambient_temperature
    }

    pub fn distribute_cell_temperature(&mut self, x: i32, y: i32) {
        // not working
        let mut top = self.get_particle(x, y);
        let mut bottom = self.get_particle(x, y + 1);
        let mut left = self.get_particle(x - 1, y);
        let mut right = self.get_particle(x + 1, y);

        let heat = self.get_temperature(x, y) / 4.;

        // share heat with neighbors

        top.temperature += heat;
        bottom.temperature += heat;
        left.temperature += heat;
        right.temperature += heat;
    }

    pub fn get_pressure(&self, x: i32, y: i32) -> f32 {
        let idx = self.get_idx(x, y);
        self.environment[idx].ambient_pressure
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
        self.particles[idx].temperature =
            (self.particles[idx].temperature + heat).clamp(-200., 9275.)
    }

    pub fn is_modified(&self) -> bool {
        self.modified_state
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

    pub fn get_particle_count(&self) -> usize {
        self.particles
            .iter()
            .filter(|p| p.get_variant() != Variant::Empty && p.get_variant() != Variant::Wall)
            .count()
    }

    pub fn erase_indestructible(&mut self, x: i32, y: i32) {
        // gives ability to erase immutable particles
        let idx = self.get_idx(x, y);
        let particle = self.particles[idx];
        if particle.variant_type.has_flag(variant_type::FLAG_IMMUTABLE) {
            self.particles[idx] = EMPTY_CELL;
        }
    }

    pub fn set_particle(&mut self, x: i32, y: i32, variant: Variant) {
        if x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1 {
            return;
        }

        // if the particle is already set to the same variant, don't do anything
        if self.get_particle(x, y).get_variant() == variant {
            return;
        }

        // if the particle is already a wall, don't do anything
        if self.get_particle(x, y).get_variant() == Variant::Wall {
            return;
        }

        let idx = self.get_idx(x, y);
        self.particles[idx] = Particle::new(*VariantType::from_variant(variant), 0, 0);
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
