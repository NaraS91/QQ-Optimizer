use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1000, 0.1200, 0.0600),
    (0.1250, 0.1500, 0.0750),
    (0.1500, 0.1800, 0.0900),
    (0.1750, 0.2100, 0.1050),
    (0.2000, 0.2400, 0.1200),
];
