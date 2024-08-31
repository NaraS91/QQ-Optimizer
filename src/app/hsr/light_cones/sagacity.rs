use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.2400, 2.0000),
    (0.3000, 2.0000),
    (0.3600, 2.0000),
    (0.4200, 2.0000),
    (0.4800, 2.0000),
];
