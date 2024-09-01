use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.2800, 0.1500, 2.0000),
    (0.3500, 0.1875, 2.0000),
    (0.4200, 0.2250, 2.0000),
    (0.4900, 0.2625, 2.0000),
    (0.5600, 0.3000, 2.0000),
];
