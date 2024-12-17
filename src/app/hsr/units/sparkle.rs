use crate::app::hsr::{
    basics::Element,
    units::{BonusDMGFlag, ConfigType, ModifierConfig},
    utils::flat_value,
};

use super::{
    AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT, ModifierTarget,
    Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let skill_data: Vec<ModifierData> = vec![ModifierData::new(
        if unit.unique_data.eidolon >= 6 {
            ModifierTarget::Team
        } else {
            ModifierTarget::Ally
        },
        Stat::Advanced(AdvancedStat::CritDamage),
        BuffScaling::Additive,
        |_, buffer, _, team, light_cones_store, relics_store| {
            let skill_level = buffer.unique_data.skill_level
                + (if buffer.unique_data.eidolon >= 3 {
                    2
                } else {
                    0
                });
            let sparkle_cdmg = buffer.get_effective_advanced_stat(
                AdvancedStat::CritDamage,
                team,
                light_cones_store,
                relics_store,
            );
            let bonus_cdmg = if buffer.unique_data.eidolon >= 6 {
                0.3 * sparkle_cdmg
            } else {
                0.
            };
            SKILL_PARAMS[skill_level as usize].0 * sparkle_cdmg
                + SKILL_PARAMS[skill_level as usize].1
                + bonus_cdmg
        },
    )];

    let mut talent_data: Vec<ModifierData> = vec![ModifierData::new(
        if unit.unique_data.eidolon >= 6 {
            ModifierTarget::Team
        } else {
            ModifierTarget::Ally
        },
        Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
        BuffScaling::Additive,
        |_, buffer, config_op, _, _, _| {
            let talent_level = buffer.unique_data.talent_level as usize
                + (if buffer.unique_data.eidolon >= 5 {
                    2
                } else {
                    0
                }) as usize;
            let ult_level = buffer.unique_data.ultimate_level as usize
                + (if buffer.unique_data.eidolon >= 5 {
                    2
                } else {
                    0
                }) as usize;

            let (stacks, is_ult_active) = config_op.unwrap().get_number_with_bool();

            stacks as f32
                * (TALENT_PARAMS[talent_level].1
                    + if is_ult_active {
                        ULT_PARAMS[ult_level].2
                    } else {
                        0.
                    })
        },
    )];
    if unit.unique_data.eidolon >= 1 {
        talent_data.push(ModifierData::new(
            ModifierTarget::Team,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Multiplicative,
            flat_value!(0.4),
        ))
    };
    if unit.unique_data.eidolon >= 2 {
        talent_data.push(ModifierData::new(
            ModifierTarget::Team,
            Stat::Advanced(AdvancedStat::DefIgnore),
            BuffScaling::Additive,
            |_, _, config_op, _, _, _| {
                let (stacks, _) = config_op.unwrap().get_number_with_bool();
                stacks as f32 * 0.08
            },
        ))
    }
    let trace3_data: Vec<ModifierData> = vec![ModifierData::new(
        ModifierTarget::Team,
        Stat::Base(BaseStat::Atk),
        BuffScaling::Multiplicative,
        |_, _, _, team, _, _| {
            let quantum_allies = team
                .iter()
                .filter(|unit_op| {
                    if let Some(unit) = unit_op {
                        unit.main_element == Element::Quantum
                    } else {
                        false
                    }
                })
                .count()
                - 1; // - sparkle, only allies should count
            0.15 + 0.025 * (quantum_allies * (quantum_allies + 1)) as f32
        },
    )];

    vec![
        ModifierOrDOT::Modifier(Modifier::new((unit.kind, Source::Skill), skill_data, true)),
        ModifierOrDOT::Modifier(Modifier::new_with_config(
            (unit.kind, Source::Talent),
            talent_data,
            Some(ModifierConfig::new(ConfigType::StacksWithFlag(3))),
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Trace(3)),
            trace3_data,
            true,
        )),
    ]
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.1200, 0.2700, 1.0000, 0.5000),
    (0.1320, 0.2880, 1.0000, 0.5000),
    (0.1440, 0.3060, 1.0000, 0.5000),
    (0.1560, 0.3240, 1.0000, 0.5000),
    (0.1680, 0.3420, 1.0000, 0.5000),
    (0.1800, 0.3600, 1.0000, 0.5000),
    (0.1950, 0.3825, 1.0000, 0.5000),
    (0.2100, 0.4050, 1.0000, 0.5000),
    (0.2250, 0.4275, 1.0000, 0.5000),
    (0.2400, 0.4500, 1.0000, 0.5000),
    (0.2520, 0.4680, 1.0000, 0.5000),
    (0.2640, 0.4860, 1.0000, 0.5000),
    (0.2760, 0.5040, 1.0000, 0.5000),
    (0.2880, 0.5220, 1.0000, 0.5000),
    (0.3000, 0.5400, 1.0000, 0.5000),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (2.0000, 4.0000, 0.0600, 2.0000),
    (2.0000, 4.0000, 0.0640, 2.0000),
    (2.0000, 4.0000, 0.0680, 2.0000),
    (2.0000, 4.0000, 0.0720, 2.0000),
    (2.0000, 4.0000, 0.0760, 2.0000),
    (2.0000, 4.0000, 0.0800, 2.0000),
    (2.0000, 4.0000, 0.0850, 2.0000),
    (2.0000, 4.0000, 0.0900, 2.0000),
    (2.0000, 4.0000, 0.0950, 2.0000),
    (2.0000, 4.0000, 0.1000, 2.0000),
    (2.0000, 4.0000, 0.1040, 2.0000),
    (2.0000, 4.0000, 0.1080, 2.0000),
    (2.0000, 4.0000, 0.1120, 2.0000),
    (2.0000, 4.0000, 0.1160, 2.0000),
    (2.0000, 4.0000, 0.1200, 2.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (2.0000, 0.0300, 2.0000, 3.0000),
    (2.0000, 0.0330, 2.0000, 3.0000),
    (2.0000, 0.0360, 2.0000, 3.0000),
    (2.0000, 0.0390, 2.0000, 3.0000),
    (2.0000, 0.0420, 2.0000, 3.0000),
    (2.0000, 0.0450, 2.0000, 3.0000),
    (2.0000, 0.0488, 2.0000, 3.0000),
    (2.0000, 0.0525, 2.0000, 3.0000),
    (2.0000, 0.0563, 2.0000, 3.0000),
    (2.0000, 0.0600, 2.0000, 3.0000),
    (2.0000, 0.0630, 2.0000, 3.0000),
    (2.0000, 0.0660, 2.0000, 3.0000),
    (2.0000, 0.0690, 2.0000, 3.0000),
    (2.0000, 0.0720, 2.0000, 3.0000),
    (2.0000, 0.0750, 2.0000, 3.0000),
];

const TECH_PARAMS: [(f32, f32); 1] = [(3.0000, 20.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
