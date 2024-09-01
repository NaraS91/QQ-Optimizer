use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.3000, 0.0800, 5.0000),
    (0.3500, 0.1000, 5.0000),
    (0.4000, 0.1200, 5.0000),
    (0.4500, 0.1400, 5.0000),
    (0.5000, 0.1600, 5.0000),
];
