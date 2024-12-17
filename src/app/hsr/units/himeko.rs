use crate::app::hsr::{
    basics::Element,
    units::{AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat},
    utils::flat_value,
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Technique),
        vec![ModifierData::new_dimension(
            ModifierTarget::Enemies,
            Stat::Advanced(AdvancedStat::ElemDmgReceived(Element::Fire)),
            BuffScaling::Additive,
            flat_value!(0.1),
        )],
        true,
    ))]
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (1.0000, 0.4000),
    (1.1000, 0.4400),
    (1.2000, 0.4800),
    (1.3000, 0.5200),
    (1.4000, 0.5600),
    (1.5000, 0.6000),
    (1.6250, 0.6500),
    (1.7500, 0.7000),
    (1.8750, 0.7500),
    (2.0000, 0.8000),
    (2.1000, 0.8400),
    (2.2000, 0.8800),
    (2.3000, 0.9200),
    (2.4000, 0.9600),
    (2.5000, 1.0000),
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (1.3800, 5.0000),
    (1.4720, 5.0000),
    (1.5640, 5.0000),
    (1.6560, 5.0000),
    (1.7480, 5.0000),
    (1.8400, 5.0000),
    (1.9550, 5.0000),
    (2.0700, 5.0000),
    (2.1850, 5.0000),
    (2.3000, 5.0000),
    (2.3920, 5.0000),
    (2.4840, 5.0000),
    (2.5760, 5.0000),
    (2.6680, 5.0000),
    (2.7600, 5.0000),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.7000, 3.0000),
    (0.7700, 3.0000),
    (0.8400, 3.0000),
    (0.9100, 3.0000),
    (0.9800, 3.0000),
    (1.0500, 3.0000),
    (1.1375, 3.0000),
    (1.2250, 3.0000),
    (1.3125, 3.0000),
    (1.4000, 3.0000),
    (1.4700, 3.0000),
    (1.5400, 3.0000),
    (1.6100, 3.0000),
    (1.6800, 3.0000),
    (1.7500, 3.0000),
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(1.0000, 0.1000, 2.0000, 15.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
