use std::fmt::Display;

pub use crate::{api::API, particle::Particle, variant_type::VariantProperty};
use crate::{particle::Velocity, variant_type::*};
use serde::{Deserialize, Serialize};

pub static EMPTY_CELL: Particle = Particle {
    variant_type: EMPTY,
    clock: 0,
    strength: 0,
    modified: false,
    velocity: Velocity { x: 0., y: 0. },
    temperature: 0.,
    ra: 0,
    rb: 0,
};

pub static EMPTY_LL: Particle = Particle {
    variant_type: EMPTY_L,
    clock: 0,
    strength: 0,
    modified: false,
    velocity: Velocity { x: 0., y: 0. },
    temperature: 0.,
    ra: 0,
    rb: 0,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Glass = 17,
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

    //GASES
    WTVP = 15, //water vapor

    //LIFE
    GOL = 16, //game of life
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.get_name(), *self as u8)
    }
}

impl Variant {
    pub fn from_u8(n: u8) -> Variant {
        match n {
            0 => Variant::Empty,
            1 => Variant::Wall,
            2 => Variant::Sand,
            3 => Variant::Water,
            4 => Variant::Fire,
            5 => Variant::Smoke,
            6 => Variant::Salt,
            7 => Variant::SaltWater,
            8 => Variant::OXGN,
            9 => Variant::HYGN,
            10 => Variant::HELM,
            11 => Variant::CARB,
            12 => Variant::NITR,
            13 => Variant::IRON,
            14 => Variant::CO2,
            15 => Variant::WTVP,
            16 => Variant::GOL,
            17 => Variant::Glass,
            _ => Variant::Empty,
        }
    }
    pub fn is_empty(&self) -> bool {
        *self == Variant::Empty
    }
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
            Variant::WTVP => update_wtvp(particle, api),
            Variant::GOL => update_gol(particle, api),
            Variant::Empty => update_empty(particle, api),
            Variant::Glass => update_glass(particle, api),

            _ => false,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Variant::Empty => "EMPT",
            Variant::Wall => "WALL",
            Variant::Sand => "SAND",
            Variant::Water => "WATR",
            Variant::Fire => "FIRE",
            Variant::Smoke => "SMOK",
            Variant::Salt => "SALT",
            Variant::SaltWater => "SWTR",
            Variant::CARB => "CRBN",
            Variant::IRON => "IRON",
            Variant::OXGN => "OXYG",
            Variant::HYGN => "HYDR",
            Variant::HELM => "HELM",
            Variant::NITR => "NITR",
            Variant::CO2 => "CO2",
            Variant::WTVP => "STM",
            Variant::GOL => "GOL",
            Variant::Glass => "GLAS",
        }
    }
}

fn update_empty(_particle: Particle, mut api: API) -> bool {
    let mut alive_nbrs = 0;
    if api.get(0, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(0, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, 0).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, 0).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if alive_nbrs == 3 {
        api.set(0, 0, Particle::new(GOL, 0, 0));
        return true;
    }
    false
}

fn update_sand(particle: Particle, mut api: API) -> bool {
    if particle.temperature > 1700. {
        api.set(
            0,
            0,
            Particle {
                variant_type: GLASS,
                ..particle
            },
        );
        return true;
    }
    false
}

fn update_glass(_particle: Particle, _api: API) -> bool {
    true
}

fn update_salt(particle: Particle, mut api: API) -> bool {
    // swap down with water if water above
    let top = api.get(0, -1);
    let bottom = api.get(0, 1);

    if top.get_variant() == Variant::Water && bottom.get_variant() == Variant::Empty {
        api.set(
            0,
            -1,
            Particle {
                variant_type: EMPTY,
                ..top
            },
        );
        api.set(
            0,
            1,
            Particle {
                variant_type: SALT_WATER,
                ..particle
            },
        );
        return true;
    }

    false
}

fn update_salt_water(_particle: Particle, mut _api: API) -> bool {
    // swap down with water if water above\
    // if temp > 102, then turn into one part steam, one part salt
    let top = _api.get(0, -1);
    let bottom = _api.get(0, 1);

    if _particle.temperature > 102. {
        // turn into one part steam, one part salt
        // check to see if there's room for steam AND salt, then split
        // otherwise, just turn into salt
        if top.get_variant() == Variant::Empty && bottom.get_variant() == Variant::Empty {
            _api.set(
                0,
                -1,
                Particle {
                    variant_type: WTVP,
                    .._particle
                },
            );
            _api.set(
                0,
                1,
                Particle {
                    variant_type: SALT,
                    .._particle
                },
            );
        } else {
            _api.set(
                0,
                0,
                Particle {
                    variant_type: SALT,
                    .._particle
                },
            );
        }

        return true;
    }
    false
}

fn update_fire(mut particle: Particle, mut api: API) -> bool {
    if api.once_per(50) && particle.dissolve_to(EMPTY) {
        api.set(
            0,
            0,
            Particle {
                variant_type: SMOKE,
                ..particle
            },
        );
        return true;
    }

    api.world.set_temperature(api.x, api.y, 800.);
    false
}

fn update_water(particle: Particle, mut api: API) -> bool {
    let _dx = api.rand_dir();
    let _nbr = api.get(0, 1);

    if particle.temperature > 100. {
        api.set(
            0,
            0,
            Particle {
                variant_type: WTVP,
                ..particle
            },
        );
        return true;
    }

    false
}

fn update_smoke(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return true;
    }

    false
}

fn update_carbon(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_iron(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_oxygen(particle: Particle, mut api: API) -> bool {
    /*
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
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

    if nbr.get_variant() == Variant::HYGN && nbr.dissolve_to(WATER) {
        {
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
                variant_type: FIRE,
                ..particle
            },
        );
        return true;
    }

    false
}

fn update_wtvp(particle: Particle, mut api: API) -> bool {
    if particle.temperature < 100. && particle.temperature > 0. {
        api.set(
            0,
            0,
            Particle {
                variant_type: WATER,
                ..particle
            },
        );
        return true;
    }

    false
}

fn update_gol(particle: Particle, mut api: API) -> bool {
    let mut alive_nbrs: u32 = 0;

    if api.get(0, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(0, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, 0).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, 0).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(1, -1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if api.get(-1, 1).get_variant() == Variant::GOL {
        alive_nbrs += 1;
    }

    if alive_nbrs < 2 || alive_nbrs > 3 {
        api.set(0, 0, EMPTY_LL);
    }

    // get self temperature and die
    if particle.temperature > 100. {
        api.set(
            0,
            0,
            Particle {
                variant_type: SAND,
                ..particle
            },
        );
    }
    true
}

fn update_hydrogen(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_helium(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_nitrogen(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

fn update_co2(mut particle: Particle, mut api: API) -> bool {
    if api.once_in(10) && particle.dissolve_to(EMPTY) {
        api.set(0, 0, EMPTY_CELL);
        return false;
    }
    false
}

pub fn particle_to_color(variant_type: VariantType) -> (u8, u8, u8, u8) {
    variant_type.color.to_rgba8()
}
