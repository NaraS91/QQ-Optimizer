use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (0.7000, 0.3500),
    (0.7700, 0.3850),
    (0.8400, 0.4200),
    (0.9100, 0.4550),
    (0.9800, 0.4900),
    (1.0500, 0.5250),
    (1.1375, 0.5687),
    (1.2250, 0.6125),
    (1.3125, 0.6562),
    (1.4000, 0.7000),
    (1.4700, 0.7350),
    (1.5400, 0.7700),
    (1.6100, 0.8050),
    (1.6800, 0.8400),
    (1.7500, 0.8750),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (1.5000, 0.0900, 0.3600),
    (1.6000, 0.0960, 0.3840),
    (1.7000, 0.1020, 0.4080),
    (1.8000, 0.1080, 0.4320),
    (1.9000, 0.1140, 0.4560),
    (2.0000, 0.1200, 0.4800),
    (2.1250, 0.1275, 0.5100),
    (2.2500, 0.1350, 0.5400),
    (2.3750, 0.1425, 0.5700),
    (2.5000, 0.1500, 0.6000),
    (2.6000, 0.1560, 0.6240),
    (2.7000, 0.1620, 0.6480),
    (2.8000, 0.1680, 0.6720),
    (2.9000, 0.1740, 0.6960),
    (3.0000, 0.1800, 0.7200),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (8.0000, 0.4500, 1.0000),
    (8.0000, 0.4950, 1.0000),
    (8.0000, 0.5400, 1.0000),
    (8.0000, 0.5850, 1.0000),
    (8.0000, 0.6300, 1.0000),
    (8.0000, 0.6750, 1.0000),
    (8.0000, 0.7312, 1.0000),
    (8.0000, 0.7875, 1.0000),
    (8.0000, 0.8438, 1.0000),
    (8.0000, 0.9000, 1.0000),
    (8.0000, 0.9450, 1.0000),
    (8.0000, 0.9900, 1.0000),
    (8.0000, 1.0350, 1.0000),
    (8.0000, 1.0800, 1.0000),
    (8.0000, 1.1250, 1.0000),
];

const TECH_PARAMS: [f32; 1] = [0.8000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
