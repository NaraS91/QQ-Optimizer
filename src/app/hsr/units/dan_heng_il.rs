use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (0.0600, 4.0000),
    (0.0660, 4.0000),
    (0.0720, 4.0000),
    (0.0780, 4.0000),
    (0.0840, 4.0000),
    (0.0900, 4.0000),
    (0.0975, 4.0000),
    (0.1050, 4.0000),
    (0.1125, 4.0000),
    (0.1200, 4.0000),
    (0.1260, 4.0000),
    (0.1320, 4.0000),
    (0.1380, 4.0000),
    (0.1440, 4.0000),
    (0.1500, 4.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.8000, 0.8400, 2.0000, 3.0000),
    (1.9200, 0.8960, 2.0000, 3.0000),
    (2.0400, 0.9520, 2.0000, 3.0000),
    (2.1600, 1.0080, 2.0000, 3.0000),
    (2.2800, 1.0640, 2.0000, 3.0000),
    (2.4000, 1.1200, 2.0000, 3.0000),
    (2.5500, 1.1900, 2.0000, 3.0000),
    (2.7000, 1.2600, 2.0000, 3.0000),
    (2.8500, 1.3300, 2.0000, 3.0000),
    (3.0000, 1.4000, 2.0000, 3.0000),
    (3.1200, 1.4560, 2.0000, 3.0000),
    (3.2400, 1.5120, 2.0000, 3.0000),
    (3.3600, 1.5680, 2.0000, 3.0000),
    (3.4800, 1.6240, 2.0000, 3.0000),
    (3.6000, 1.6800, 2.0000, 3.0000),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.0500, 6.0000),
    (0.0550, 6.0000),
    (0.0600, 6.0000),
    (0.0650, 6.0000),
    (0.0700, 6.0000),
    (0.0750, 6.0000),
    (0.0813, 6.0000),
    (0.0875, 6.0000),
    (0.0938, 6.0000),
    (0.1000, 6.0000),
    (0.1050, 6.0000),
    (0.1100, 6.0000),
    (0.1150, 6.0000),
    (0.1200, 6.0000),
    (0.1250, 6.0000),
];

const TECH_PARAMS: [(f32, f32, f32); 1] = [(1.0000, 20.0000, 1.2000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
