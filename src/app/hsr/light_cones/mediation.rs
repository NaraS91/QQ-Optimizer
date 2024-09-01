use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (12.0000, 1.0000),
    (14.0000, 1.0000),
    (16.0000, 1.0000),
    (18.0000, 1.0000),
    (20.0000, 1.0000),
];
