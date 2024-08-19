use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [f32; 5] = [
    0.6000,
    0.7500,
    0.9000,
    1.0500,
    1.2000,
];

