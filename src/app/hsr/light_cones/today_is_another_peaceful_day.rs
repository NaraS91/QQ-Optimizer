use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.0020, 160.0000),
    (0.0025, 160.0000),
    (0.0030, 160.0000),
    (0.0035, 160.0000),
    (0.0040, 160.0000),
];
