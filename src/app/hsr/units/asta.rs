use crate::app::hsr::{
    basics::Element,
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, UnitKind,
    },
    utils::{flat_value, value_with_buffer},
};

use super::{BaseStat, ConfigType, ModifierConfig, ModifierOrDOT, Unit};

pub fn modifiers(_unit: &Unit) -> Vec<ModifierOrDOT> {
    //TODO: implement astas auto burn

    vec![
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Asta, Source::Trace(2)),
            vec![ModifierData::new(
                ModifierTarget::Team,
                Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)),
                BuffScaling::Additive,
                flat_value!(0.18),
            )],
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new_with_config(
            (UnitKind::Asta, Source::Talent),
            vec![ModifierData::new(
                ModifierTarget::Team,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |_, buffer, config_data, _, _, _| {
                    let talent_bonus = if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    };
                    config_data
                        .and_then(|config| Some(config.get_index() as f32))
                        .unwrap_or(0.)
                        * TALENT_PARAMS[(buffer.unique_data.talent_level + talent_bonus) as usize].0
                },
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(5))),
            true,
        )),
        ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Asta, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Team,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Additive,
                value_with_buffer!(
                    |buffer: &Unit| ULT_PARAMS[(buffer.unique_data.ultimate_level
                        + if buffer.unique_data.eidolon >= 5 {
                            2
                        } else {
                            0
                        }) as usize]
                        .0
                ),
            )],
            true,
        )),
    ]
}

const SKILL_PARAMS: [f32; 15] = [
    0.2500, 0.2750, 0.3000, 0.3250, 0.3500, 0.3750, 0.4062, 0.4375, 0.4688, 0.5000, 0.5250, 0.5500,
    0.5750, 0.6000, 0.6250,
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (36.0000, 2.0000),
    (37.4000, 2.0000),
    (38.8000, 2.0000),
    (40.2000, 2.0000),
    (41.6000, 2.0000),
    (43.0000, 2.0000),
    (44.7500, 2.0000),
    (46.5000, 2.0000),
    (48.2500, 2.0000),
    (50.0000, 2.0000),
    (51.4000, 2.0000),
    (52.8000, 2.0000),
    (54.2000, 2.0000),
    (55.6000, 2.0000),
    (57.0000, 2.0000),
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (0.0700, 5.0000, 3.0000),
    (0.0770, 5.0000, 3.0000),
    (0.0840, 5.0000, 3.0000),
    (0.0910, 5.0000, 3.0000),
    (0.0980, 5.0000, 3.0000),
    (0.1050, 5.0000, 3.0000),
    (0.1138, 5.0000, 3.0000),
    (0.1225, 5.0000, 3.0000),
    (0.1313, 5.0000, 3.0000),
    (0.1400, 5.0000, 3.0000),
    (0.1470, 5.0000, 3.0000),
    (0.1540, 5.0000, 3.0000),
    (0.1610, 5.0000, 3.0000),
    (0.1680, 5.0000, 3.0000),
    (0.1750, 5.0000, 3.0000),
];

const TECH_PARAMS: [f32; 1] = [0.5000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
