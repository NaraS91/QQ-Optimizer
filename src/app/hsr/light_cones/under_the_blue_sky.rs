use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1600, 0.1200, 3.0000),
    (0.2000, 0.1500, 3.0000),
    (0.2400, 0.1800, 3.0000),
    (0.2800, 0.2100, 3.0000),
    (0.3200, 0.2400, 3.0000),
];