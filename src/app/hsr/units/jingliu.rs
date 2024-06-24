use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (1.0000, 1.0000),
    (1.1000, 1.0000),
    (1.2000, 1.0000),
    (1.3000, 1.0000),
    (1.4000, 1.0000),
    (1.5000, 1.0000),
    (1.6250, 1.0000),
    (1.7500, 1.0000),
    (1.8750, 1.0000),
    (2.0000, 1.0000),
    (2.1000, 1.0000),
    (2.2000, 1.0000),
    (2.3000, 1.0000),
    (2.4000, 1.0000),
    (2.5000, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.8000, 1.0000, 0.9000, 1.0000),
    (1.9200, 1.0000, 0.9600, 1.0000),
    (2.0400, 1.0000, 1.0200, 1.0000),
    (2.1600, 1.0000, 1.0800, 1.0000),
    (2.2800, 1.0000, 1.1400, 1.0000),
    (2.4000, 1.0000, 1.2000, 1.0000),
    (2.5500, 1.0000, 1.2750, 1.0000),
    (2.7000, 1.0000, 1.3500, 1.0000),
    (2.8500, 1.0000, 1.4250, 1.0000),
    (3.0000, 1.0000, 1.5000, 1.0000),
    (3.1200, 1.0000, 1.5600, 1.0000),
    (3.2400, 1.0000, 1.6200, 1.0000),
    (3.3600, 1.0000, 1.6800, 1.0000),
    (3.4800, 1.0000, 1.7400, 1.0000),
    (3.6000, 1.0000, 1.8000, 1.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32, f32, f32); 15] = [
    (0.3000, 0.0400, 5.4000, 0.9000, 2.0000, 1.0000, 0.4000),
    (0.3000, 0.0400, 5.4000, 0.9900, 2.0000, 1.0000, 0.4100),
    (0.3000, 0.0400, 5.4000, 1.0800, 2.0000, 1.0000, 0.4200),
    (0.3000, 0.0400, 5.4000, 1.1700, 2.0000, 1.0000, 0.4300),
    (0.3000, 0.0400, 5.4000, 1.2600, 2.0000, 1.0000, 0.4400),
    (0.3000, 0.0400, 5.4000, 1.3500, 2.0000, 1.0000, 0.4500),
    (0.3000, 0.0400, 5.4000, 1.4625, 2.0000, 1.0000, 0.4625),
    (0.3000, 0.0400, 5.4000, 1.5750, 2.0000, 1.0000, 0.4750),
    (0.3000, 0.0400, 5.4000, 1.6875, 2.0000, 1.0000, 0.4875),
    (0.3000, 0.0400, 5.4000, 1.8000, 2.0000, 1.0000, 0.5000),
    (0.3000, 0.0400, 5.4000, 1.8900, 2.0000, 1.0000, 0.5100),
    (0.3000, 0.0400, 5.4000, 1.9800, 2.0000, 1.0000, 0.5200),
    (0.3000, 0.0400, 5.4000, 2.0700, 2.0000, 1.0000, 0.5300),
    (0.3000, 0.0400, 5.4000, 2.1600, 2.0000, 1.0000, 0.5400),
    (0.3000, 0.0400, 5.4000, 2.2500, 2.0000, 1.0000, 0.5500),
];

const TECH_PARAMS: [(f32, f32, f32, f32, f32, f32); 1] =
    [(1.0000, 1.0000, 20.0000, 1.0000, 0.8000, 15.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];