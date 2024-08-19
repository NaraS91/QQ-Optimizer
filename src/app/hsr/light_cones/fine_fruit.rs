use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [f32; 5] = [
    6.0000,
    7.5000,
    9.0000,
    10.5000,
    12.0000,
];

