use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.1000, 2.0000),
    (0.1250, 2.5000),
    (0.1500, 3.0000),
    (0.1750, 3.5000),
    (0.2000, 4.0000),
];
