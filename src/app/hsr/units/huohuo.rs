use crate::app::hsr::units::utils::flat_value;

use super::{
    utils::value_with_buffer, AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier,
    ModifierData, ModifierOrDOT, ModifierTarget, Source, Stat, Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Ultimate),
        vec![ModifierData::new(
            ModifierTarget::Allies,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Multiplicative,
            value_with_buffer!(|buffer: &Unit| {
                let ultimate_bonus = if buffer.unique_data.eidolon >= 3 {
                    2
                } else {
                    0
                };
                ULT_PARAMS[(buffer.unique_data.talent_level + ultimate_bonus) as usize].1
            }),
        )],
        true,
    ))];

    if unit.unique_data.eidolon >= 1 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(1)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Spd),
                BuffScaling::Multiplicative,
                flat_value!(0.12),
            )],
            true,
        )));
    }

    if unit.unique_data.eidolon >= 6 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Eidolon(6)),
            vec![ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(BonusDMGFlag::MAX)),
                BuffScaling::Additive,
                flat_value!(0.5),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.1400, 140.0000, 0.1120, 112.0000, 1.0000),
    (0.1487, 224.0000, 0.1190, 179.2000, 1.0000),
    (0.1575, 287.0000, 0.1260, 229.6000, 1.0000),
    (0.1663, 350.0000, 0.1330, 280.0000, 1.0000),
    (0.1750, 392.0000, 0.1400, 313.6000, 1.0000),
    (0.1820, 434.0000, 0.1456, 347.2000, 1.0000),
    (0.1890, 465.5000, 0.1512, 372.4000, 1.0000),
    (0.1960, 497.0000, 0.1568, 397.6000, 1.0000),
    (0.2030, 528.5000, 0.1624, 422.8000, 1.0000),
    (0.2100, 560.0000, 0.1680, 448.0000, 1.0000),
    (0.2170, 591.5000, 0.1736, 473.2000, 1.0000),
    (0.2240, 623.0000, 0.1792, 498.4000, 1.0000),
    (0.2310, 654.5000, 0.1848, 523.6000, 1.0000),
    (0.2380, 686.0000, 0.1904, 548.8000, 1.0000),
    (0.2450, 717.5000, 0.1960, 574.0000, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32); 15] = [
    (0.1500, 0.2400, 2.0000),
    (0.1550, 0.2560, 2.0000),
    (0.1600, 0.2720, 2.0000),
    (0.1650, 0.2880, 2.0000),
    (0.1700, 0.3040, 2.0000),
    (0.1750, 0.3200, 2.0000),
    (0.1812, 0.3400, 2.0000),
    (0.1875, 0.3600, 2.0000),
    (0.1938, 0.3800, 2.0000),
    (0.2000, 0.4000, 2.0000),
    (0.2050, 0.4160, 2.0000),
    (0.2100, 0.4320, 2.0000),
    (0.2150, 0.4480, 2.0000),
    (0.2200, 0.4640, 2.0000),
    (0.2250, 0.4800, 2.0000),
];

const TALENT_PARAMS: [(f32, f32, f32, f32, f32, f32, f32); 15] = [
    (2.0000, 1.0000, 0.0300, 0.0000, 30.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0319, 0.0000, 48.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0338, 0.0000, 61.5000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0356, 0.0000, 75.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0375, 0.0000, 84.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0390, 0.0000, 93.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0405, 0.0000, 99.7500, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0420, 0.0000, 106.5000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0435, 0.0000, 113.2500, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0450, 0.0000, 120.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0465, 0.0000, 126.7500, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0480, 0.0000, 133.5000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0495, 0.0000, 140.2500, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0510, 0.0000, 147.0000, 0.5000, 6.0000),
    (2.0000, 1.0000, 0.0525, 0.0000, 153.7500, 0.5000, 6.0000),
];

const TECH_PARAMS: [(f32, f32, f32, f32); 1] = [(1.0000, 0.2500, 2.0000, 10.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.2500, 0.3000, 0.3500, 0.4000, 0.4500, 0.5000, 0.5500, 0.6000, 0.6500,
];
