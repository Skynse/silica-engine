use rand::Rng;
use std::convert::TryInto;

fn example_function() {
    let my_usize: usize = 42;
    let my_i32: i32 = my_usize.try_into().unwrap();
    println!("my_i32: {}", my_i32);
}

use crate::{
    particle::{self, Particle},
    variant::Variant,
    world,
};

pub struct API<'a> {
    pub(crate) world: &'a mut world::World,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl<'a> API<'a> {
    pub fn set(&mut self, dx: i32, dy: i32, particle: particle::Particle) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0
            || nx >= (self.world.width - 1).try_into().unwrap()
            || ny < 0
            || ny >= (self.world.height - 1).try_into().unwrap()
        {
            return;
        }

        let idx = self
            .world
            .get_idx(nx.try_into().unwrap(), ny.try_into().unwrap());
        self.world.particles[idx] = particle;

        // make sure particle does not go out of bounds
        if self.x < 0 {
            self.x = 0;
        }
    }

    pub fn update_world(&mut self) {
        self.world.tick();
    }

    pub fn reset(&mut self) {
        self.world.reset();
    }

    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..n);
        x
    }

    pub fn swap(&mut self, dx: i32, dy: i32) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0
            || nx >= (self.world.width - 1).try_into().unwrap()
            || ny < 0
            || ny >= (self.world.height - 1).try_into().unwrap()
        {
            return;
        }

        let idx = self
            .world
            .get_idx(self.x.try_into().unwrap(), self.y.try_into().unwrap());
        let nidx = self
            .world
            .get_idx(nx.try_into().unwrap(), ny.try_into().unwrap());

        let tmp = self.world.particles[idx];
        self.world.particles[idx] = self.world.particles[nidx];
        self.world.particles[nidx] = tmp;
    }

    pub fn get(&mut self, dx: i32, dy: i32) -> Particle {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0
            || nx >= (self.world.width - 1).try_into().unwrap()
            || ny < 0
            || ny >= (self.world.height - 1).try_into().unwrap()
        {
            return Particle::new(Variant::Empty, 0, 0);
        }
        self.world
            .get(nx.try_into().unwrap(), ny.try_into().unwrap())
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
