use crate::app::hsr::utils::flat_value;

use super::{
    AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierOrDOT,
    ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![];

    if unit.unique_data.eidolon >= 1 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                flat_value!(0.2),
            )],
            true,
        )))
    }

    if unit.unique_data.eidolon >= 4 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(4)),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                flat_value!(-0.12),
            )],
            true,
        )))
    }

    if unit.unique_data.eidolon >= 6 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(6)),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Advanced(AdvancedStat::TotalResPen),
                BuffScaling::Additive,
                flat_value!(-0.2),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32); 15] = [
    (0.4000, 200.0000, 0.5000, 2.0000),
    (0.4250, 320.0000, 0.5000, 2.0000),
    (0.4500, 410.0000, 0.5000, 2.0000),
    (0.4750, 500.0000, 0.5000, 2.0000),
    (0.5000, 560.0000, 0.5000, 2.0000),
    (0.5200, 620.0000, 0.5000, 2.0000),
    (0.5400, 665.0000, 0.5000, 2.0000),
    (0.5600, 710.0000, 0.5000, 2.0000),
    (0.5800, 755.0000, 0.5000, 2.0000),
    (0.6000, 800.0000, 0.5000, 2.0000),
    (0.6200, 845.0000, 0.5000, 2.0000),
    (0.6400, 890.0000, 0.5000, 2.0000),
    (0.6600, 935.0000, 0.5000, 2.0000),
    (0.6800, 980.0000, 0.5000, 2.0000),
    (0.7000, 1025.0000, 0.5000, 2.0000),
];

const ULT_PARAMS: [(f32, f32); 15] = [
    (1.2000, 1.0000),
    (1.2800, 1.0000),
    (1.3600, 1.0000),
    (1.4400, 1.0000),
    (1.5200, 1.0000),
    (1.6000, 1.0000),
    (1.7000, 1.0000),
    (1.8000, 1.0000),
    (1.9000, 1.0000),
    (2.0000, 1.0000),
    (2.0800, 1.0000),
    (2.1600, 1.0000),
    (2.2400, 1.0000),
    (2.3200, 1.0000),
    (2.4000, 1.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32); 15] = [
    (2.0000, 0.1200, 2.0000, 60.0000),
    (2.0000, 0.1275, 2.0000, 96.0000),
    (2.0000, 0.1350, 2.0000, 123.0000),
    (2.0000, 0.1425, 2.0000, 150.0000),
    (2.0000, 0.1500, 2.0000, 168.0000),
    (2.0000, 0.1560, 2.0000, 186.0000),
    (2.0000, 0.1620, 2.0000, 199.5000),
    (2.0000, 0.1680, 2.0000, 213.0000),
    (2.0000, 0.1740, 2.0000, 226.5000),
    (2.0000, 0.1800, 2.0000, 240.0000),
    (2.0000, 0.1860, 2.0000, 253.5000),
    (2.0000, 0.1920, 2.0000, 267.0000),
    (2.0000, 0.1980, 2.0000, 280.5000),
    (2.0000, 0.2040, 2.0000, 294.0000),
    (2.0000, 0.2100, 2.0000, 307.5000),
];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
