use super::{
    utils::{flat_value, value_with_buffer},
    AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                value_with_buffer!(|buffer: &Unit| {
                    let skill_index = (buffer.unique_data.skill_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize;

                    SKILL_PARAMS[skill_index].0
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Talent),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                value_with_buffer!(|buffer: &Unit| {
                    let talent_index = (buffer.unique_data.talent_level
                        + if buffer.unique_data.eidolon >= 2 {
                            2
                        } else {
                            0
                        }) as usize;

                    SKILL_PARAMS[talent_index].0
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                flat_value!(0.2),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                |_, buffer, _, team, light_cones_store, relics_store| {
                    let rm_be = buffer.get_effective_advanced_stat(
                        AdvancedStat::BreakEffect,
                        team,
                        light_cones_store,
                        relics_store,
                    );
                    f32::max(0., ((rm_be - 1.2) * 6.).min(0.36))
                },
            )],
            true,
        )),
    ];

    let mut ult_vec = vec![ModifierData::new(
        ModifierTarget::Allies,
        Stat::Advanced(AdvancedStat::TotalResPen),
        BuffScaling::Additive,
        value_with_buffer!(|buffer: &Unit| {
            let ult_index = (buffer.unique_data.ultimate_level
                + if buffer.unique_data.eidolon >= 2 {
                    2
                } else {
                    0
                }) as usize;

            ULT_PARAMS[ult_index].0
        }),
    )];

    if unit.unique_data.eidolon >= 1 {
        ult_vec.push(ModifierData::new(
            ModifierTarget::Allies,
            Stat::Advanced(AdvancedStat::DefIgnore),
            BuffScaling::Additive,
            flat_value!(0.2),
        ))
    }

    result.push(ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Ultimate),
        ult_vec,
        true,
    )));

    if unit.unique_data.eidolon >= 2 {
        //TODO: only allow if enemy is weakness broken
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(2)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                flat_value!(0.4),
            )],
            true,
        )))
    }

    if unit.unique_data.eidolon >= 4 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(4)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                flat_value!(1.),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (0.1600, 0.5000, 3.0000),
    (0.1760, 0.5000, 3.0000),
    (0.1920, 0.5000, 3.0000),
    (0.2080, 0.5000, 3.0000),
    (0.2240, 0.5000, 3.0000),
    (0.2400, 0.5000, 3.0000),
    (0.2600, 0.5000, 3.0000),
    (0.2800, 0.5000, 3.0000),
    (0.3000, 0.5000, 3.0000),
    (0.3200, 0.5000, 3.0000),
    (0.3360, 0.5000, 3.0000),
    (0.3520, 0.5000, 3.0000),
    (0.3680, 0.5000, 3.0000),
    (0.3840, 0.5000, 3.0000),
    (0.4000, 0.5000, 3.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.1500, 2.0000, 0.2000, 0.1000, 0.3000),
    (0.1600, 2.0000, 0.2000, 0.1000, 0.3200),
    (0.1700, 2.0000, 0.2000, 0.1000, 0.3400),
    (0.1800, 2.0000, 0.2000, 0.1000, 0.3600),
    (0.1900, 2.0000, 0.2000, 0.1000, 0.3800),
    (0.2000, 2.0000, 0.2000, 0.1000, 0.4000),
    (0.2125, 2.0000, 0.2000, 0.1000, 0.4250),
    (0.2250, 2.0000, 0.2000, 0.1000, 0.4500),
    (0.2375, 2.0000, 0.2000, 0.1000, 0.4750),
    (0.2500, 2.0000, 0.2000, 0.1000, 0.5000),
    (0.2600, 2.0000, 0.2000, 0.1000, 0.5200),
    (0.2700, 2.0000, 0.2000, 0.1000, 0.5400),
    (0.2800, 2.0000, 0.2000, 0.1000, 0.5600),
    (0.2900, 2.0000, 0.2000, 0.1000, 0.5800),
    (0.3000, 2.0000, 0.2000, 0.1000, 0.6000),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.0800, 0.6000),
    (0.0820, 0.6600),
    (0.0840, 0.7200),
    (0.0860, 0.7800),
    (0.0880, 0.8400),
    (0.0900, 0.9000),
    (0.0925, 0.9750),
    (0.0950, 1.0500),
    (0.0975, 1.1250),
    (0.1000, 1.2000),
    (0.1020, 1.2600),
    (0.1040, 1.3200),
    (0.1060, 1.3800),
    (0.1080, 1.4400),
    (0.1100, 1.5000),
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(1.0000, 1.0000, 1.0000, 20.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
