use crate::{
    api::API,
    prelude::{variant_type, FLAG_IMMUTABLE},
    variant::Variant,
    variant_type::VARIANTS,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Particle {
    pub variant: Variant,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
    pub strength: u8,
    pub modified: bool,
    pub velocity: u8,
}

impl Particle {
    pub fn new(variant: Variant, ra: u8, rb: u8) -> Particle {
        Particle {
            variant: variant,
            ra: 100 + rand::thread_rng().gen_range(0..=1) * 50 as u8,
            rb,
            clock: 0,
            strength: 0,
            modified: false,
            velocity: 0,
        }
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn update_vel(&mut self) {
        let max_speed = 8;
        let new_vel = self.velocity as f32 + 0.2;
        if (new_vel).abs() > max_speed as f32 {
            self.velocity = max_speed * new_vel.signum() as u8;
        } else {
            self.velocity = new_vel as u8;
        }
    }

    pub fn update(&mut self, mut api: API) -> bool {
        self.variant.update(*self, api)
    }
}

pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => (0, 0, 0),
        Variant::Wall => VARIANTS[1].color,
        Variant::Sand => VARIANTS[2].color,
        Variant::Water => VARIANTS[3].color,
        Variant::Fire => VARIANTS[4].color,
        Variant::Smoke => VARIANTS[5].color,
        Variant::Salt => VARIANTS[6].color,
        Variant::SaltWater => VARIANTS[7].color,
    };

    res
}
pub fn vary_color(color: (u8, u8, u8), variance: u8) -> (u8, u8, u8) {
    // grainy color not deviating too much from original color
    // DO NOT OVERFLOW SUBTRACTION
    let r = if color.0 > variance {
        color.0 - variance
    } else {
        0
    };

    (
        r.wrapping_add(rand::thread_rng().gen_range(0..variance * 2)),
        color
            .1
            .wrapping_add(rand::thread_rng().gen_range(0..variance * 2)),
        color
            .2
            .wrapping_add(rand::thread_rng().gen_range(0..variance * 2)),
    )
}

pub fn interpolate(
    color_1: &(u8, u8, u8),
    color_2: &(u8, u8, u8),
    factor: u8,
    max: u8,
) -> (u8, u8, u8) {
    let factor_f32 = factor as f32 / max as f32;
    let inv_factor_f32 = 1.0 - factor_f32;
    (
        (color_1.0 as f32 * factor_f32 + color_2.0 as f32 * inv_factor_f32) as u8,
        (color_1.1 as f32 * factor_f32 + color_2.1 as f32 * inv_factor_f32) as u8,
        (color_1.2 as f32 * factor_f32 + color_2.2 as f32 * inv_factor_f32) as u8,
    )
}

impl Particle {
    pub fn dissolve_to(&mut self, variant: Variant) -> bool {
        if self.strength > 0 {
            self.strength -= 1;
            return false;
        } else {
            self.variant = variant;
            self.strength = variant_type(variant).strength;
            return true;
        }
    }
}
