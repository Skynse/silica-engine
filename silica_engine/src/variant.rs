use std::fmt::Display;

pub use crate::{
    api::API,
    particle::Particle,
    variant_type::{variant_type, VariantProperty, VARIANTS},
};

pub static EMPTY_CELL: Particle = Particle {
    variant: Variant::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
    strength: 0,
    modified: false,
    velocity: 0,
    temperature: 0.,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Fire = 4,
    Smoke = 5,
    Salt = 6,
    SaltWater = 7,

    // CHEM
    OXGN = 8,
    HYGN = 9,
    HELM = 10,
    CARB = 11,
    NITR = 12,
    IRON = 13,

    // COMPOUNDS
    CO2 = 14, //gas
              // skip water
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.get_name(), *self as u8)
    }
}

impl Variant {
    pub fn update(&self, particle: Particle, api: API) -> bool {
        match self {
            Variant::Sand => update_sand(particle, api),
            Variant::Water => update_water(particle, api),
            Variant::Fire => update_fire(particle, api),
            Variant::Smoke => update_smoke(particle, api),
            Variant::Salt => update_salt(particle, api),
            Variant::SaltWater => update_salt_water(particle, api),
            Variant::CARB => update_carbon(particle, api),
            Variant::IRON => update_iron(particle, api),
            Variant::OXGN => update_oxygen(particle, api),
            Variant::HYGN => update_hydrogen(particle, api),
            Variant::HELM => update_helium(particle, api),
            Variant::NITR => update_nitrogen(particle, api),
            Variant::CO2 => update_co2(particle, api),

            _ => false,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Variant::Empty => "Empty",
            Variant::Wall => "Wall",
            Variant::Sand => "Sand",
            Variant::Water => "Water",
            Variant::Fire => "Fire",
            Variant::Smoke => "Smoke",
            Variant::Salt => "Salt",
            Variant::SaltWater => "SaltWater",
            Variant::CARB => "Carbon",
            Variant::IRON => "Iron",
            Variant::OXGN => "Oxygen",
            Variant::HYGN => "Hydrogen",
            Variant::HELM => "Helium",
            Variant::NITR => "Nitrogen",
            Variant::CO2 => "CO2",
        }
    }
}

fn update_sand(particle: Particle, mut api: API) -> bool {
    let dx = api.rand_dir();
    let nbr = api.get(0, 1);

    if nbr.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, 1).variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if variant_type(nbr.variant).variant_property == VariantProperty::Liquid {
        api.set(0, 0, nbr);
        api.set(0, 1, particle);
    } else {
        api.set(0, 0, particle);
    }
    false
}

fn update_salt(particle: Particle, mut api: API) -> bool {
    let dx = api.rand_dir();
    let nbr = api.get(0, 1);

    if nbr.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, 1).variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if variant_type(nbr.variant).variant_property == VariantProperty::Liquid {
        api.set(0, 0, nbr);
        api.set(0, 1, particle);
    }
    false
}

fn update_salt_water(particle: Particle, mut api: API) -> bool {
    false
}

fn update_fire(particle: Particle, mut api: API) -> bool {
    if api.once_in(10) {
        api.set(
            0,
            0,
            Particle {
                variant: Variant::Smoke,
                ..particle
            },
        );
        return true;
    }

    let ra = particle.ra;

    if ra > 0 {
        api.set(
            0,
            0,
            Particle {
                ra: ra - 1,
                ..particle
            },
        );
    } else {
        api.set(0, 0, EMPTY_CELL);
    }

    // set color based on time lived
    // more yellow if new

    // tendency to rise and spread
    let dx = api.rand_dir();
    let nbr = api.get(dx, -1);

    api.world.set_temperature(api.x, api.y, 1000.);
    false
}

fn update_water(particle: Particle, mut api: API) -> bool {
    let dx = api.rand_dir();
    let mut nbr = api.get(0, 1);

    if nbr.variant == Variant::Salt {
        if nbr.dissolve_to(variant_type(particle.variant).source_variant) {
            api.set(0, 1, particle);

            if nbr.strength > 0 {
                nbr.strength -= 1;
                api.set(0, 0, nbr);
            } else {
                api.set(
                    0,
                    0,
                    Particle {
                        variant: Variant::SaltWater,
                        ..particle
                    },
                );
            }
        }
    }
    false
}

fn update_smoke(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_carbon(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_iron(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_oxygen(mut particle: Particle, mut api: API) -> bool {
    /*
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }

    */
    // check surrounding environment pressure
    // also check temperature

    // if pressure is high, temp is high, and particle is right next to hydrogen, then, combine into water

    let left = api.get(-1, 0);
    let right = api.get(1, 0);
    let up = api.get(0, -1);
    let down = api.get(0, 1);

    let mut nbrs = vec![left, right, up, down];

    nbrs.sort_by(|a, b| a.temperature.partial_cmp(&b.temperature).unwrap());

    let mut nbr = nbrs[0];

    if nbr.variant == Variant::HYGN {
        if nbr.dissolve_to(variant_type(particle.variant).source_variant) {
            api.set(0, 0, nbr);
            api.set(0, 1, particle);
        }
    }

    // if temperature high enough, burn into fire
    if api.world.get_temperature(api.x, api.y) > 100. {
        api.set(
            0,
            0,
            Particle {
                variant: Variant::Fire,
                ..particle
            },
        );
        return true;
    }

    false
}

fn update_hydrogen(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_helium(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_nitrogen(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_co2(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(Variant::Empty) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => VARIANTS[0].color,
        Variant::Wall => VARIANTS[1].color,
        _ => variant_type(variant).color,
    };

    res
}
