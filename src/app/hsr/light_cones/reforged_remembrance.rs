use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.4000, 0.0500, 0.0720, 4.0000),
    (0.4500, 0.0600, 0.0790, 4.0000),
    (0.5000, 0.0700, 0.0860, 4.0000),
    (0.5500, 0.0800, 0.0930, 4.0000),
    (0.6000, 0.0900, 0.1000, 4.0000),
];

