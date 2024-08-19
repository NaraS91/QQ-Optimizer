use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.2400, 0.5000, 0.2400),
    (0.3000, 0.5000, 0.3000),
    (0.3600, 0.5000, 0.3600),
    (0.4200, 0.5000, 0.4200),
    (0.4800, 0.5000, 0.4800),
];

