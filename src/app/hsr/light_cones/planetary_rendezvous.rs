use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [f32; 5] = [0.1200, 0.1500, 0.1800, 0.2100, 0.2400];
