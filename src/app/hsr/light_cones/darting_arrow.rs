use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.2400, 3.0000),
    (0.3000, 3.0000),
    (0.3600, 3.0000),
    (0.4200, 3.0000),
    (0.4800, 3.0000),
];
