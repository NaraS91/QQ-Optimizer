use super::{ModifierSource, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierSource>{
    vec![]
}


const SKILL_PARAMS: [f32; 15] = [
    0.6000,
    0.6600,
    0.7200,
    0.7800,
    0.8400,
    0.9000,
    0.9750,
    1.0500,
    1.1250,
    1.2000,
    1.2600,
    1.3200,
    1.3800,
    1.4400,
    1.5000,
];


const ULT_PARAMS: [(f32, f32); 15] = [
    (0.9600, 90.0000),
    (1.0240, 90.0000),
    (1.0880, 90.0000),
    (1.1520, 90.0000),
    (1.2160, 90.0000),
    (1.2800, 90.0000),
    (1.3600, 90.0000),
    (1.4400, 90.0000),
    (1.5200, 90.0000),
    (1.6000, 90.0000),
    (1.6640, 90.0000),
    (1.7280, 90.0000),
    (1.7920, 90.0000),
    (1.8560, 90.0000),
    (1.9200, 90.0000),
];


const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (3.0000, 0.0100, 10.0000),
    (3.0000, 0.0115, 10.0000),
    (3.0000, 0.0130, 10.0000),
    (3.0000, 0.0145, 10.0000),
    (3.0000, 0.0160, 10.0000),
    (3.0000, 0.0175, 10.0000),
    (3.0000, 0.0194, 10.0000),
    (3.0000, 0.0213, 10.0000),
    (3.0000, 0.0231, 10.0000),
    (3.0000, 0.0250, 10.0000),
    (3.0000, 0.0265, 10.0000),
    (3.0000, 0.0280, 10.0000),
    (3.0000, 0.0295, 10.0000),
    (3.0000, 0.0310, 10.0000),
    (3.0000, 0.0325, 10.0000),
];


const TECH_PARAMS: [(f32, f32, f32); 1] = [
    (10.0000, 0.8000, 15.0000),
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

