use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (0.0600, 0.1000, 1.0000),
    (0.0640, 0.1100, 1.0000),
    (0.0680, 0.1200, 1.0000),
    (0.0720, 0.1300, 1.0000),
    (0.0760, 0.1400, 1.0000),
    (0.0800, 0.1500, 1.0000),
    (0.0850, 0.1625, 1.0000),
    (0.0900, 0.1750, 1.0000),
    (0.0950, 0.1875, 1.0000),
    (0.1000, 0.2000, 1.0000),
    (0.1040, 0.2100, 1.0000),
    (0.1080, 0.2200, 1.0000),
    (0.1120, 0.2300, 1.0000),
    (0.1160, 0.2400, 1.0000),
    (0.1200, 0.2500, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (1.4400, 2.0000, 0.2000),
    (1.5360, 2.0000, 0.2000),
    (1.6320, 2.0000, 0.2000),
    (1.7280, 2.0000, 0.2000),
    (1.8240, 2.0000, 0.2000),
    (1.9200, 2.0000, 0.2000),
    (2.0400, 2.0000, 0.2000),
    (2.1600, 2.0000, 0.2000),
    (2.2800, 2.0000, 0.2000),
    (2.4000, 2.0000, 0.2000),
    (2.4960, 2.0000, 0.2000),
    (2.5920, 2.0000, 0.2000),
    (2.6880, 2.0000, 0.2000),
    (2.7840, 2.0000, 0.2000),
    (2.8800, 2.0000, 0.2000),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (7.0000, 0.4000, 10.0000),
    (7.0000, 0.4400, 10.0000),
    (7.0000, 0.4800, 10.0000),
    (7.0000, 0.5200, 10.0000),
    (7.0000, 0.5600, 10.0000),
    (7.0000, 0.6000, 10.0000),
    (7.0000, 0.6500, 10.0000),
    (7.0000, 0.7000, 10.0000),
    (7.0000, 0.7500, 10.0000),
    (7.0000, 0.8000, 10.0000),
    (7.0000, 0.8400, 10.0000),
    (7.0000, 0.8800, 10.0000),
    (7.0000, 0.9200, 10.0000),
    (7.0000, 0.9600, 10.0000),
    (7.0000, 1.0000, 10.0000),
];

const TECH_PARAMS: [(f32, f32); 1] = [(3.0000, 30.0000)];

const BASIC_PARAMS: [(f32, f32); 9] = [
    (0.5000, 1.0000),
    (0.6000, 1.0000),
    (0.7000, 1.0000),
    (0.8000, 1.0000),
    (0.9000, 1.0000),
    (1.0000, 1.0000),
    (1.1000, 1.0000),
    (1.2000, 1.0000),
    (1.3000, 1.0000),
];
