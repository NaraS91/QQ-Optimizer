use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (1.0500, 0.5000, 0.3300),
    (1.1550, 0.5500, 0.3300),
    (1.2600, 0.6000, 0.3300),
    (1.3650, 0.6500, 0.3300),
    (1.4700, 0.7000, 0.3300),
    (1.5750, 0.7500, 0.3300),
    (1.7063, 0.8125, 0.3300),
    (1.8375, 0.8750, 0.3300),
    (1.9688, 0.9375, 0.3300),
    (2.1000, 1.0000, 0.3300),
    (2.2050, 1.0500, 0.3300),
    (2.3100, 1.1000, 0.3300),
    (2.4150, 1.1500, 0.3300),
    (2.5200, 1.2000, 0.3300),
    (2.6250, 1.2500, 0.3300),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.9200, 2.0000, 0.5000, 0.1800),
    (2.0480, 2.0000, 0.5000, 0.1920),
    (2.1760, 2.0000, 0.5000, 0.2040),
    (2.3040, 2.0000, 0.5000, 0.2160),
    (2.4320, 2.0000, 0.5000, 0.2280),
    (2.5600, 2.0000, 0.5000, 0.2400),
    (2.7200, 2.0000, 0.5000, 0.2550),
    (2.8800, 2.0000, 0.5000, 0.2700),
    (3.0400, 2.0000, 0.5000, 0.2850),
    (3.2000, 2.0000, 0.5000, 0.3000),
    (3.3280, 2.0000, 0.5000, 0.3120),
    (3.4560, 2.0000, 0.5000, 0.3240),
    (3.5840, 2.0000, 0.5000, 0.3360),
    (3.7120, 2.0000, 0.5000, 0.3480),
    (3.8400, 2.0000, 0.5000, 0.3600),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.1500, 2.0000),
    (0.1550, 2.0000),
    (0.1600, 2.0000),
    (0.1650, 2.0000),
    (0.1700, 2.0000),
    (0.1750, 2.0000),
    (0.1812, 2.0000),
    (0.1875, 2.0000),
    (0.1938, 2.0000),
    (0.2000, 2.0000),
    (0.2050, 2.0000),
    (0.2100, 2.0000),
    (0.2150, 2.0000),
    (0.2200, 2.0000),
    (0.2250, 2.0000),
];

const TECH_PARAMS: [f32; 1] = [0.8000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
