use crate::variant::Variant;

pub const VARIANT_COUNT: usize = 18;
use crate::{colors::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct VariantType {
    pub weight: u8,
    pub color: ParticleColor,
    pub strength: u8,
    pub source_variant: Variant,
    pub base_temperature: f32,
    pub variant_property: VariantProperty,
    pub flags: u8,
    pub name: &'static str,
} // flags

pub const FLAG_BURNS: u8 = 0b00000001;
pub const FLAG_EXPLOSIVE: u8 = 0b00000010;
pub const FLAG_IMMUTABLE: u8 = 0b00000100;
pub const FLAG_IGNITES: u8 = 0b00001000;
pub const FLAG_ALIVE: u8 = 0b00100000;

impl VariantType {
    pub fn has_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    pub fn from_variant(variant: Variant) -> &'static VariantType {
        match variant {
            Variant::Empty => &EMPTY,
            Variant::Wall => &WALL,
            Variant::Sand => &SAND,
            Variant::Water => &WATER,
            Variant::Fire => &FIRE,
            Variant::Smoke => &SMOKE,
            Variant::Salt => &SALT,
            Variant::SaltWater => &SALT_WATER,
            Variant::OXGN => &OXGN,
            Variant::HYGN => &HYGN,
            Variant::HELM => &HELM,
            Variant::CARB => &CARB,
            Variant::NITR => &NITR,
            Variant::IRON => &IRON,
            Variant::CO2 => &CO2,
            Variant::WTVP => &WTVP,
            Variant::GOL => &GOL,
            Variant::Glass => &GLASS,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum VariantProperty {
    Solid,
    Powder,
    Liquid,
    Gas,
}

pub fn get_color(variant: Variant) {
    match variant {
        Variant::Empty => EMPTY_COLOR,
        Variant::Wall => WALL_COLOR,
        Variant::Sand => SAND_COLOR,
        Variant::Water => WATER_COLOR,
        Variant::Fire => FIRE_COLOR,
        Variant::Smoke => SMOKE_COLOR,
        Variant::Salt => SALT_COLOR,
        Variant::SaltWater => SALT_WATER_COLOR,
        Variant::OXGN => OXYGEN_COLOR,
        Variant::HYGN => HYDROGEN_COLOR,
        Variant::HELM => HELIUM_COLOR,
        Variant::CARB => CARBON_COLOR,
        Variant::NITR => NITROGEN_COLOR,
        Variant::IRON => IRON_COLOR,
        Variant::CO2 => CO2_COLOR,
        Variant::WTVP => STEAM_COLOR,
        Variant::Glass => GLASS_COLOR,
        Variant::GOL => ParticleColor::from_rgba((0, 0, 0, 0)),
    };
}

pub const EMPTY: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: EMPTY_COLOR,
    source_variant: Variant::Empty,
    flags: 0,
    variant_property: VariantProperty::Solid,
    base_temperature: 22.,
    name: "Empty",
};

pub const WALL: VariantType = VariantType {
    weight: 255,
    strength: 0,
    color: WALL_COLOR,
    source_variant: Variant::Wall,
    flags: FLAG_IMMUTABLE,
    variant_property: VariantProperty::Solid,
    base_temperature: 22.,
    name: "Wall",
};

pub const SAND: VariantType = VariantType {
    weight: 64,
    strength: 0,
    color: SAND_COLOR,
    source_variant: Variant::Sand,
    variant_property: VariantProperty::Powder,
    flags: 0,
    base_temperature: 22.,
    name: "Sand",
};

pub const WATER: VariantType = VariantType {
    weight: 32,
    strength: 0,
    color: WATER_COLOR,
    source_variant: Variant::Water,
    variant_property: VariantProperty::Liquid,
    flags: 0,
    base_temperature: 22.,
    name: "Water",
};

pub const FIRE: VariantType = VariantType {
    weight: 64,
    strength: 16,
    color: FIRE_COLOR,
    source_variant: Variant::Fire,
    variant_property: VariantProperty::Gas,
    flags: FLAG_BURNS,
    base_temperature: 422.,
    name: "Fire",
};

pub const SMOKE: VariantType = VariantType {
    weight: 1,
    strength: 64,
    color: SMOKE_COLOR,
    source_variant: Variant::Smoke,
    variant_property: VariantProperty::Gas,
    flags: 0,
    base_temperature: 22.,
    name: "Smoke",
};

pub const SALT: VariantType = VariantType {
    weight: 1,
    strength: 0,
    color: SALT_COLOR,
    source_variant: Variant::Salt,
    variant_property: VariantProperty::Powder,
    flags: 0,
    base_temperature: 22.,
    name: "Salt",
};

pub const SALT_WATER: VariantType = VariantType {
    weight: 38,
    strength: 0,
    color: SALT_WATER_COLOR,
    source_variant: Variant::SaltWater,
    variant_property: VariantProperty::Liquid,
    flags: 0,
    base_temperature: 22.,
    name: "SaltWater",
};

pub const OXGN: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: OXYGEN_COLOR,
    source_variant: Variant::OXGN,
    variant_property: VariantProperty::Gas,
    flags: FLAG_BURNS | FLAG_IGNITES,
    base_temperature: 22.,
    name: "OXGN",
};

pub const HYGN: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: HYDROGEN_COLOR,
    source_variant: Variant::HYGN,
    variant_property: VariantProperty::Gas,
    flags: FLAG_BURNS | FLAG_IGNITES,
    base_temperature: 22.,
    name: "HYGN",
};

