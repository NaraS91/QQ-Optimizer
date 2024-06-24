use crate::app::hsr::{
    basics::Element,
    units::{
        utils::{flat_value, value_with_buffer},
        AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat,
    },
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                value_with_buffer!(|buffer: &Unit| {
                    let skill_level = (buffer.unique_data.skill_level
                        + if buffer.unique_data.eidolon >= 3 {
                            2
                        } else {
                            0
                        }) as usize;
                    SKILL_PARAMS[skill_level].1
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            vec![
                ModifierData::new(
                    ModifierTarget::Allies,
                    Stat::Advanced(AdvancedStat::CritRate),
                    BuffScaling::Additive,
                    value_with_buffer!(|buffer: &Unit| {
                        let ult_level = (buffer.unique_data.ultimate_level
                            + if buffer.unique_data.eidolon >= 5 {
                                2
                            } else {
                                0
                            }) as usize;
                        ULT_PARAMS[ult_level].1
                    }),
                ),
                ModifierData::new(
                    ModifierTarget::Allies,
                    Stat::Advanced(AdvancedStat::CritDamage),
                    BuffScaling::Additive,
                    value_with_buffer!(|buffer: &Unit| {
                        let ult_level = (buffer.unique_data.ultimate_level
                            + if buffer.unique_data.eidolon >= 5 {
                                2
                            } else {
                                0
                            }) as usize;
                        ULT_PARAMS[ult_level].2
                    }),
                ),
            ],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(2)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)),
                BuffScaling::Additive,
                flat_value!(0.12),
            )],
            true,
        )),
    ];
    if unit.unique_data.eidolon >= 1 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(0.1),
            )],
            true,
        )))
    }
    result
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (2.0000, 0.4000),
    (2.0000, 0.4400),
    (2.0000, 0.4800),
    (2.0000, 0.5200),
    (2.0000, 0.5600),
    (2.0000, 0.6000),
    (2.0000, 0.6500),
    (2.0000, 0.7000),
    (2.0000, 0.7500),
    (2.0000, 0.8000),
    (2.0000, 0.8400),
    (2.0000, 0.8800),
    (2.0000, 0.9200),
    (2.0000, 0.9600),
    (2.0000, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (2.2800, 0.2100, 0.3900),
    (2.4320, 0.2170, 0.4160),
    (2.5840, 0.2240, 0.4420),
    (2.7360, 0.2310, 0.4680),
    (2.8880, 0.2380, 0.4940),
    (3.0400, 0.2450, 0.5200),
    (3.2300, 0.2537, 0.5525),
    (3.4200, 0.2625, 0.5850),
    (3.6100, 0.2712, 0.6175),
    (3.8000, 0.2800, 0.6500),
    (3.9520, 0.2870, 0.6760),
    (4.1040, 0.2940, 0.7020),
    (4.2560, 0.3010, 0.7280),
    (4.4080, 0.3080, 0.7540),
    (4.5600, 0.3150, 0.7800),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (0.4000, 1.0000, 1.0000),
    (0.4400, 1.0000, 1.0000),
    (0.4800, 1.0000, 1.0000),
    (0.5200, 1.0000, 1.0000),
    (0.5600, 1.0000, 1.0000),
    (0.6000, 1.0000, 1.0000),
    (0.6500, 1.0000, 1.0000),
    (0.7000, 1.0000, 1.0000),
    (0.7500, 1.0000, 1.0000),
    (0.8000, 1.0000, 1.0000),
    (0.8400, 1.0000, 1.0000),
    (0.8800, 1.0000, 1.0000),
    (0.9200, 1.0000, 1.0000),
    (0.9600, 1.0000, 1.0000),
    (1.0000, 1.0000, 1.0000),
];

const TECH_PARAMS: [(f32, f32, f32); 1] = [(20.0000, 0.3500, 2.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
