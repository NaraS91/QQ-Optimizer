use crate::app::hsr::utils::flat_value;

use super::{
    BaseStat, BuffScaling, Modifier, ModifierData, ModifierOrDOT, ModifierTarget, Source, Stat,
    Unit,
};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut result = vec![];

    if unit.unique_data.eidolon >= 2 {
        result.push(ModifierOrDOT::Modifier(Modifier::new(
            (unit.kind, Source::Ultimate),
            vec![ModifierData::new(
                ModifierTarget::Enemies,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                flat_value!(-0.16),
            )],
            true,
        )))
    }

    result
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (1.0000, 0.4000, 1.0000),
    (1.1000, 0.4400, 1.0000),
    (1.2000, 0.4800, 1.0000),
    (1.3000, 0.5200, 1.0000),
    (1.4000, 0.5600, 1.0000),
    (1.5000, 0.6000, 1.0000),
    (1.6250, 0.6500, 1.0000),
    (1.7500, 0.7000, 1.0000),
    (1.8750, 0.7500, 1.0000),
    (2.0000, 0.8000, 1.0000),
    (2.1000, 0.8400, 1.0000),
    (2.2000, 0.8800, 1.0000),
    (2.3000, 0.9200, 1.0000),
    (2.4000, 0.9600, 1.0000),
    (2.5000, 1.0000, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (3.0000, 0.3600, 0.1200, 0.1800, 10.0000),
    (3.0000, 0.3840, 0.1280, 0.1920, 10.0000),
    (3.0000, 0.4080, 0.1360, 0.2040, 10.0000),
    (3.0000, 0.4320, 0.1440, 0.2160, 10.0000),
    (3.0000, 0.4560, 0.1520, 0.2280, 10.0000),
    (3.0000, 0.4800, 0.1600, 0.2400, 10.0000),
    (3.0000, 0.5100, 0.1700, 0.2550, 10.0000),
    (3.0000, 0.5400, 0.1800, 0.2700, 10.0000),
    (3.0000, 0.5700, 0.1900, 0.2850, 10.0000),
    (3.0000, 0.6000, 0.2000, 0.3000, 10.0000),
    (3.0000, 0.6240, 0.2080, 0.3120, 10.0000),
    (3.0000, 0.6480, 0.2160, 0.3240, 10.0000),
    (3.0000, 0.6720, 0.2240, 0.3360, 10.0000),
    (3.0000, 0.6960, 0.2320, 0.3480, 10.0000),
    (3.0000, 0.7200, 0.2400, 0.3600, 10.0000),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (1.0000, 1.0000),
    (1.1000, 1.0000),
    (1.2000, 1.0000),
    (1.3000, 1.0000),
    (1.4000, 1.0000),
    (1.5000, 1.0000),
    (1.6250, 1.0000),
    (1.7500, 1.0000),
    (1.8750, 1.0000),
    (2.0000, 1.0000),
    (2.1000, 1.0000),
    (2.2000, 1.0000),
    (2.3000, 1.0000),
    (2.4000, 1.0000),
    (2.5000, 1.0000),
];

const TECH_PARAMS: [(f32, f32); 1] = [(15.0000, 2.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];