pub const HELM: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: HELIUM_COLOR,
    source_variant: Variant::HELM,
    variant_property: VariantProperty::Gas,
    flags: 0,
    base_temperature: 22.,
    name: "HELM",
};

pub const CARB: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: CARBON_COLOR,
    source_variant: Variant::CARB,
    variant_property: VariantProperty::Powder,
    flags: 0,
    base_temperature: 22.,
    name: "CARB",
};

pub const NITR: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: NITROGEN_COLOR,
    source_variant: Variant::NITR,
    variant_property: VariantProperty::Gas,
    flags: 0,
    base_temperature: 22.,
    name: "NITR",
};

pub const IRON: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: IRON_COLOR,
    source_variant: Variant::IRON,
    variant_property: VariantProperty::Solid,
    flags: 0,
    base_temperature: 22.,
    name: "IRON",
};

pub const CO2: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: CO2_COLOR,
    source_variant: Variant::CO2,
    variant_property: VariantProperty::Gas,
    flags: 0,
    base_temperature: 22.,
    name: "CO2",
};

pub const WTVP: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: STEAM_COLOR,
    source_variant: Variant::WTVP,
    variant_property: VariantProperty::Gas,
    flags: 0,
    base_temperature: 22.,
    name: "WTVP",
};

pub const GOL: VariantType = VariantType {
    weight: 0,
    strength: 16,
    color: ParticleColor {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    },
    source_variant: Variant::GOL,
    variant_property: VariantProperty::Solid,
    flags: 0,
    base_temperature: 22.,
    name: "GOL",
};

