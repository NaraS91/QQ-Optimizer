use super::{
    utils::flat_value, AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Skill),
        vec![ModifierData::new(
            ModifierTarget::Ally,
            Stat::Base(BaseStat::Hp),
            BuffScaling::Additive,
            |_, buffer, _, team, light_cones_store, relics_store| {
                let skill_index = (buffer.unique_data.skill_level
                    + if buffer.unique_data.eidolon >= 2 {
                        2
                    } else {
                        0
                    }) as usize;
                let lynx_hp = buffer.get_effective_base_stat(
                    BaseStat::Hp,
                    team,
                    light_cones_store,
                    relics_store,
                );

                SKILL_PARAMS[skill_index].0 * lynx_hp
                    + SKILL_PARAMS[skill_index].1
                    + if buffer.unique_data.eidolon >= 6 {
                        0.06 * lynx_hp
                    } else {
                        0.
                    }
            },
        )],
        true,
    ))];

    if unit.unique_data.eidolon >= 4 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Ally,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Additive,
                |_, buffer, _, team, light_cones_store, relics_store| {
                    buffer.get_effective_base_stat(
                        BaseStat::Hp,
                        team,
                        light_cones_store,
                        relics_store,
                    ) * 0.03
                },
            )],
            true,
        )))
    }

    if unit.unique_data.eidolon >= 6 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Skill),
            vec![ModifierData::new(
                ModifierTarget::Ally,
                Stat::Advanced(AdvancedStat::EffectRes),
                BuffScaling::Additive,
                flat_value!(0.3),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32, f32); 15] = [
    (0.0500, 50.0000, 2.0000, 0.0800, 80.0000, 5.0000),
    (0.0525, 80.0000, 2.0000, 0.0850, 128.0000, 5.0000),
    (0.0550, 102.5000, 2.0000, 0.0900, 164.0000, 5.0000),
    (0.0575, 125.0000, 2.0000, 0.0950, 200.0000, 5.0000),
    (0.0600, 140.0000, 2.0000, 0.1000, 224.0000, 5.0000),
    (0.0625, 155.0000, 2.0000, 0.1040, 248.0000, 5.0000),
    (0.0656, 166.2500, 2.0000, 0.1080, 266.0000, 5.0000),
    (0.0688, 177.5000, 2.0000, 0.1120, 284.0000, 5.0000),
    (0.0719, 188.7500, 2.0000, 0.1160, 302.0000, 5.0000),
    (0.0750, 200.0000, 2.0000, 0.1200, 320.0000, 5.0000),
    (0.0775, 211.2500, 2.0000, 0.1240, 338.0000, 5.0000),
    (0.0800, 222.5000, 2.0000, 0.1280, 356.0000, 5.0000),
    (0.0825, 233.7500, 2.0000, 0.1320, 374.0000, 5.0000),
    (0.0850, 245.0000, 2.0000, 0.1360, 392.0000, 5.0000),
    (0.0875, 256.2500, 2.0000, 0.1400, 410.0000, 5.0000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (1.0000, 0.0900, 90.0000),
    (1.0000, 0.0956, 144.0000),
    (1.0000, 0.1013, 184.5000),
    (1.0000, 0.1069, 225.0000),
    (1.0000, 0.1125, 252.0000),
    (1.0000, 0.1170, 279.0000),
    (1.0000, 0.1215, 299.2500),
    (1.0000, 0.1260, 319.5000),
    (1.0000, 0.1305, 339.7500),
    (1.0000, 0.1350, 360.0000),
    (1.0000, 0.1395, 380.2500),
    (1.0000, 0.1440, 400.5000),
    (1.0000, 0.1485, 420.7500),
    (1.0000, 0.1530, 441.0000),
    (1.0000, 0.1575, 461.2500),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (2.0000, 0.0240, 24.0000, 0.0300, 30.0000),
    (2.0000, 0.0255, 38.4000, 0.0319, 48.0000),
    (2.0000, 0.0270, 49.2000, 0.0338, 61.5000),
    (2.0000, 0.0285, 60.0000, 0.0356, 75.0000),
    (2.0000, 0.0300, 67.2000, 0.0375, 84.0000),
    (2.0000, 0.0312, 74.4000, 0.0390, 93.0000),
    (2.0000, 0.0324, 79.8000, 0.0405, 99.7500),
    (2.0000, 0.0336, 85.2000, 0.0420, 106.5000),
    (2.0000, 0.0348, 90.6000, 0.0435, 113.2500),
    (2.0000, 0.0360, 96.0000, 0.0450, 120.0000),
    (2.0000, 0.0372, 101.4000, 0.0465, 126.7500),
    (2.0000, 0.0384, 106.8000, 0.0480, 133.5000),
    (2.0000, 0.0396, 112.2000, 0.0495, 140.2500),
    (2.0000, 0.0408, 117.6000, 0.0510, 147.0000),
    (2.0000, 0.0420, 123.0000, 0.0525, 153.7500),
];

const TECH_PARAMS: [f32; 1] = [2.0000];

const BASIC_PARAMS: [f32; 9] = [
    0.2500, 0.3000, 0.3500, 0.4000, 0.4500, 0.5000, 0.5500, 0.6000, 0.6500,
];
