use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1600, 0.0800),
    (0.1800, 0.0900),
    (0.2000, 0.1000),
    (0.2200, 0.1100),
    (0.2400, 0.1200),
];
