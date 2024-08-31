use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 5] = [
    (2.0000, 0.2400, 0.2400, 0.2400),
    (2.0000, 0.2800, 0.2800, 0.2800),
    (2.0000, 0.3200, 0.3200, 0.3200),
    (2.0000, 0.3600, 0.3600, 0.3600),
    (2.0000, 0.4000, 0.4000, 0.4000),
];
