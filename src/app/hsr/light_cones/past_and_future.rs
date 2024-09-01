use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1600, 1.0000),
    (0.2000, 1.0000),
    (0.2400, 1.0000),
    (0.2800, 1.0000),
    (0.3200, 1.0000),
];
