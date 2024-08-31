use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.1200, 100.0000, 0.0080, 0.3200),
    (0.1400, 100.0000, 0.0090, 0.3600),
    (0.1600, 100.0000, 0.0100, 0.4000),
    (0.1800, 100.0000, 0.0110, 0.4400),
    (0.2000, 100.0000, 0.0120, 0.4800),
];