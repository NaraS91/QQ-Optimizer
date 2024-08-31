use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.2400, 0.0200, 800.0000),
    (0.3000, 0.0250, 800.0000),
    (0.3600, 0.0300, 800.0000),
    (0.4200, 0.0350, 800.0000),
    (0.4800, 0.0400, 800.0000),
];
