use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.6000, 0.1200, 1.0000),
    (0.7000, 0.1300, 1.0000),
    (0.8000, 0.1400, 1.0000),
    (0.9000, 0.1500, 1.0000),
    (1.0000, 0.1600, 1.0000),
];
