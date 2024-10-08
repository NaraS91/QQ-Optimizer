use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32, f32); 5] = [
    (0.3200, 0.2800, 4.0000, 4.0000, 0.1000, 3.0000),
    (0.3900, 0.3500, 4.0000, 4.0000, 0.1100, 3.0000),
    (0.4600, 0.4200, 4.0000, 4.0000, 0.1200, 3.0000),
    (0.5300, 0.4900, 4.0000, 4.0000, 0.1300, 3.0000),
    (0.6000, 0.5600, 4.0000, 4.0000, 0.1400, 3.0000),
];
