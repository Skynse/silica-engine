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
    fn update(&self, particle: Particle, api: API);
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
    fn update(&self, particle: Particle, mut api: API) {
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

        //handle wall collision
        let nl = api.get(-1, 0);
        let nr = api.get(1, 0);
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
    pub fn update(&self, particle: Particle, api: API) {
        // pass
        match self {
            Variant::Empty => (),
            Variant::Wall => (),
            Variant::Sand => Sand.update(particle, api),
            Variant::Water => Water.update(particle, api),
            Variant::Fire => Fire.update(particle, api),
            Variant::Smoke => Smoke.update(particle, api),
            Variant::Salt => Salt.update(particle, api),
            Variant::SaltWater => SaltWater.update(particle, api),
        }
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

    fn update(&self, mut particle: Particle, mut api: API) {
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

    fn update(&self, particle: Particle, mut api: API) {
        // fall just like salt or sand but move sideways if the bottom is a wall or empty
        let dx = api.rand_dir();
        let nbr = api.get(0, 1);

        if nbr.variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        }
        if variant_type(nbr.variant).variant_property == VariantProperty::Liquid {
            api.set(0, 0, nbr);
            api.set(0, 1, particle);
        } else if variant_type(api.get(dx, 0).variant).variant_property == VariantProperty::Solid {
            api.set(dx, 0, particle);
            api.set(0, 0, EMPTY_CELL);
        } else {
            api.set(0, 0, particle);
        }
    }
}

impl VariantTrait for Fire {
    fn get_variant(&self) -> Variant {
        Variant::Fire
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0xFF, 0x00, 0x00)
    }

    fn update(&self, particle: Particle, mut api: API) {
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
            return;
        }
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

    fn update(&self, particle: Particle, mut api: API) {
        // fall just like salt or sand but move sideways if the bottom is a wall or empty
        let dx = api.rand_dir();
        let nbr = api.get(0, 1);

        if nbr.variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if api.get(dx, 1).variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if variant_type(nbr.variant).variant_property == VariantProperty::Liquid {
            // displace if heavier
            let other_weight = variant_type(nbr.variant).weight;
            let self_weight = variant_type(particle.variant).weight;

            if self_weight > other_weight {
                api.set(0, 0, nbr);
                api.set(0, 1, particle);
            } else {
                api.set(0, 0, particle);
            }
        } else if variant_type(api.get(dx, 0).variant).variant_property == VariantProperty::Solid {
            if api.get(dx, 0).variant == Variant::Salt {
                api.set(
                    dx,
                    0,
                    Particle {
                        variant: Variant::SaltWater,
                        ..particle
                    },
                );
                api.set(0, 0, EMPTY_CELL);
            } else {
                api.set(dx, 0, particle);
                api.set(0, 0, EMPTY_CELL);
            }
        } else {
            api.set(0, 0, particle);
        }
    }
}

impl VariantTrait for Smoke {
    fn get_variant(&self) -> Variant {
        Variant::Fire
    }

    fn get_color(&self) -> (u8, u8, u8) {
        (0x00, 0x00, 0x00)
    }
    fn update(&self, mut particle: Particle, mut api: API) {
        // eventually gas dissolves into empty
        if api.once_in(100) && particle.dissolve_to(Variant::Empty) {
            api.set(0, 0, EMPTY_CELL);
            return;
        }

        // gas moves up
        let above = api.get(0, -1);

        if above.variant == Variant::Empty {
            api.set(0, -1, particle);
            api.set(0, 0, EMPTY_CELL);
            return;
        }

        // spread gas horizontally
        let rand_dir = api.rand_vec();

        let next = api.get(rand_dir.0, rand_dir.1);

        if next.variant == Variant::Empty {
            api.set(rand_dir.0, rand_dir.1, particle);
            api.set(0, 0, EMPTY_CELL);
            return;
        }
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
