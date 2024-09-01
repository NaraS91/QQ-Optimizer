use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.1800, 0.3000, 0.1200, 2.0000),
    (0.2100, 0.3500, 0.1400, 2.0000),
    (0.2400, 0.4000, 0.1600, 2.0000),
    (0.2700, 0.4500, 0.1800, 2.0000),
    (0.3000, 0.5000, 0.2000, 2.0000),
];
