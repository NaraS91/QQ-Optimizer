use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.3200, 20.0000),
    (0.4000, 23.0000),
    (0.4800, 26.0000),
    (0.5600, 29.0000),
    (0.6400, 32.0000),
];
