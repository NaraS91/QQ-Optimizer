use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 5] = [
    (0.1200, 0.1000, 0.0240, 5.0000, 2.0000),
    (0.1400, 0.1100, 0.0280, 5.0000, 2.0000),
    (0.1600, 0.1200, 0.0320, 5.0000, 2.0000),
    (0.1800, 0.1300, 0.0360, 5.0000, 2.0000),
    (0.2000, 0.1400, 0.0400, 5.0000, 2.0000),
];

