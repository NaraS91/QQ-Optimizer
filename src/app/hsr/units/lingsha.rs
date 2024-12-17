use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.4000, 0.1000, 105.0000, 0.2000),
    (0.4400, 0.1050, 168.0000, 0.2000),
    (0.4800, 0.1100, 215.2500, 0.2000),
    (0.5200, 0.1150, 262.5000, 0.2000),
    (0.5600, 0.1200, 294.0000, 0.2000),
    (0.6000, 0.1240, 325.5000, 0.2000),
    (0.6500, 0.1280, 349.1250, 0.2000),
    (0.7000, 0.1320, 372.7500, 0.2000),
    (0.7500, 0.1360, 396.3750, 0.2000),
    (0.8000, 0.1400, 420.0000, 0.2000),
    (0.8400, 0.1440, 443.6250, 0.2000),
    (0.8800, 0.1480, 467.2500, 0.2000),
    (0.9200, 0.1520, 490.8750, 0.2000),
    (0.9600, 0.1560, 514.5000, 0.2000),
    (1.0000, 0.1600, 538.1250, 0.2000),
];

const ULT_PARAMS: [(f32, f32, f32, f32, f32, f32); 15] = [
    (0.9000, 0.0800, 90.0000, 0.1500, 2.0000, 1.0000),
    (0.9600, 0.0850, 144.0000, 0.1600, 2.0000, 1.0000),
    (1.0200, 0.0900, 184.5000, 0.1700, 2.0000, 1.0000),
    (1.0800, 0.0950, 225.0000, 0.1800, 2.0000, 1.0000),
    (1.1400, 0.1000, 252.0000, 0.1900, 2.0000, 1.0000),
    (1.2000, 0.1040, 279.0000, 0.2000, 2.0000, 1.0000),
    (1.2750, 0.1080, 299.2500, 0.2125, 2.0000, 1.0000),
    (1.3500, 0.1120, 319.5000, 0.2250, 2.0000, 1.0000),
    (1.4250, 0.1160, 339.7500, 0.2375, 2.0000, 1.0000),
    (1.5000, 0.1200, 360.0000, 0.2500, 2.0000, 1.0000),
    (1.5600, 0.1240, 380.2500, 0.2600, 2.0000, 1.0000),
    (1.6200, 0.1280, 400.5000, 0.2700, 2.0000, 1.0000),
    (1.6800, 0.1320, 420.7500, 0.2800, 2.0000, 1.0000),
    (1.7400, 0.1360, 441.0000, 0.2900, 2.0000, 1.0000),
    (1.8000, 0.1400, 461.2500, 0.3000, 2.0000, 1.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32, f32, f32, f32); 15] = [
    (
        90.0000, 0.3750, 0.0800, 90.0000, 5.0000, 1.0000, 3.0000, 0.3750,
    ),
    (
        90.0000, 0.4125, 0.0850, 144.0000, 5.0000, 1.0000, 3.0000, 0.4125,
    ),
    (
        90.0000, 0.4500, 0.0900, 184.5000, 5.0000, 1.0000, 3.0000, 0.4500,
    ),
    (
        90.0000, 0.4875, 0.0950, 225.0000, 5.0000, 1.0000, 3.0000, 0.4875,
    ),
    (
        90.0000, 0.5250, 0.1000, 252.0000, 5.0000, 1.0000, 3.0000, 0.5250,
    ),
    (
        90.0000, 0.5625, 0.1040, 279.0000, 5.0000, 1.0000, 3.0000, 0.5625,
    ),
    (
        90.0000, 0.6094, 0.1080, 299.2500, 5.0000, 1.0000, 3.0000, 0.6094,
    ),
    (
        90.0000, 0.6562, 0.1120, 319.5000, 5.0000, 1.0000, 3.0000, 0.6562,
    ),
    (
        90.0000, 0.7031, 0.1160, 339.7500, 5.0000, 1.0000, 3.0000, 0.7031,
    ),
    (
        90.0000, 0.7500, 0.1200, 360.0000, 5.0000, 1.0000, 3.0000, 0.7500,
    ),
    (
        90.0000, 0.7875, 0.1240, 380.2500, 5.0000, 1.0000, 3.0000, 0.7875,
    ),
    (
        90.0000, 0.8250, 0.1280, 400.5000, 5.0000, 1.0000, 3.0000, 0.8250,
    ),
    (
        90.0000, 0.8625, 0.1320, 420.7500, 5.0000, 1.0000, 3.0000, 0.8625,
    ),
    (
        90.0000, 0.9000, 0.1360, 441.0000, 5.0000, 1.0000, 3.0000, 0.9000,
    ),
    (
        90.0000, 0.9375, 0.1400, 461.2500, 5.0000, 1.0000, 3.0000, 0.9375,
    ),
];

const TECH_PARAMS: [f32; 1] = [2.0000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
