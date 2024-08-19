use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat,
        UnitKind,
    },
    utils::{flat_value, value_with_buffer},
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Hanya, Source::Skill),
            vec![
                ModifierData::new(
                    ModifierTarget::Ally,
                    Stat::Base(BaseStat::Spd),
                    BuffScaling::Additive,
                    |_, buffer, _, team, light_cones_store, relics_store| {
                        let ult_bonus = if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        };
                        ULT_PARAMS[(buffer.unique_data.ultimate_level + ult_bonus) as usize].2
                            * buffer.get_effective_base_stat(
                                BaseStat::Spd,
                                team,
                                light_cones_store,
                                relics_store,
                            )
                    },
                ),
                ModifierData::new(
                    ModifierTarget::Ally,
                    Stat::Base(BaseStat::Atk),
                    BuffScaling::Multiplicative,
                    value_with_buffer!(|buffer: &Unit| {
                        let ult_bonus = if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        };
                        ULT_PARAMS[(buffer.unique_data.ultimate_level + ult_bonus) as usize].0
                    }),
                ),
            ],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Hanya, Source::Talent),
            vec![ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::TotalDmgReceived(
                    AdvancedStat::create_dmg_bonus_flag(1, 1, 1, 0, 0),
                )),
                BuffScaling::Additive,
                value_with_buffer!(|buffer: &Unit| {
                    let talent_bonus = if buffer.unique_data.eidolon >= 5 {
                        2
                    } else {
                        0
                    };
                    TALENT_PARAMS[(buffer.unique_data.talent_level + talent_bonus) as usize].0
                        + if buffer.unique_data.eidolon >= 6 {
                            0.1
                        } else {
                            0.
                        }
                }),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Hanya, Source::Trace(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                flat_value!(0.1),
            )],
            true,
        )),
    ];

    if unit.unique_data.eidolon >= 2 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Hanya, Source::Eidolon(2)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(0.2),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32); 15] = [
    (1.2000, 2.0000),
    (1.3200, 2.0000),
    (1.4400, 2.0000),
    (1.5600, 2.0000),
    (1.6800, 2.0000),
    (1.8000, 2.0000),
    (1.9500, 2.0000),
    (2.1000, 2.0000),
    (2.2500, 2.0000),
    (2.4000, 2.0000),
    (2.5200, 2.0000),
    (2.6400, 2.0000),
    (2.7600, 2.0000),
    (2.8800, 2.0000),
    (3.0000, 2.0000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (0.3600, 2.0000, 0.1500),
    (0.3840, 2.0000, 0.1550),
    (0.4080, 2.0000, 0.1600),
    (0.4320, 2.0000, 0.1650),
    (0.4560, 2.0000, 0.1700),
    (0.4800, 2.0000, 0.1750),
    (0.5100, 2.0000, 0.1812),
    (0.5400, 2.0000, 0.1875),
    (0.5700, 2.0000, 0.1938),
    (0.6000, 2.0000, 0.2000),
    (0.6240, 2.0000, 0.2050),
    (0.6480, 2.0000, 0.2100),
    (0.6720, 2.0000, 0.2150),
    (0.6960, 2.0000, 0.2200),
    (0.7200, 2.0000, 0.2250),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.1500, 2.0000),
    (0.1650, 2.0000),
    (0.1800, 2.0000),
    (0.1950, 2.0000),
    (0.2100, 2.0000),
    (0.2250, 2.0000),
    (0.2437, 2.0000),
    (0.2625, 2.0000),
    (0.2812, 2.0000),
    (0.3000, 2.0000),
    (0.3150, 2.0000),
    (0.3300, 2.0000),
    (0.3450, 2.0000),
    (0.3600, 2.0000),
    (0.3750, 2.0000),
];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
