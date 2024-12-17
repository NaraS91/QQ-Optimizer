use crate::app::hsr::units::{BonusDMGFlag, Source};

use super::{
    AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT, ModifierTarget,
    Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut skill_data: Vec<ModifierData> = vec![ModifierData::new(
        ModifierTarget::Ally,
        Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
        BuffScaling::Additive,
        |_, buffer, _, _, _, _| {
            let skill_level = buffer.unique_data.skill_level
                + (if buffer.unique_data.eidolon >= 5 {
                    2
                } else {
                    0
                });
            SKILL_PARAMS[skill_level as usize].0
        },
    )];

    if unit.unique_data.eidolon >= 2 {
        skill_data.push(ModifierData::new(
            ModifierTarget::Ally,
            Stat::Base(BaseStat::Spd),
            BuffScaling::Multiplicative,
            |_, _, _, _, _, _| 0.3,
        ))
    }

    let ultimate_data: Vec<ModifierData> = vec![
        ModifierData::new(
            ModifierTarget::Team,
            Stat::Advanced(AdvancedStat::CritDamage),
            BuffScaling::Additive,
            |_, buffer, _, team, light_cones_store, relics_store| {
                let ult_level = buffer.unique_data.ultimate_level as usize
                    + (if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    }) as usize;
                ULT_PARAMS[ult_level].2
                    + ULT_PARAMS[ult_level].1
                        * buffer.get_effective_advanced_stat(
                            AdvancedStat::CritDamage,
                            team,
                            light_cones_store,
                            relics_store,
                        )
            },
        ),
        ModifierData::new(
            ModifierTarget::Team,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Multiplicative,
            |_, buffer, _, _, _, _| {
                let ult_level = buffer.unique_data.ultimate_level as usize
                    + (if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    }) as usize;
                ULT_PARAMS[ult_level].0
            },
        ),
    ];

    let trace2_data: Vec<ModifierData> = vec![ModifierData::new(
        ModifierTarget::Team,
        Stat::Base(BaseStat::Def),
        BuffScaling::Multiplicative,
        |_, _, _, _, _, _| 0.2,
    )];

    let trace3_data: Vec<ModifierData> = vec![ModifierData::new(
        ModifierTarget::Team,
        Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
        BuffScaling::Additive,
        |_, _, _, _, _, _| 0.1,
    )];
    let technique_data: Vec<ModifierData> = vec![ModifierData::new(
        ModifierTarget::Team,
        Stat::Base(BaseStat::Atk),
        BuffScaling::Multiplicative,
        |_, _, _, _, _, _| 0.15,
    )];

    vec![
        ModifierOrDOT::Modifier(Modifier::new((unit.kind, Source::Skill), skill_data, true)),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            ultimate_data,
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(2)),
            trace2_data,
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(3)),
            trace3_data,
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Technique),
            technique_data,
            true,
        )),
    ]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.3300, 0.0000, 1.0000, 1.0000),
    (0.3630, 0.0000, 1.0000, 1.0000),
    (0.3960, 0.0000, 1.0000, 1.0000),
    (0.4290, 0.0000, 1.0000, 1.0000),
    (0.4620, 0.0000, 1.0000, 1.0000),
    (0.4950, 0.0000, 1.0000, 1.0000),
    (0.5363, 0.0000, 1.0000, 1.0000),
    (0.5775, 0.0000, 1.0000, 1.0000),
    (0.6188, 0.0000, 1.0000, 1.0000),
    (0.6600, 0.0000, 1.0000, 1.0000),
    (0.6930, 0.0000, 1.0000, 1.0000),
    (0.7260, 0.0000, 1.0000, 1.0000),
    (0.7590, 0.0000, 1.0000, 1.0000),
    (0.7920, 0.0000, 1.0000, 1.0000),
    (0.8250, 0.0000, 1.0000, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.3300, 0.1200, 0.1200, 2.0000),
    (0.3520, 0.1240, 0.1280, 2.0000),
    (0.3740, 0.1280, 0.1360, 2.0000),
    (0.3960, 0.1320, 0.1440, 2.0000),
    (0.4180, 0.1360, 0.1520, 2.0000),
    (0.4400, 0.1400, 0.1600, 2.0000),
    (0.4675, 0.1450, 0.1700, 2.0000),
    (0.4950, 0.1500, 0.1800, 2.0000),
    (0.5225, 0.1550, 0.1900, 2.0000),
    (0.5500, 0.1600, 0.2000, 2.0000),
    (0.5720, 0.1640, 0.2080, 2.0000),
    (0.5940, 0.1680, 0.2160, 2.0000),
    (0.6160, 0.1720, 0.2240, 2.0000),
    (0.6380, 0.1760, 0.2320, 2.0000),
    (0.6600, 0.1800, 0.2400, 2.0000),
];

const TALENT_PARAMS: [f32; 15] = [
    0.1500, 0.1650, 0.1800, 0.1950, 0.2100, 0.2250, 0.2437, 0.2625, 0.2812, 0.3000, 0.3150, 0.3300,
    0.3450, 0.3600, 0.3750,
];

const TECH_PARAMS: [(f32, f32); 1] = [(0.1500, 2.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
