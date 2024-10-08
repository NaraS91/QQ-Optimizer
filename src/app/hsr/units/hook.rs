use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.2000, 1.0000, 2.0000, 0.2500),
    (1.3200, 1.0000, 2.0000, 0.2750),
    (1.4400, 1.0000, 2.0000, 0.3000),
    (1.5600, 1.0000, 2.0000, 0.3250),
    (1.6800, 1.0000, 2.0000, 0.3500),
    (1.8000, 1.0000, 2.0000, 0.3875),
    (1.9500, 1.0000, 2.0000, 0.4375),
    (2.1000, 1.0000, 2.0000, 0.5000),
    (2.2500, 1.0000, 2.0000, 0.5750),
    (2.4000, 1.0000, 2.0000, 0.6500),
    (2.5200, 1.0000, 2.0000, 0.6825),
    (2.6400, 1.0000, 2.0000, 0.7150),
    (2.7600, 1.0000, 2.0000, 0.7475),
    (2.8800, 1.0000, 2.0000, 0.7800),
    (3.0000, 1.0000, 2.0000, 0.8125),
];

const ULT_PARAMS: [f32; 15] = [
    2.4000, 2.5600, 2.7200, 2.8800, 3.0400, 3.2000, 3.4000, 3.6000, 3.8000, 4.0000, 4.1600, 4.3200,
    4.4800, 4.6400, 4.8000,
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.5000, 5.0000),
    (0.5500, 5.0000),
    (0.6000, 5.0000),
    (0.6500, 5.0000),
    (0.7000, 5.0000),
    (0.7500, 5.0000),
    (0.8125, 5.0000),
    (0.8750, 5.0000),
    (0.9375, 5.0000),
    (1.0000, 5.0000),
    (1.0500, 5.0000),
    (1.1000, 5.0000),
    (1.1500, 5.0000),
    (1.2000, 5.0000),
    (1.2500, 5.0000),
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(1.0000, 0.5000, 3.0000, 0.5000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
