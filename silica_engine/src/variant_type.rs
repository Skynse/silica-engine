use crate::variant::Variant;
pub const VARIANT_COUNT: usize = 8;

#[derive(PartialEq, Eq)]
pub struct VariantType {
    pub weight: u8,
    pub color: (u8, u8, u8),
    pub strength: u8,
    pub source_variant: Variant,
    pub variant_property: VariantProperty,
    pub flags: u8,
} // flags
pub const FLAG_BURNS: u8 = 0b00000001;
pub const FLAG_EXPLOSIVE: u8 = 0b00000010;

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
    },
    // 1 Wall
    VariantType {
        weight: 0,
        strength: 0,
        color: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Wall,
        flags: 0,
        variant_property: VariantProperty::Solid,
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
    },
    // 3 Water
    VariantType {
        weight: 32,
        strength: 0,
        color: (0x00, 0x00, 0xFF),
        source_variant: Variant::Water,
        variant_property: VariantProperty::Liquid,
        flags: 0,
    },
    // 4 Fire
    VariantType {
        weight: 64,
        strength: 16,
        color: (0xFF, 0x00, 0x00),
        source_variant: Variant::Fire,
        variant_property: VariantProperty::Gas,
        flags: FLAG_BURNS,
    },
    // 5 Smoke
    VariantType {
        weight: 1,
        strength: 32,
        color: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Smoke,
        variant_property: VariantProperty::Gas,
        flags: 0,
    },
    // 6 Salt
    VariantType {
        weight: 1,
        strength: 0,
        color: (0xFF, 0xFF, 0xFF),
        source_variant: Variant::Salt,
        variant_property: VariantProperty::Powder,
        flags: 0,
    },
    // 7 SaltWater
    VariantType {
        weight: 32,
        strength: 0,
        color: crate::LIGHT_BLUE,
        source_variant: Variant::SaltWater,
        variant_property: VariantProperty::Liquid,
        flags: 0,
    },
];
