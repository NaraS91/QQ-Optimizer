use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1600, 0.0200),
    (0.2000, 0.0250),
    (0.2400, 0.0300),
    (0.2800, 0.0350),
    (0.3200, 0.0400),
];

