use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.2000, 4.0000),
    (0.2500, 5.0000),
    (0.3000, 6.0000),
    (0.3500, 7.0000),
    (0.4000, 8.0000),
];
