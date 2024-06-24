use super::{
    utils::{flat_value, value_with_buffer},
    AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![
                ModifierData::new(
                    ModifierTarget::Enemy,
                    Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                    BuffScaling::Additive,
                    |target, buffer, _, team, lc_store, relic_store| {
                        if target.is_weak_to(team[0].unwrap().main_element) {
                            -SKILL_PARAMS[0].3
                        } else {
                            0.
                        }
                    },
                ),
                ModifierData::new(
                    ModifierTarget::Enemy,
                    Stat::Advanced(AdvancedStat::TotalDmgRes),
                    BuffScaling::Additive,
                    value_with_buffer!(|buffer: &Unit| {
                        let skill_index = (buffer.unique_data.skill_level
                            + if buffer.unique_data.eidolon >= 3 {
                                2
                            } else {
                                0
                            }) as usize;
                        -SKILL_PARAMS[skill_index].5
                    }),
                ),
            ],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                value_with_buffer!(|buffer: &Unit| {
                    let ult_index = (buffer.unique_data.ultimate_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize;

                    -ULT_PARAMS[ult_index].2
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Talent),
            vec![
                ModifierData::new(
                    ModifierTarget::Enemy,
                    Stat::Base(BaseStat::Atk),
                    BuffScaling::Multiplicative,
                    value_with_buffer!(|buffer: &Unit| {
                        let talent_index = (buffer.unique_data.talent_level
                            + if buffer.unique_data.eidolon >= 3 {
                                2
                            } else {
                                0
                            }) as usize;

                        -TALENT_PARAMS[talent_index].0
                    }),
                ),
                ModifierData::new(
                    ModifierTarget::Enemy,
                    Stat::Base(BaseStat::Def),
                    BuffScaling::Multiplicative,
                    value_with_buffer!(|buffer: &Unit| {
                        let talent_index = (buffer.unique_data.talent_level
                            + if buffer.unique_data.eidolon >= 3 {
                                2
                            } else {
                                0
                            }) as usize;

                        -TALENT_PARAMS[talent_index].1
                    }),
                ),
                ModifierData::new(
                    ModifierTarget::Enemy,
                    Stat::Base(BaseStat::Spd),
                    BuffScaling::Multiplicative,
                    value_with_buffer!(|buffer: &Unit| {
                        let talent_index = (buffer.unique_data.talent_level
                            + if buffer.unique_data.eidolon >= 3 {
                                2
                            } else {
                                0
                            }) as usize;

                        -TALENT_PARAMS[talent_index].3
                    }),
                ),
            ],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(3)),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::TotalDmgRes),
                BuffScaling::Additive,
                flat_value!(-0.03),
            )],
            true,
        )),
    ];

    if unit.unique_data.eidolon >= 2 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(2)),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Advanced(AdvancedStat::EffectRes),
                BuffScaling::Additive,
                flat_value!(-0.2),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32, f32, f32); 15] = [
    (0.9800, 0.7500, 2.0000, 0.2000, 1.0000, 0.0750, 2.0000),
    (1.0780, 0.7600, 2.0000, 0.2000, 1.0000, 0.0775, 2.0000),
    (1.1760, 0.7700, 2.0000, 0.2000, 1.0000, 0.0800, 2.0000),
    (1.2740, 0.7800, 2.0000, 0.2000, 1.0000, 0.0825, 2.0000),
    (1.3720, 0.7900, 2.0000, 0.2000, 1.0000, 0.0850, 2.0000),
    (1.4700, 0.8000, 2.0000, 0.2000, 1.0000, 0.0875, 2.0000),
    (1.5925, 0.8125, 2.0000, 0.2000, 1.0000, 0.0906, 2.0000),
    (1.7150, 0.8250, 2.0000, 0.2000, 1.0000, 0.0938, 2.0000),
    (1.8375, 0.8375, 2.0000, 0.2000, 1.0000, 0.0969, 2.0000),
    (1.9600, 0.8500, 2.0000, 0.2000, 1.0000, 0.1000, 2.0000),
    (2.0580, 0.8600, 2.0000, 0.2000, 1.0000, 0.1025, 2.0000),
    (2.1560, 0.8700, 2.0000, 0.2000, 1.0000, 0.1050, 2.0000),
    (2.2540, 0.8800, 2.0000, 0.2000, 1.0000, 0.1075, 2.0000),
    (2.3520, 0.8900, 2.0000, 0.2000, 1.0000, 0.1100, 2.0000),
    (2.4500, 0.9000, 2.0000, 0.2000, 1.0000, 0.1125, 2.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (2.2800, 0.8500, 0.3600, 3.0000),
    (2.4320, 0.8650, 0.3690, 3.0000),
    (2.5840, 0.8800, 0.3780, 3.0000),
    (2.7360, 0.8950, 0.3870, 3.0000),
    (2.8880, 0.9100, 0.3960, 3.0000),
    (3.0400, 0.9250, 0.4050, 3.0000),
    (3.2300, 0.9437, 0.4163, 3.0000),
    (3.4200, 0.9625, 0.4275, 3.0000),
    (3.6100, 0.9812, 0.4387, 3.0000),
    (3.8000, 1.0000, 0.4500, 3.0000),
    (3.9520, 1.0150, 0.4590, 3.0000),
    (4.1040, 1.0300, 0.4680, 3.0000),
    (4.2560, 1.0450, 0.4770, 3.0000),
    (4.4080, 1.0600, 0.4860, 3.0000),
    (4.5600, 1.0750, 0.4950, 3.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.0500, 0.0400, 0.0300, 0.6000, 3.0000),
    (0.0550, 0.0440, 0.0330, 0.6120, 3.0000),
    (0.0600, 0.0480, 0.0360, 0.6240, 3.0000),
    (0.0650, 0.0520, 0.0390, 0.6360, 3.0000),
    (0.0700, 0.0560, 0.0420, 0.6480, 3.0000),
    (0.0750, 0.0600, 0.0450, 0.6600, 3.0000),
    (0.0813, 0.0650, 0.0488, 0.6750, 3.0000),
    (0.0875, 0.0700, 0.0525, 0.6900, 3.0000),
    (0.0938, 0.0750, 0.0563, 0.7050, 3.0000),
    (0.1000, 0.0800, 0.0600, 0.7200, 3.0000),
    (0.1050, 0.0840, 0.0630, 0.7320, 3.0000),
    (0.1100, 0.0880, 0.0660, 0.7440, 3.0000),
    (0.1150, 0.0920, 0.0690, 0.7560, 3.0000),
    (0.1200, 0.0960, 0.0720, 0.7680, 3.0000),
    (0.1250, 0.1000, 0.0750, 0.7800, 3.0000),
];

const TECH_PARAMS: [f32; 1] = [0.8000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
