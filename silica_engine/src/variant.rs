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
            Variant::Empty => "Empty",
            Variant::Wall => "Wall",
            Variant::Sand => "Sand",
            Variant::Water => "Water",
            Variant::Fire => "Fire",
            Variant::Smoke => "Smoke",
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
        let ra = particle.ra;
        let mut decayed = particle.clone();
        decayed.ra = ra - (2 + api.rand_dir()) as u8;

        let (dx, dy) = api.rand_vec();
        if ra < 5 || api.get(dx, dy).variant == Variant::Water {
            api.set(0, 0, EMPTY_CELL);
        } else if api.get(dx, dy).variant == Variant::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, decayed);
        }
        api.set(0, 0, decayed);
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
        let mut dx = api.rand_dir();
        let below = api.get(0, 1);
        let dx1 = api.get(dx, 1);

        if below.variant == Variant::Empty {
            api.set(0, 0, below);
            let mut ra = particle.ra;
            if api.once_in(20) {
                ra = 100 + api.rand_int(50) as u8;
            }
            api.set(0, 1, Particle { ra: ra, ..particle });
            return;
        } else if dx1.variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
            return;
        } else if api.get(-dx, 1).variant == Variant::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(-dx, 1, particle);
            return;
        }

        let left = particle.ra % 2 == 0;
        dx = if left { -1 } else { 1 };
        let dx0 = api.get(dx, 0);
        let dxd = api.get(dx * 2, 0);

        if dx0.variant == Variant::Empty || dxd.variant == Variant::Empty {
            api.set(0, 0, dxd);
            api.set(2 * dx, 0, Particle { rb: 6, ..particle });
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

    fn update(&self, particle: Particle, mut api: API) {
        let dx = api.rand_dir();
        let nb = api.get(dx, 1);
        let nbr = api.get(dx + 1, 1);
        let nbl = api.get(dx - 1, 1);
        let nl = api.get(-1, 0);
        let nr = api.get(1, 0);

        // check if neighbor is powder property and displace if heavier
        let nb_type = variant_type(nb.variant);

        if nb_type.variant_property == VariantProperty::Powder {
            if nb_type.weight > VARIANTS[particle.variant as usize].weight {
                api.set(0, 1, particle);
                api.set(0, 0, nb);
            }
        }

        if nb.variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if nbr.variant == Variant::Empty {
            api.set(dx + 1, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if nbl.variant == Variant::Empty {
            api.set(dx - 1, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if nl.variant == Variant::Empty {
            api.set(-1, 0, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if nr.variant == Variant::Empty {
            api.set(1, 0, particle);
            api.set(0, 0, EMPTY_CELL);
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
