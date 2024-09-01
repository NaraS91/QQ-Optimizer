use crate::app::hsr::units::{Modifier, Unit};

pub fn modifiers(_wearer: &Unit) -> Vec<Modifier> {
    vec![]
}

const SKILL_PARAMS: [f32; 5] = [0.4800, 0.6000, 0.7200, 0.8400, 0.9600];
