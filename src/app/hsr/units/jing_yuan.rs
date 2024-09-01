use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (0.5000, 2.0000),
    (0.5500, 2.0000),
    (0.6000, 2.0000),
    (0.6500, 2.0000),
    (0.7000, 2.0000),
    (0.7500, 2.0000),
    (0.8125, 2.0000),
    (0.8750, 2.0000),
    (0.9375, 2.0000),
    (1.0000, 2.0000),
    (1.0500, 2.0000),
    (1.1000, 2.0000),
    (1.1500, 2.0000),
    (1.2000, 2.0000),
    (1.2500, 2.0000),
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (1.2000, 3.0000),
    (1.2800, 3.0000),
    (1.3600, 3.0000),
    (1.4400, 3.0000),
    (1.5200, 3.0000),
    (1.6000, 3.0000),
    (1.7000, 3.0000),
    (1.8000, 3.0000),
    (1.9000, 3.0000),
    (2.0000, 3.0000),
    (2.0800, 3.0000),
    (2.1600, 3.0000),
    (2.2400, 3.0000),
    (2.3200, 3.0000),
    (2.4000, 3.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32, f32); 15] = [
    (60.0000, 0.3300, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.3630, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.3960, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.4290, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.4620, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.4950, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.5363, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.5775, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.6188, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.6600, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.6930, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.7260, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.7590, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.7920, 10.0000, 3.0000, 0.2500, 10.0000),
    (60.0000, 0.8250, 10.0000, 3.0000, 0.2500, 10.0000),
];

const TECH_PARAMS: [f32; 1] = [3.0000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
