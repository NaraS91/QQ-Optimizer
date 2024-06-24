use super::{
    AdvancedStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Skill),
        vec![ModifierData::new(
            ModifierTarget::Enemy,
            Stat::Advanced(AdvancedStat::TotalDmgReceived(
                AdvancedStat::create_dmg_bonus_flag(0, 0, 0, 0, 1),
            )),
            BuffScaling::Additive,
            |target, buffer, _, team, lc_store, relic_store| {
                let skill_level = (buffer.unique_data.skill_level
                    + if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    }) as usize;
                SKILL_PARAMS[skill_level].1
            },
        )],
        true,
    ))];

    if unit.unique_data.eidolon >= 1 {
        result.push(ModifierOrDOT::Modifier(Modifier::new_with_config(
            (unit.kind, Source::Eidolon(1)),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::CritDamageReceived(
                    AdvancedStat::create_dmg_bonus_flag(0, 0, 0, 0, 1),
                )),
                BuffScaling::Additive,
                |_, _, config, _, _, _| config.unwrap().get_index() as f32 * 0.25,
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(2))),
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (0.7500, 0.2500),
    (0.8250, 0.2750),
    (0.9000, 0.3000),
    (0.9750, 0.3250),
    (1.0500, 0.3500),
    (1.1250, 0.3750),
    (1.2188, 0.4062),
    (1.3125, 0.4375),
    (1.4062, 0.4688),
    (1.5000, 0.5000),
    (1.5750, 0.5250),
    (1.6500, 0.5500),
    (1.7250, 0.5750),
    (1.8000, 0.6000),
    (1.8750, 0.6250),
];

const ULT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.7500, 0.1250, 0.5000, 2.0000),
    (0.8250, 0.1375, 0.5000, 2.0000),
    (0.9000, 0.1500, 0.5000, 2.0000),
    (0.9750, 0.1625, 0.5000, 2.0000),
    (1.0500, 0.1750, 0.5000, 2.0000),
    (1.1250, 0.1875, 0.5000, 2.0000),
    (1.2188, 0.2031, 0.5000, 2.0000),
    (1.3125, 0.2188, 0.5000, 2.0000),
    (1.4062, 0.2344, 0.5000, 2.0000),
    (1.5000, 0.2500, 0.5000, 2.0000),
    (1.5750, 0.2625, 0.5000, 2.0000),
    (1.6500, 0.2750, 0.5000, 2.0000),
    (1.7250, 0.2875, 0.5000, 2.0000),
    (1.8000, 0.3000, 0.5000, 2.0000),
    (1.8750, 0.3125, 0.5000, 2.0000),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (80.0000, 0.7500, 0.5000),
    (80.0000, 0.8250, 0.5000),
    (80.0000, 0.9000, 0.5000),
    (80.0000, 0.9750, 0.5000),
    (80.0000, 1.0500, 0.5000),
    (80.0000, 1.1250, 0.5000),
    (80.0000, 1.2188, 0.5000),
    (80.0000, 1.3125, 0.5000),
    (80.0000, 1.4062, 0.5000),
    (80.0000, 1.5000, 0.5000),
    (80.0000, 1.5750, 0.5000),
    (80.0000, 1.6500, 0.5000),
    (80.0000, 1.7250, 0.5000),
    (80.0000, 1.8000, 0.5000),
    (80.0000, 1.8750, 0.5000),
];

const TECH_PARAMS: [(f32, f32); 1] = [(60.0000, 10000.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
