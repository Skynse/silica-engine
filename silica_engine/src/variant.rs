use crate::{
    api::API,
    particle::Particle,
    variant_type::{self, variant_type, VariantProperty, VARIANTS},
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
    GOL = 5,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Variant::Empty => "Empty",
            Variant::Wall => "Wall",
            Variant::Sand => "Sand",
            Variant::Water => "Water",
            Variant::Fire => "Fire",
            Variant::GOL => "GOL",
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
            Variant::Sand => update_sand(particle, api),
            Variant::Water => update_water(particle, api),
            Variant::Fire => update_fire(particle, api),
            Variant::GOL => update_gol(particle, api),
        }
    }
}

pub fn update_sand(particle: Particle, mut api: API) {
    let dx = api.rand_dir();
    let nbr = api.get(0, 1);

    // check if neighbor is powder property and displace if heavier
    let nbr_type = variant_type(nbr.variant);

    if nbr.variant == Variant::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, particle);
    } else if api.get(dx, 1).variant == Variant::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, particle);
    } else if nbr_type.variant_property == VariantProperty::Liquid {
        api.set(0, 0, nbr);
        api.set(0, 1, particle);
    } else {
        api.set(0, 0, particle);
    }
}

pub fn update_gol(particle: Particle, mut api: API) {
    let gol_dead: Particle = Particle {
        variant: Variant::GOL,
        clock: 0,
        ra: 0,
        rb: 0,
    };

    let gol_alive: Particle = Particle {
        variant: Variant::GOL,
        clock: 0,
        ra: 0,
        rb: 1,
    };
    // get neighbors in all directions
    let nb = api.get(0, 1);
    let nt = api.get(0, -1);

    let nr = api.get(1, 0);
    let nl = api.get(-1, 0);

    let ntr = api.get(1, -1);
    let nbr = api.get(1, 1);

    let nbl = api.get(-1, 1);
    let ntl = api.get(-1, -1);

    let mut neighbors = 0;
    // check if neighbors are alive and neighbor is Variant::GOL
    if nb.variant == Variant::GOL && nb.rb == 1 {
        neighbors += 1;
    }
    if nt.variant == Variant::GOL && nt.rb == 1 {
        neighbors += 1;
    }
    if nr.variant == Variant::GOL && nr.rb == 1 {
        neighbors += 1;
    }
    if nl.variant == Variant::GOL && nl.rb == 1 {
        neighbors += 1;
    }
    if ntr.variant == Variant::GOL && ntr.rb == 1 {
        neighbors += 1;
    }
    if nbr.variant == Variant::GOL && nbr.rb == 1 {
        neighbors += 1;
    }
    if nbl.variant == Variant::GOL && nbl.rb == 1 {
        neighbors += 1;
    }
    if ntl.variant == Variant::GOL && ntl.rb == 1 {
        neighbors += 1;
    }
    // rb denotes if the particle is alive or dead
    // 1 for alive, 0 for dead
    if particle.rb == 1 {
        if neighbors < 2 {
            api.set(0, 0, gol_dead);
        } else if neighbors > 3 {
            api.set(0, 0, gol_dead);
        } else {
            api.set(0, 0, gol_alive);
        }
    } else {
        if neighbors == 3 {
            api.set(0, 0, gol_alive);
        } else {
            api.set(0, 0, gol_dead);
        }
    }
}

pub fn update_fire(particle: Particle, mut api: API) {
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

pub fn update_water(particle: Particle, mut api: API) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);

    // fall down
    if below.variant == Variant::Empty {
        api.set(0, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if dx1.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else {
        // try to displace
        dx = api.rand_dir();
        let dx1 = api.get(dx, 1);
        if dx1.variant == Variant::Empty {
            api.set(dx, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        }
    }
}
pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => VARIANTS[0].color,
        Variant::Wall => VARIANTS[1].color,
        Variant::Sand => VARIANTS[2].color,
        Variant::Water => VARIANTS[3].color,
        Variant::Fire => VARIANTS[4].color,
        Variant::GOL => (255, 255, 255),
    };

    res
}
