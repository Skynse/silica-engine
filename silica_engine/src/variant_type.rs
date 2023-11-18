use crate::variant::Variant;
pub const VARIANT_COUNT: usize = 15;

#[derive(PartialEq)]
pub struct VariantType {
    pub weight: u8,
    pub color: (u8, u8, u8),
    pub strength: u8,
    pub source_variant: Variant,
    pub base_temperature: f32,
    pub variant_property: VariantProperty,
    pub flags: u8,
} // flags
pub const FLAG_BURNS: u8 = 0b00000001;
pub const FLAG_EXPLOSIVE: u8 = 0b00000010;
pub const FLAG_IMMUTABLE: u8 = 0b00000100;
pub const FLAG_IGNITES: u8 = 0b00001000;

impl VariantType {
    pub fn has_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

#[derive(PartialEq, Eq)]
pub enum VariantProperty {
    Solid,
    Powder,
    Liquid,
    Gas,
}

#[inline(always)]
pub fn variant_type(variant: Variant) -> &'static VariantType {
    &VARIANTS[variant as usize]
}

pub static VARIANTS: [VariantType; VARIANT_COUNT] = [
    // 0 Empty
    VariantType {
        weight: 0,
        strength: 0,
        color: (0, 0, 0),
        source_variant: Variant::Empty,
        flags: 0,
        variant_property: VariantProperty::Solid,
        base_temperature: 22.,
    },
    // 1 Wall
    VariantType {
        weight: 0,
        strength: 0,
        color: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Wall,
        flags: FLAG_IMMUTABLE,
        variant_property: VariantProperty::Solid,
        base_temperature: 22.,
    },
    // 2 Sand
    VariantType {
        weight: 1,
        strength: 0,
        // peach brown
        color: (0xFF, 0xCC, 0x99),
        source_variant: Variant::Sand,
        variant_property: VariantProperty::Powder,
        flags: 0,
        base_temperature: 22.,
    },
    // 3 Water
    VariantType {
        weight: 32,
        strength: 0,
        color: crate::BLUE,
        source_variant: Variant::Water,
        variant_property: VariantProperty::Liquid,
        flags: 0,
        base_temperature: 22.,
    },
    // 4 Fire
    VariantType {
        weight: 64,
        strength: 16,
        color: (0xFF, 0x00, 0x00),
        source_variant: Variant::Fire,
        variant_property: VariantProperty::Gas,
        flags: FLAG_BURNS,
        base_temperature: 422.,
    },
    // 5 Smoke
    VariantType {
        weight: 1,
        strength: 32,
        color: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Smoke,
        variant_property: VariantProperty::Gas,
        flags: 0,
        base_temperature: 22.,
    },
    // 6 Salt
    VariantType {
        weight: 1,
        strength: 0,
        color: (0xFF, 0xFF, 0xFF),
        source_variant: Variant::Salt,
        variant_property: VariantProperty::Powder,
        flags: 0,
        base_temperature: 22.,
    },
    // 7 SaltWater
    VariantType {
        weight: 32,
        strength: 0,
        color: crate::LIGHT_BLUE,
        source_variant: Variant::SaltWater,
        variant_property: VariantProperty::Liquid,
        flags: 0,
        base_temperature: 22.,
    },
    // 8 OXGN
    VariantType {
        weight: 0,
        strength: 0,
        color: (0xFF, 0xFF, 0xFF),
        source_variant: Variant::OXGN,
        variant_property: VariantProperty::Gas,
        flags: FLAG_BURNS | FLAG_IGNITES,
        base_temperature: 22.,
    },
    // 9 HYGN
    VariantType {
        weight: 0,
        strength: 0,
        color: (0xFF, 0xFF, 0xFF),
        source_variant: Variant::HYGN,
        variant_property: VariantProperty::Gas,
        flags: FLAG_BURNS | FLAG_IGNITES,
        base_temperature: 22.,
    },
    // 10 HELM
    VariantType {
        weight: 0,
        strength: 0,
        color: (0xFF, 0xFF, 0xFF),
        source_variant: Variant::HELM,
        variant_property: VariantProperty::Gas,
        flags: 0,
        base_temperature: 22.,
    },
    // 11 CARB
    VariantType {
        weight: 0,
        strength: 0,
        color: // black but not too black because the background is black
        (0x11, 0x11, 0x11),
        source_variant: Variant::CARB,
        variant_property: VariantProperty::Powder,
        flags: 0,
        base_temperature: 22.,
    },
    // 12 NITR
    VariantType {
        weight: 0,
        strength: 0,
        color: crate::BLUE,
        source_variant: Variant::NITR,
        variant_property: VariantProperty::Gas,
        flags: 0,
        base_temperature: 22.,
    },
    // 13 IRON
    VariantType {
        weight: 0,
        strength: 0,
        color: crate::IRON,
        source_variant: Variant::IRON,
        variant_property: VariantProperty::Powder,
        flags: 0,
        base_temperature: 22.,
    },
    // 14 CO2
    VariantType {
        weight: 0,
        strength: 0,
        color: crate::DARK_BLUE,
        source_variant: Variant::CO2,
        variant_property: VariantProperty::Gas,
        flags: 0,
        base_temperature: 22.,
    },
];
