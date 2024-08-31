use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.0800, 4.0000, 0.1200, 2.0000),
    (0.1000, 4.0000, 0.1500, 2.0000),
    (0.1200, 4.0000, 0.1800, 2.0000),
    (0.1400, 4.0000, 0.2100, 2.0000),
    (0.1600, 4.0000, 0.2400, 2.0000),
];
