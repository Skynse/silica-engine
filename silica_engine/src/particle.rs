use crate::{
    api::API,
    prelude::{variant_type, ParticleColor, FLAG_IMMUTABLE},
    variant::Variant,
    variant_type::VARIANTS,
};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub variant: Variant,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
    pub strength: u8,
    pub modified: bool,
    pub velocity: Velocity,
    pub temperature: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
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
            velocity: Velocity { x: 0., y: 0. },
            temperature: 0.,
        }
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn add_heat(&mut self, heat: f32) {
        self.temperature += heat;
        // clamp to max_temp
        if self.temperature > crate::MAX_TEMP {
            self.temperature = crate::MAX_TEMP;
        }
    }

    pub fn update(&mut self, mut api: API) -> bool {
        self.variant.update(*self, api)
    }
}

pub fn particle_to_color(particle: Particle) -> ParticleColor {
    let res = match particle.variant {
        Variant::Empty => VARIANTS[0].color,
        Variant::Wall => VARIANTS[1].color,
        Variant::Sand => {
            // vary color based on ra
            let mut color = VARIANTS[2].color;
            color.whiten(particle.temperature);

            color.vary_color(particle.ra as i32)
        }
        Variant::Water => {
            // vary color based on ra
            let mut color = VARIANTS[3].color;

            color.vary_color(particle.ra as i32)
        }
        Variant::Fire => VARIANTS[4].color,
        Variant::Smoke => VARIANTS[5].color,
        Variant::Salt => VARIANTS[6].color,
        Variant::SaltWater => VARIANTS[7].color,
        Variant::OXGN => VARIANTS[8].color,
        Variant::HYGN => VARIANTS[9].color,
        Variant::HELM => VARIANTS[10].color,
        Variant::CARB => VARIANTS[11].color,
        Variant::NITR => VARIANTS[12].color,
        Variant::IRON => VARIANTS[13].color,
        Variant::CO2 => VARIANTS[14].color,
        Variant::WTVP => VARIANTS[15].color,
        Variant::GOL => VARIANTS[16].color,
    };

    res
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
