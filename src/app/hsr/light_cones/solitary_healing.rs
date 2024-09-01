use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.2000, 0.2400, 2.0000, 4.0000),
    (0.2500, 0.3000, 2.0000, 4.5000),
    (0.3000, 0.3600, 2.0000, 5.0000),
    (0.3500, 0.4200, 2.0000, 5.5000),
    (0.4000, 0.4800, 2.0000, 6.0000),
];
