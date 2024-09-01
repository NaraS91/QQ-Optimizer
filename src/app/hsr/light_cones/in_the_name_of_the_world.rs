use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.2400, 0.1800, 0.2400),
    (0.2800, 0.2100, 0.2800),
    (0.3200, 0.2400, 0.3200),
    (0.3600, 0.2700, 0.3600),
    (0.4000, 0.3000, 0.4000),
];
