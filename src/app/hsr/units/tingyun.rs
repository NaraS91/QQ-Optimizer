use crate::app::hsr::{
    units::{Modifier, ModifierData, ModifierTarget},
    utils::flat_value,
};

use super::{AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, ModifierOrDOT, Source, Stat, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Ally,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Additive,
                |target, buffer, _, team, lc_store, relic_store| {
                    let skill_level = (buffer.unique_data.skill_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize;
                    let target_atk = target.base_stats(lc_store)[BaseStat::Atk as usize];
                    let ting_atk =
                        buffer.get_effective_base_stat(BaseStat::Atk, team, lc_store, relic_store);
                    (target_atk * SKILL_PARAMS[skill_level].1)
                        .min(ting_atk * SKILL_PARAMS[skill_level].3)
                },
            )],
            false,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(1)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(0.2),
            )],
            true,
        )),
    ];

    let mut ult_data = vec![ModifierData::new(
        ModifierTarget::Ally,
        Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
        BuffScaling::Additive,
        |target, buffer, _, team, lc_store, relic_store| {
            let ult_level = (buffer.unique_data.ultimate_level
                + if buffer.unique_data.eidolon >= 3 {
                    2
                } else {
                    0
                }) as usize;

            ULT_PARAMS[ult_level].2
                + if buffer.unique_data.eidolon >= 4 {
                    0.2
                } else {
                    0.
                }
        },
    )];

    if unit.unique_data.eidolon >= 1 {
        ult_data.push(ModifierData::new(
            ModifierTarget::Ally,
            Stat::Base(BaseStat::Spd),
            BuffScaling::Multiplicative,
            flat_value!(0.2),
        ))
    }

    result.push(ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Skill),
        ult_data,
        true,
    )));

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.2000, 0.2500, 3.0000, 0.1500),
    (0.2200, 0.2750, 3.0000, 0.1600),
    (0.2400, 0.3000, 3.0000, 0.1700),
    (0.2600, 0.3250, 3.0000, 0.1800),
    (0.2800, 0.3500, 3.0000, 0.1900),
    (0.3000, 0.3750, 3.0000, 0.2000),
    (0.3250, 0.4062, 3.0000, 0.2125),
    (0.3500, 0.4375, 3.0000, 0.2250),
    (0.3750, 0.4688, 3.0000, 0.2375),
    (0.4000, 0.5000, 3.0000, 0.2500),
    (0.4200, 0.5250, 3.0000, 0.2600),
    (0.4400, 0.5500, 3.0000, 0.2700),
    (0.4600, 0.5750, 3.0000, 0.2800),
    (0.4800, 0.6000, 3.0000, 0.2900),
    (0.5000, 0.6250, 3.0000, 0.3000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (50.0000, 2.0000, 0.2000),
    (50.0000, 2.0000, 0.2300),
    (50.0000, 2.0000, 0.2600),
    (50.0000, 2.0000, 0.2900),
    (50.0000, 2.0000, 0.3200),
    (50.0000, 2.0000, 0.3500),
    (50.0000, 2.0000, 0.3875),
    (50.0000, 2.0000, 0.4250),
    (50.0000, 2.0000, 0.4625),
    (50.0000, 2.0000, 0.5000),
    (50.0000, 2.0000, 0.5300),
    (50.0000, 2.0000, 0.5600),
    (50.0000, 2.0000, 0.5900),
    (50.0000, 2.0000, 0.6200),
    (50.0000, 2.0000, 0.6500),
];

const TALENT_PARAMS: [f32; 15] = [
    0.3000, 0.3300, 0.3600, 0.3900, 0.4200, 0.4500, 0.4875, 0.5250, 0.5625, 0.6000, 0.6300, 0.6600,
    0.6900, 0.7200, 0.7500,
];

const TECH_PARAMS: [f32; 1] = [50.0000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
