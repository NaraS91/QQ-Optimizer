use crate::app::hsr::units::{
    utils::flat_value, BaseStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData,
    ModifierTarget, Source, Stat, UnitKind,
};

use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![ModifierOrDOT::Modifier(Modifier::new(
        (UnitKind::Herta, Source::Technique),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Multiplicative,
            flat_value!(0.4),
        )],
        true,
    ))];
    if unit.unique_data.eidolon >= 6 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (UnitKind::Herta, Source::Eidolon(6)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                flat_value!(0.25),
            )],
            true,
        )));
    }

    if unit.unique_data.eidolon >= 2 {
        result.push(ModifierOrDOT::Modifier(Modifier::new_with_config(
            (UnitKind::Herta, Source::Eidolon(2)),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |_, _, config_op, _, _, _| {
                    config_op
                        .and_then(|config| Some(config.get_index() as f32))
                        .unwrap_or(0.)
                        * 0.03
                },
            )],
            Some(ModifierConfig::new(ConfigType::Stacks(5))),
            true,
        )));
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (0.5000, 0.5000, 0.2000),
    (0.5500, 0.5000, 0.2000),
    (0.6000, 0.5000, 0.2000),
    (0.6500, 0.5000, 0.2000),
    (0.7000, 0.5000, 0.2000),
    (0.7500, 0.5000, 0.2000),
    (0.8125, 0.5000, 0.2000),
    (0.8750, 0.5000, 0.2000),
    (0.9375, 0.5000, 0.2000),
    (1.0000, 0.5000, 0.2000),
    (1.0500, 0.5000, 0.2000),
    (1.1000, 0.5000, 0.2000),
    (1.1500, 0.5000, 0.2000),
    (1.2000, 0.5000, 0.2000),
    (1.2500, 0.5000, 0.2000),
];

const ULT_PARAMS: [f32; 15] = [
    1.2000, 1.2800, 1.3600, 1.4400, 1.5200, 1.6000, 1.7000, 1.8000, 1.9000, 2.0000, 2.0800, 2.1600,
    2.2400, 2.3200, 2.4000,
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (0.5000, 0.2500),
    (0.5000, 0.2650),
    (0.5000, 0.2800),
    (0.5000, 0.2950),
    (0.5000, 0.3100),
    (0.5000, 0.3250),
    (0.5000, 0.3438),
    (0.5000, 0.3625),
    (0.5000, 0.3812),
    (0.5000, 0.4000),
    (0.5000, 0.4150),
    (0.5000, 0.4300),
    (0.5000, 0.4450),
    (0.5000, 0.4600),
    (0.5000, 0.4750),
];

const TECH_PARAMS: [(f32, f32); 1] = [(0.4000, 3.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
