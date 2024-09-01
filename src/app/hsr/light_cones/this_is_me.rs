use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1600, 0.6000),
    (0.2000, 0.7500),
    (0.2400, 0.9000),
    (0.2800, 1.0500),
    (0.3200, 1.2000),
];
