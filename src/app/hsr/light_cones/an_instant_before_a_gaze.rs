use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.3600, 0.0036, 180.0000),
    (0.4200, 0.0042, 180.0000),
    (0.4800, 0.0048, 180.0000),
    (0.5400, 0.0054, 180.0000),
    (0.6000, 0.0060, 180.0000),
];
