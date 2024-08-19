use crate::app::hsr::{
    basics::Element,
    utils::{flat_value, value_with_buffer},
};

use super::{
    AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT, ModifierTarget,
    Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                value_with_buffer!(|buffer: &Unit| {
                    let ult_index = (buffer.unique_data.ultimate_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize;

                    -ULT_PARAMS[ult_index].1
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Technique),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                flat_value!(-0.2),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(2)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::EffectHitRate),
                BuffScaling::Additive,
                flat_value!(0.1),
            )],
            true,
        )),
    ];

    if unit.unique_data.eidolon >= 4 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::ElemDmgRes(Element::Ice)),
                BuffScaling::Additive,
                flat_value!(-0.12),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (1.0500, 1.0000),
    (1.1550, 1.0000),
    (1.2600, 1.0000),
    (1.3650, 1.0000),
    (1.4700, 1.0000),
    (1.5750, 1.0000),
    (1.7063, 1.0000),
    (1.8375, 1.0000),
    (1.9688, 1.0000),
    (2.1000, 1.0000),
    (2.2050, 1.0000),
    (2.3100, 1.0000),
    (2.4150, 1.0000),
    (2.5200, 1.0000),
    (2.6250, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (1.0000, 0.3000, 2.0000, 0.6000),
    (1.0000, 0.3100, 2.0000, 0.6400),
    (1.0000, 0.3200, 2.0000, 0.6800),
    (1.0000, 0.3300, 2.0000, 0.7200),
    (1.0000, 0.3400, 2.0000, 0.7600),
    (1.0000, 0.3500, 2.0000, 0.8000),
    (1.0000, 0.3625, 2.0000, 0.8500),
    (1.0000, 0.3750, 2.0000, 0.9000),
    (1.0000, 0.3875, 2.0000, 0.9500),
    (1.0000, 0.4000, 2.0000, 1.0000),
    (1.0000, 0.4100, 2.0000, 1.0400),
    (1.0000, 0.4200, 2.0000, 1.0800),
    (1.0000, 0.4300, 2.0000, 1.1200),
    (1.0000, 0.4400, 2.0000, 1.1600),
    (1.0000, 0.4500, 2.0000, 1.2000),
];

const TALENT_PARAMS: [f32; 15] = [
    5.0000, 5.5000, 6.0000, 6.5000, 7.0000, 7.5000, 8.1250, 8.7500, 9.3750, 10.0000, 10.5000,
    11.0000, 11.5000, 12.0000, 12.5000,
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(1.0000, 0.2000, 2.0000, 0.8000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
