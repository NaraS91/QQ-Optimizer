use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.5000, 8.0000),
    (0.5000, 10.0000),
    (0.5000, 12.0000),
    (0.5000, 14.0000),
    (0.5000, 16.0000),
];

