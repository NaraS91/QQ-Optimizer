use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1600, 0.3300, 0.1500),
    (0.2000, 0.3600, 0.1800),
    (0.2400, 0.3900, 0.2100),
    (0.2800, 0.4200, 0.2400),
    (0.3200, 0.4500, 0.2700),
];
