use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (0.2000, 3.0000, 0.1400, 0.1200),
    (0.2300, 3.0000, 0.1650, 0.1400),
    (0.2600, 3.0000, 0.1900, 0.1600),
    (0.2900, 3.0000, 0.2150, 0.1800),
    (0.3200, 3.0000, 0.2400, 0.2000),
];

