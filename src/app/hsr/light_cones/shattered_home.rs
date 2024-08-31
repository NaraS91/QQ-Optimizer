use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.5000, 0.2000),
    (0.5000, 0.2500),
    (0.5000, 0.3000),
    (0.5000, 0.3500),
    (0.5000, 0.4000),
];
