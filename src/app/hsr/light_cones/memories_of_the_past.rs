use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.2800, 4.0000),
    (0.3500, 5.0000),
    (0.4200, 6.0000),
    (0.4900, 7.0000),
    (0.5600, 8.0000),
];

