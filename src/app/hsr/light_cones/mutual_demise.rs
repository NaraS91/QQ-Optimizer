use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.8000, 0.1200),
    (0.8000, 0.1500),
    (0.8000, 0.1800),
    (0.8000, 0.2100),
    (0.8000, 0.2400),
];

