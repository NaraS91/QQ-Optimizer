use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1800, 0.1200, 0.3600),
    (0.2100, 0.1400, 0.4200),
    (0.2400, 0.1600, 0.4800),
    (0.2700, 0.1800, 0.5400),
    (0.3000, 0.2000, 0.6000),
];
