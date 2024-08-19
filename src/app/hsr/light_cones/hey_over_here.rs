use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.0800, 0.1600, 2.0000),
    (0.0900, 0.1900, 2.0000),
    (0.1000, 0.2200, 2.0000),
    (0.1100, 0.2500, 2.0000),
    (0.1200, 0.2800, 2.0000),
];

