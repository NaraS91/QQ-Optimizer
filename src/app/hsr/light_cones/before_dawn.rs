use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.3600, 0.1800, 0.4800),
    (0.4200, 0.2100, 0.5600),
    (0.4800, 0.2400, 0.6400),
    (0.5400, 0.2700, 0.7200),
    (0.6000, 0.3000, 0.8000),
];

