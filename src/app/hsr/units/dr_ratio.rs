use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, UnitKind,
    },
    utils::flat_value,
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![ModifierOrDOT::Modifier(Modifier::new(
        (UnitKind::Dr_Ratio, Source::Trace(2)),
        vec![ModifierData::new(
            ModifierTarget::Enemy,
            Stat::Advanced(AdvancedStat::EffectRes),
            BuffScaling::Additive,
            flat_value!(-0.1),
        )],
        true,
    ))]
}

const SKILL_PARAMS: [f32; 15] = [
    0.7500, 0.8250, 0.9000, 0.9750, 1.0500, 1.1250, 1.2188, 1.3125, 1.4062, 1.5000, 1.5750, 1.6500,
    1.7250, 1.8000, 1.8750,
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (1.4400, 2.0000),
    (1.5360, 2.0000),
    (1.6320, 2.0000),
    (1.7280, 2.0000),
    (1.8240, 2.0000),
    (1.9200, 2.0000),
    (2.0400, 2.0000),
    (2.1600, 2.0000),
    (2.2800, 2.0000),
    (2.4000, 2.0000),
    (2.4960, 2.0000),
    (2.5920, 2.0000),
    (2.6880, 2.0000),
    (2.7840, 2.0000),
    (2.8800, 2.0000),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (1.3500, 0.4000, 0.2000),
    (1.4850, 0.4000, 0.2000),
    (1.6200, 0.4000, 0.2000),
    (1.7550, 0.4000, 0.2000),
    (1.8900, 0.4000, 0.2000),
    (2.0250, 0.4000, 0.2000),
    (2.1938, 0.4000, 0.2000),
    (2.3625, 0.4000, 0.2000),
    (2.5312, 0.4000, 0.2000),
    (2.7000, 0.4000, 0.2000),
    (2.8350, 0.4000, 0.2000),
    (2.9700, 0.4000, 0.2000),
    (3.1050, 0.4000, 0.2000),
    (3.2400, 0.4000, 0.2000),
    (3.3750, 0.4000, 0.2000),
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(10.0000, 1.0000, 0.1500, 2.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
