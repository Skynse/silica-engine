use rand::Rng;
use std::convert::TryInto;
use variant_type::{VariantProperty, FLAG_IMMUTABLE};
use world::Wind;

use crate::{
    particle::{self, Particle},
    variant::Variant,
    variant_type, world,
};

pub struct API<'a> {
    pub(crate) world: &'a mut world::World,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl<'a> API<'a> {
    pub fn set(&mut self, dx: i32, dy: i32, mut particle: particle::Particle) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set")
        }

        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx > self.world.width - 1 || ny < 0 || ny > self.world.height - 1 {
            return;
        }
        let idx = self.world.get_idx(nx, ny);

        self.world.particles[idx] = particle;
        self.world.particles[idx].clock = self.world.generation.wrapping_add(1);
    }

    pub fn update_world(&mut self) {
        self.world.tick();
    }

    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let idx1 = self.world.get_idx(x1, y1);
        let idx2 = self.world.get_idx(x2, y2);
        let mut c1 = self.world.particles[idx1];
        let mut c2 = self.world.particles[idx2];

        if variant_type::variant_type(c1.variant).flags & FLAG_IMMUTABLE
            | variant_type::variant_type(c2.variant).flags & FLAG_IMMUTABLE
            == 0
        {
            return;
        }

        c1.modified = self.world.modified_state;
        c2.modified = self.world.modified_state;
        self.world.particles[idx1] = c2;
        self.world.particles[idx2] = c1;
    }
    pub fn get_from_idx(&mut self, idx: usize) -> Particle {
        self.world.particles[idx]
    }

    pub fn get_fluid(&mut self) -> Wind {
        let idx = self.world.get_idx(self.x, self.y);
        self.world.wind[idx]
    }

    pub fn get_idx(&mut self, x: i32, y: i32) -> usize {
        self.world.get_idx(x, y)
    }

    pub fn set_fluid(&mut self, dx: i32, dy: i32) {
        let idx = self.world.get_idx(self.x, self.y);
        self.world.wind[idx].dx = dx;
        self.world.wind[idx].dy = dy;
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        let i = self.rand_int(2000);
        match i % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn reset(&mut self) {
        self.world.reset();
    }

    pub fn once_in(&mut self, n: i32) -> bool {
        let i = self.rand_int(n);
        i == 0
    }

    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        rand::thread_rng().gen_range(0..n)
    }

    pub fn get(&mut self, dx: i32, dy: i32) -> Particle {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx > self.world.width - 1 || ny < 0 || ny > self.world.height - 1 {
            return Particle {
                variant: Variant::Wall,
                ra: 0,
                rb: 0,
                clock: self.world.generation,
                strength: 0,
                modified: false,
                velocity: 0,
            };
        }
        self.world.get_particle(nx, ny)
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    use super::*;

    #[test]
    fn test_api() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        assert_eq!(api.get(0, 0).get_variant(), Variant::Sand);
    }

    #[test]
    fn test_world_set() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        assert_eq!(api.get(0, 0).get_variant(), Variant::Sand);
    }

    #[test]
    fn test_world_reset() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        api.reset();
        assert_eq!(api.get(0, 0).get_variant(), Variant::Empty);
        assert_eq!(api.get(1, 1).get_variant(), Variant::Empty);
    }
}
