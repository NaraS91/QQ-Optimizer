use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1600, 0.2000, 2.0000),
    (0.1800, 0.2500, 2.0000),
    (0.2000, 0.3000, 2.0000),
    (0.2200, 0.3500, 2.0000),
    (0.2400, 0.4000, 2.0000),
];
