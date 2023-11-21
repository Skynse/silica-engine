use crate::{api::API, prelude::ParticleColor, variant::Variant, variant_type};
use rand::Rng;
use variant_type::VariantType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    pub variant_type: VariantType,
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
impl Default for Particle {
    fn default() -> Self {
        Self {
            variant_type: variant_type::EMPTY,
            ra: 0,
            rb: 0,
            clock: 0,
            strength: 0,
            modified: false,
            velocity: Velocity { x: 0., y: 0. },
            temperature: 0.,
        }
    }
}
impl Particle {
    pub fn new(variant_type: VariantType, _ra: u8, rb: u8) -> Particle {
        Particle {
            variant_type: variant_type,
            ra: 100 + rand::thread_rng().gen_range(0..=1) * 50,
            rb,
            clock: 0,
            strength: 0,
            modified: false,
            velocity: Velocity { x: 0., y: 0. },
            temperature: 0.,
        }
    }

    // save method that only takes the color of the particle

    pub fn save(&self) -> (u8, u8, u8) {
        let color = particle_to_color(*self);
        (color.r, color.g, color.b)
    }

    pub fn get_variant(&self) -> Variant {
        self.variant_type.source_variant
    }

    pub fn add_heat(&mut self, heat: f32) {
        self.temperature += heat;
        // clamp to max_temp
        if self.temperature > crate::MAX_TEMP {
            self.temperature = crate::MAX_TEMP;
        }
    }

    pub fn update(&mut self, api: API) -> bool {
        self.variant_type.source_variant.update(*self, api)
    }
}

pub fn particle_to_color(particle: Particle) -> ParticleColor {
    let res = match particle.get_variant() {
        Variant::Empty => variant_type::EMPTY.color,
        Variant::Wall => variant_type::WALL.color,
        Variant::Sand => {
            // vary color based on ra
            let mut color = variant_type::SAND.color;
            color.whiten(particle.temperature);

            color.vary_color(particle.ra as i32)
        }
        Variant::Water => {
            // vary color based on ra
            let mut color = variant_type::WATER.color;
            color.whiten(particle.temperature);

            color.vary_color(particle.ra as i32)
        }
        Variant::Fire => {
            // vary color based on ra
            let mut color = variant_type::FIRE.color;
            //  color.fire_color();

            color.vary_color(particle.ra as i32)
        }
        Variant::Smoke => {
            // vary color based on ra
            let mut color = variant_type::SMOKE.color;
            color.darken_by_strength(particle.strength);
            color
        }
        Variant::Salt => variant_type::SALT.color,
        Variant::SaltWater => variant_type::SALT_WATER.color,
        Variant::OXGN => variant_type::OXGN.color,
        Variant::HYGN => variant_type::HYGN.color,
        Variant::HELM => variant_type::HELM.color,
        Variant::CARB => variant_type::CARB.color,
        Variant::NITR => variant_type::NITR.color,
        Variant::IRON => {
            // vary color based on ra
            let mut color = variant_type::IRON.color;
            color.whiten(particle.temperature);

            color.vary_color(particle.ra as i32)
        }
        Variant::CO2 => variant_type::CO2.color,
        Variant::WTVP => variant_type::WTVP.color,
        Variant::GOL => variant_type::GOL.color,
        Variant::Glass => {
            // vary color based on ra
            let mut color = variant_type::GLASS.color;
            color.whiten(particle.temperature);

            color.vary_color(particle.ra as i32)
        }
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
    pub fn dissolve_to(&mut self, variant_type: VariantType) -> bool {
        if self.strength > 0 {
            self.strength -= 1;
            return false;
        } else {
            self.variant_type = variant_type;
            self.strength = variant_type.strength; // we need to grab the variants base strength later
            return true;
        }
    }
}
