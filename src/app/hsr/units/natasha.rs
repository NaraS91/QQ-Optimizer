use super::{ModifierSource, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierSource>{
    vec![]
}


const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.0700, 0.0480, 2.0000, 70.0000, 48.0000),
    (0.0744, 0.0510, 2.0000, 112.0000, 76.8000),
    (0.0788, 0.0540, 2.0000, 143.5000, 98.4000),
    (0.0831, 0.0570, 2.0000, 175.0000, 120.0000),
    (0.0875, 0.0600, 2.0000, 196.0000, 134.4000),
    (0.0910, 0.0624, 2.0000, 217.0000, 148.8000),
    (0.0945, 0.0648, 2.0000, 232.7500, 159.6000),
    (0.0980, 0.0672, 2.0000, 248.5000, 170.4000),
    (0.1015, 0.0696, 2.0000, 264.2500, 181.2000),
    (0.1050, 0.0720, 2.0000, 280.0000, 192.0000),
    (0.1085, 0.0744, 2.0000, 295.7500, 202.8000),
    (0.1120, 0.0768, 2.0000, 311.5000, 213.6000),
    (0.1155, 0.0792, 2.0000, 327.2500, 224.4000),
    (0.1190, 0.0816, 2.0000, 343.0000, 235.2000),
    (0.1225, 0.0840, 2.0000, 358.7500, 246.0000),
];


const ULT_PARAMS: [(f32, f32); 15] = [
    (0.0920, 92.0000),
    (0.0978, 147.2000),
    (0.1035, 188.6000),
    (0.1092, 230.0000),
    (0.1150, 257.6000),
    (0.1196, 285.2000),
    (0.1242, 305.9000),
    (0.1288, 326.6000),
    (0.1334, 347.3000),
    (0.1380, 368.0000),
    (0.1426, 388.7000),
    (0.1472, 409.4000),
    (0.1518, 430.1000),
    (0.1564, 450.8000),
    (0.1610, 471.5000),
];


const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.3000, 0.2500),
    (0.3000, 0.2750),
    (0.3000, 0.3000),
    (0.3000, 0.3250),
    (0.3000, 0.3500),
    (0.3000, 0.3750),
    (0.3000, 0.4062),
    (0.3000, 0.4375),
    (0.3000, 0.4688),
    (0.3000, 0.5000),
    (0.3000, 0.5250),
    (0.3000, 0.5500),
    (0.3000, 0.5750),
    (0.3000, 0.6000),
    (0.3000, 0.6250),
];


const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [
    (1.0000, 0.3000, 1.0000, 0.8000),
];


const BASIC_PARAMS: [f32; 9] = [
    0.5000,
    0.6000,
    0.7000,
    0.8000,
    0.9000,
    1.0000,
    1.1000,
    1.2000,
    1.3000,
];