pub const GLASS: VariantType = VariantType {
    weight: 0,
    strength: 0,
    color: GLASS_COLOR,
    source_variant: Variant::Glass,
    variant_property: VariantProperty::Solid,
    flags: 0,
    base_temperature: 22.,
    name: "GLASS",
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParticleColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ParticleColor {
    pub fn from_rgba(color: (u8, u8, u8, u8)) -> ParticleColor {
        ParticleColor {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl HSV {
    pub fn hsv_to_rgb(&self) -> ParticleColor {
        let c = self.v * self.s;
        let x = c * (1. - ((self.h / 60.) % 2. - 1.).abs());
        let m = self.v - c;

        let (r, g, b) = if self.h < 60. {
            (c, x, 0.)
        } else if self.h < 120. {
            (x, c, 0.)
        } else if self.h < 180. {
            (0., c, x)
        } else if self.h < 240. {
            (0., x, c)
        } else if self.h < 300. {
            (x, 0., c)
        } else {
            (c, 0., x)
        };

        ParticleColor {
            r: ((r + m) * 255.) as u8,
            g: ((g + m) * 255.) as u8,
            b: ((b + m) * 255.) as u8,
            a: 255,
        }
    }
}

struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl HSL {
    pub fn hsl_to_rgb(&self) -> ParticleColor {
        let c = (1. - (2. * self.l - 1.).abs()) * self.s;
        let x = c * (1. - ((self.h / 60.) % 2. - 1.).abs());
        let m = self.l - c / 2.;

        let (r, g, b) = if self.h < 60. {
            (c, x, 0.)
        } else if self.h < 120. {
            (x, c, 0.)
        } else if self.h < 180. {
            (0., c, x)
        } else if self.h < 240. {
            (0., x, c)
        } else if self.h < 300. {
            (x, 0., c)
        } else {
            (c, 0., x)
        };

        ParticleColor {
            r: ((r + m) * 255.) as u8,
            g: ((g + m) * 255.) as u8,
            b: ((b + m) * 255.) as u8,
            a: 255,
        }
    }
}

impl ParticleColor {
    pub fn to_u32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    pub fn blend(&self, other: &ParticleColor) -> ParticleColor {
        let mut res = ParticleColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };

        let alpha = other.a as f32 / 255.;
        res.r = (self.r as f32 * alpha + other.r as f32 * (1. - alpha)) as u8;
        res.g = (self.g as f32 * alpha + other.g as f32 * (1. - alpha)) as u8;
        res.b = (self.b as f32 * alpha + other.b as f32 * (1. - alpha)) as u8;
        res.a = 255;

        res
    }

    pub fn inverse(&self) -> ParticleColor {
        ParticleColor {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
            a: 255,
        }
    }

    pub fn fire_color(&self) -> ParticleColor {
        let mut res = ParticleColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };

        let alpha = self.a as f32 / 255.;
        res.r = (self.r as f32 * alpha + 255. * (1. - alpha)) as u8;
        res.g = (self.g as f32 * alpha + 255. * (1. - alpha)) as u8;
        res.b = (self.b as f32 * alpha + 0. * (1. - alpha)) as u8;
        res.a = 255;

        res
    }

    pub fn darken_by_strength(&mut self, strength: u8) {
        let factor = strength as f32 / 255.;
        self.r = (self.r as f32 * (1. - factor)) as u8;
        self.g = (self.g as f32 * (1. - factor)) as u8;
        self.b = (self.b as f32 * (1. - factor)) as u8;
    }

    // whiten based on given temperature value
    pub fn whiten(&mut self, temperature: f32) {
        if temperature < 20. {
            return;
        }

        // temperature must be above 900 C for this to happen gradually from original color to white
        // dont just set to 255, gradually increase above 900C thesh
        let mut white_factor = (temperature - 20.) / 1000.;
        if white_factor > 1. {
            white_factor = 1.;
        }

        self.r = (self.r as f32 * (1. - white_factor) + 255. * white_factor) as u8;
        self.g = (self.g as f32 * (1. - white_factor) + 255. * white_factor) as u8;
        self.b = (self.b as f32 * (1. - white_factor) + 255. * white_factor) as u8;
    }

    pub fn melting(&mut self, temperature: f32) -> ParticleColor {
        // orangish color
        let mut res = ParticleColor {
            r: 255,
            g: 128,
            b: 0,
            a: 255,
        };

        // if temperature is below melting point, dont change color
        if temperature < 1000. {
            return *self;
        }

        // temperature must be above 900 C for this to happen gradually from original color to white
        // dont just set to 255, gradually increase above 900C thesh
        let mut melt_factor = (temperature - 1000.) / 1000.;
        if melt_factor > 1. {
            melt_factor = 1.;
        }

        res.r = (self.r as f32 * (1. - melt_factor) + 255. * melt_factor) as u8;
        res.g = (self.g as f32 * (1. - melt_factor) + 128. * melt_factor) as u8;
        res.b = (self.b as f32 * (1. - melt_factor) + 0. * melt_factor) as u8;

        res
    }

    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn to_rgb8(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn rgb_to_hsv(&self) -> HSV {
        let r = self.r as f32 / 255.;
        let g = self.g as f32 / 255.;
        let b = self.b as f32 / 255.;

        let c_max = r.max(g).max(b);
        let c_min = r.min(g).min(b);
        let delta = c_max - c_min;

        let h = if delta == 0. {
            0.
        } else if c_max == r {
            60. * (((g - b) / delta) % 6.)
        } else if c_max == g {
            60. * ((b - r) / delta + 2.)
        } else {
            60. * ((r - g) / delta + 4.)
        };

        let s = if c_max == 0. { 0. } else { delta / c_max };

        HSV { h, s, v: c_max }
    }

    pub fn hue_distance(&self, other: &ParticleColor) -> f32 {
        let hsv1 = self.rgb_to_hsv();
        let hsv2 = other.rgb_to_hsv();

        let mut dist = (hsv1.h - hsv2.h).abs();
        if dist > 180. {
            dist = 360. - dist;
        }
        dist
    }

    pub fn brightness(&self) -> f32 {
        // return ((f32)c.r * 0.299f + (f32)c.g * 0.587f + (f32)c.b *0.114f) / 256.f;
        return (self.r as f32 * 0.299 + self.g as f32 * 0.587 + self.b as f32 * 0.114) / 256.;
    }

    pub fn color_num(&self) -> f32 {
        /*
            const f32 bright_factor = 100.0f;
        const f32 sat_factor = 0.1f;
        hsv_t hsv = rgb_to_hsv(c);
        return hsv.s * sat_factor + brightness(c) * bright_factor; */

        let bright_factor = 100.;
        let sat_factor = 0.1;
        let hsv = self.rgb_to_hsv();
        hsv.s * sat_factor + self.brightness() * bright_factor
    }

    pub fn vary_color(&mut self, amount: i32) -> ParticleColor {
        // slightly adjust lightness and saturation
        let mut hsv = self.rgb_to_hsv();
        hsv.s += amount as f32 / 200.;
        hsv.v += amount as f32 / 200.;
        hsv.h = hsv.h;

        hsv.hsv_to_rgb()
    }
}
