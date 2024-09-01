use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.3000, 0.3600, 1.0000, 3.0000),
    (0.3500, 0.4200, 1.0000, 3.0000),
    (0.4000, 0.4800, 1.0000, 3.0000),
    (0.4500, 0.5400, 1.0000, 3.0000),
    (0.5000, 0.6000, 1.0000, 3.0000),
];
