use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1200, 0.0400, 3.0000),
    (0.1500, 0.0500, 3.0000),
    (0.1800, 0.0600, 3.0000),
    (0.2100, 0.0700, 3.0000),
    (0.2400, 0.0800, 3.0000),
];

