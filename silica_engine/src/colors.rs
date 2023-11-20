use crate::variant_type::ParticleColor;

/*
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
Variant::WTVP => "Steam", */

pub const EMPTY_COLOR: ParticleColor = ParticleColor {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const WALL_COLOR: ParticleColor = ParticleColor {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};
pub const SAND_COLOR: ParticleColor = ParticleColor {
    //rgb(215, 172, 86)
    r: 215,
    g: 172,
    b: 86,
    a: 255,
};
pub const WATER_COLOR: ParticleColor = ParticleColor {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
pub const FIRE_COLOR: ParticleColor = ParticleColor {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
pub const SMOKE_COLOR: ParticleColor = ParticleColor {
    r: 128,
    g: 128,
    b: 128,
    a: 128,
};
pub const SALT_COLOR: ParticleColor = ParticleColor {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const SALT_WATER_COLOR: ParticleColor = ParticleColor {
    r: 50,
    g: 164,
    b: 168,
    a: 255,
};
pub const CARBON_COLOR: ParticleColor = ParticleColor {
    r: 10,
    g: 10,
    b: 20,
    a: 255,
};
pub const IRON_COLOR: ParticleColor = ParticleColor {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};
pub const OXYGEN_COLOR: ParticleColor = ParticleColor {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
pub const HYDROGEN_COLOR: ParticleColor = ParticleColor {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const HELIUM_COLOR: ParticleColor = ParticleColor {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
};
pub const NITROGEN_COLOR: ParticleColor = ParticleColor {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
pub const CO2_COLOR: ParticleColor = ParticleColor {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
};
pub const STEAM_COLOR: ParticleColor = ParticleColor {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

/*
            Variant::Empty => EMPTY_COLOR,
            Variant::Wall => WALL_COLOR,
            Variant::Sand => SAND_COLOR,
            Variant::Water => WATER_COLOR,
            Variant::Fire => FIRE_COLOR,
            Variant::Smoke => SMOKE_COLOR,
            Variant::Salt => SALT_COLOR,
            Variant::SaltWater => SALT_WATER_COLOR,
            Variant::CARB => CARBON_COLOR,
            Variant::IRON => IRON_COLOR,
            Variant::OXGN => OXYGEN_COLOR,
            Variant::HYGN => HYDROGEN_COLOR,
            Variant::HELM => HELIUM_COLOR,
            Variant::NITR => NITROGEN_COLOR,
            Variant::CO2 => CO2_COLOR,
            Variant::WTVP => STEAM_COLOR,
*/
