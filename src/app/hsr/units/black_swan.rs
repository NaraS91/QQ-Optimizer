use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, DOTKind, Modifier, ModifierData, ModifierTarget,
        Source, Stat, UnitKind,
    },
    utils::{flat_value, value_with_buffer},
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Black_Swan, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                value_with_buffer!(|buffer: &Unit| SKILL_PARAMS[(buffer.unique_data.skill_level
                    + if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    }) as usize]
                    .3),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Black_Swan, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::Vulnerability),
                BuffScaling::Additive,
                value_with_buffer!(
                    |buffer: &Unit| ULT_PARAMS[(buffer.unique_data.ultimate_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize]
                        .2
                ),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Black_Swan, Source::Eidolon(1)),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::TotalDmgRes),
                BuffScaling::Additive,
                flat_value!(-0.2),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Black_Swan, Source::Eidolon(4)),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::EffectRes),
                BuffScaling::Additive,
                flat_value!(-0.1),
            )],
            true,
        )),
        //TODO: properly add config and stacks
        ModifierOrDOT::DOT(DOTKind::Arcana(5)),
    ]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.4500, 1.0000, 1.0000, 0.1480, 3.0000),
    (0.4950, 1.0000, 1.0000, 0.1540, 3.0000),
    (0.5400, 1.0000, 1.0000, 0.1600, 3.0000),
    (0.5850, 1.0000, 1.0000, 0.1660, 3.0000),
    (0.6300, 1.0000, 1.0000, 0.1720, 3.0000),
    (0.6750, 1.0000, 1.0000, 0.1780, 3.0000),
    (0.7312, 1.0000, 1.0000, 0.1855, 3.0000),
    (0.7875, 1.0000, 1.0000, 0.1930, 3.0000),
    (0.8438, 1.0000, 1.0000, 0.2005, 3.0000),
    (0.9000, 1.0000, 1.0000, 0.2080, 3.0000),
    (0.9450, 1.0000, 1.0000, 0.2140, 3.0000),
    (0.9900, 1.0000, 1.0000, 0.2200, 3.0000),
    (1.0350, 1.0000, 1.0000, 0.2260, 3.0000),
    (1.0800, 1.0000, 1.0000, 0.2320, 3.0000),
    (1.1250, 1.0000, 1.0000, 0.2380, 3.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.7200, 2.0000, 0.1500, 1.0000),
    (0.7680, 2.0000, 0.1600, 1.0000),
    (0.8160, 2.0000, 0.1700, 1.0000),
    (0.8640, 2.0000, 0.1800, 1.0000),
    (0.9120, 2.0000, 0.1900, 1.0000),
    (0.9600, 2.0000, 0.2000, 1.0000),
    (1.0200, 2.0000, 0.2125, 1.0000),
    (1.0800, 2.0000, 0.2250, 1.0000),
    (1.1400, 2.0000, 0.2375, 1.0000),
    (1.2000, 2.0000, 0.2500, 1.0000),
    (1.2480, 2.0000, 0.2600, 1.0000),
    (1.2960, 2.0000, 0.2700, 1.0000),
    (1.3440, 2.0000, 0.2800, 1.0000),
    (1.3920, 2.0000, 0.2900, 1.0000),
    (1.4400, 2.0000, 0.3000, 1.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32, f32, f32, f32); 15] = [
    (
        0.9600, 0.5000, 0.0480, 3.0000, 0.7200, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.1184, 0.5150, 0.0559, 3.0000, 0.8388, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.2768, 0.5300, 0.0638, 3.0000, 0.9576, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.4352, 0.5450, 0.0718, 3.0000, 1.0764, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.5936, 0.5600, 0.0797, 3.0000, 1.1952, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.7520, 0.5750, 0.0876, 3.0000, 1.3140, 7.0000, 0.2000, 50.0000,
    ),
    (
        1.8960, 0.5938, 0.0948, 3.0000, 1.4220, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.0400, 0.6125, 0.1020, 3.0000, 1.5300, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.2200, 0.6312, 0.1110, 3.0000, 1.6650, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.4000, 0.6500, 0.1200, 3.0000, 1.8000, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.5200, 0.6650, 0.1260, 3.0000, 1.8900, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.6400, 0.6800, 0.1320, 3.0000, 1.9800, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.7600, 0.6950, 0.1380, 3.0000, 2.0700, 7.0000, 0.2000, 50.0000,
    ),
    (
        2.8800, 0.7100, 0.1440, 3.0000, 2.1600, 7.0000, 0.2000, 50.0000,
    ),
    (
        3.0000, 0.7250, 0.1500, 3.0000, 2.2500, 7.0000, 0.2000, 50.0000,
    ),
];

const TECH_PARAMS: [(f32, f32); 1] = [(1.5000, 0.5000)];

const BASIC_PARAMS: [(f32, f32, f32); 9] = [
    (0.3000, 0.5000, 0.5000),
    (0.3600, 0.5300, 0.5300),
    (0.4200, 0.5600, 0.5600),
    (0.4800, 0.5900, 0.5900),
    (0.5400, 0.6200, 0.6200),
    (0.6000, 0.6500, 0.6500),
    (0.6600, 0.6800, 0.6800),
    (0.7200, 0.7100, 0.7100),
    (0.7800, 0.7400, 0.7400),
];
