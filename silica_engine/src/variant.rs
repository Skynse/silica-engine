use crate::{
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
}
trait VariantTrait {
    fn get_variant(&self) -> Variant;
    fn get_color(&self) -> (u8, u8, u8);
    fn update(&self, particle: Particle, api: API) -> bool;
}
pub struct Sand;
struct Fire;
struct Smoke;

impl VariantTrait for Sand {
    fn get_variant(&self) -> Variant {
        Variant::Sand
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0xFF, 0xCC, 0x99)
    }
    fn update(&self, particle: Particle, mut api: API) -> bool {
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
}

// create a sand element using trait

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Variant::Empty => "VOID",
            Variant::Wall => "WALL",
            Variant::Sand => "SAND",
            Variant::Water => "WATR",
            Variant::Fire => "FIRE",
            Variant::Smoke => "SMOG",
            Variant::Salt => "SALT",
            Variant::SaltWater => "SWAT",
        };
        write!(f, "{}", res)
    }
}

impl Variant {
    pub fn update(&self, particle: Particle, api: API) -> bool {
        // pass
        let modified: bool = match self {
            Variant::Sand => Sand.update(particle, api),
            Variant::Water => Water.update(particle, api),
            Variant::Fire => Fire.update(particle, api),
            Variant::Smoke => Smoke.update(particle, api),
            Variant::Salt => Salt.update(particle, api),
            Variant::SaltWater => SaltWater.update(particle, api),
            _ => false,
        };
        modified
    }
}

struct Salt;
struct SaltWater;

impl VariantTrait for Salt {
    fn get_variant(&self) -> Variant {
        Variant::Salt
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0xFF, 0xFF, 0xFF)
    }

    fn update(&self, mut particle: Particle, mut api: API) -> bool {
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
}

impl VariantTrait for SaltWater {
    fn get_variant(&self) -> Variant {
        Variant::SaltWater
    }

    fn get_color(&self) -> (u8, u8, u8) {
        // lighter blue
        crate::LIGHT_BLUE
    }

    fn update(&self, particle: Particle, mut api: API) -> bool {
        false
    }
}

impl VariantTrait for Fire {
    fn get_variant(&self) -> Variant {
        Variant::Fire
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0xFF, 0x00, 0x00)
    }

    fn update(&self, particle: Particle, mut api: API) -> bool {
        // fire eventually dissolves into smoke
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
        false
    }
}

struct Water;

impl VariantTrait for Water {
    fn get_variant(&self) -> Variant {
        Variant::Water
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0x00, 0x00, 0xFF)
    }

    fn update(&self, particle: Particle, mut api: API) -> bool {
        // fall just like salt or sand but move sideways if the bottom is a wall or empty
        // check if colliding with salt
        let dx = api.rand_dir();
        let mut nbr = api.get(0, 1);

        if nbr.variant == Variant::Salt {
            if nbr.dissolve_to(variant_type(particle.variant).source_variant) {
                api.set(0, 1, particle);

                if nbr.strength > 0 {
                    nbr.strength -= 1;
                    api.set(0, 0, nbr);
                } else {
                    api.set(0, 0, EMPTY_CELL);
                }
            }
        }
        false
    }
}

impl VariantTrait for Smoke {
    fn get_variant(&self) -> Variant {
        Variant::Fire
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0x00, 0x00, 0x00)
    }
    fn update(&self, mut particle: Particle, mut api: API) -> bool {
        // eventually gas dissolves into empty
        if api.once_in(100) && particle.dissolve_to(Variant::Empty) {
            api.set(0, 0, EMPTY_CELL);
            return true;
        }
        false
    }
}
pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => VARIANTS[0].color,
        Variant::Wall => VARIANTS[1].color,
        _ => variant_type(variant).color,
    };

    res
}
