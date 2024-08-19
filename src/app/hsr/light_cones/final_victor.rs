use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1200, 0.0800, 4.0000),
    (0.1400, 0.0900, 4.0000),
    (0.1600, 0.1000, 4.0000),
    (0.1800, 0.1100, 4.0000),
    (0.2000, 0.1200, 4.0000),
];

