use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1000, 2.0000),
    (0.1200, 2.0000),
    (0.1400, 2.0000),
    (0.1600, 2.0000),
    (0.1800, 2.0000),
];
