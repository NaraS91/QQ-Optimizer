use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.2400, 0.0800, 0.2400),
    (0.2800, 0.0900, 0.2800),
    (0.3200, 0.1000, 0.3200),
    (0.3600, 0.1100, 0.3600),
    (0.4000, 0.1200, 0.4000),
];

