use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (2.0000, 0.1600),
    (2.0000, 0.1800),
    (2.0000, 0.2000),
    (2.0000, 0.2200),
    (2.0000, 0.2400),
];
