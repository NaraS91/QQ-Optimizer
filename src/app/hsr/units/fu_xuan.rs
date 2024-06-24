use crate::app::hsr::units::{AdvancedStat, BaseStat, BuffScaling, ModifierData, Source, Stat};

use super::{Modifier, ModifierOrDOT, ModifierTarget, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    let mut skill_data: Vec<ModifierData> = vec![
        ModifierData::new(
            ModifierTarget::Team,
            Stat::Advanced(AdvancedStat::CritRate),
            BuffScaling::Additive,
            |_, buffer, _, _, _, _| SKILL_PARAMS[buffer.unique_data.skill_level as usize].4,
        ),
        ModifierData::new(
            ModifierTarget::Team,
            Stat::Base(BaseStat::Hp),
            BuffScaling::Additive,
            |_, buffer, _, team, light_cones_store, relics_store| {
                let skill_level = buffer.unique_data.skill_level
                    + (if buffer.unique_data.eidolon >= 3 {
                        2
                    } else {
                        0
                    });
                SKILL_PARAMS[skill_level as usize].3
                    * buffer.get_effective_base_stat(
                        BaseStat::Hp,
                        team,
                        light_cones_store,
                        relics_store,
                    )
            },
        ),
    ];

    if unit.unique_data.eidolon >= 1 {
        skill_data.push(ModifierData::new(
            ModifierTarget::Team,
            Stat::Advanced(AdvancedStat::CritDamage),
            BuffScaling::Additive,
            |_, _, _, _, _, _| 0.3,
        ))
    }
    vec![ModifierOrDOT::Modifier(Modifier::new(
        (unit.kind, Source::Skill),
        skill_data,
        true,
    ))]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 15] = [
    (0.6500, 0.0000, 3.0000, 0.0300, 0.0600),
    (0.6500, 0.0000, 3.0000, 0.0330, 0.0660),
    (0.6500, 0.0000, 3.0000, 0.0360, 0.0720),
    (0.6500, 0.0000, 3.0000, 0.0390, 0.0780),
    (0.6500, 0.0000, 3.0000, 0.0420, 0.0840),
    (0.6500, 0.0000, 3.0000, 0.0450, 0.0900),
    (0.6500, 0.0000, 3.0000, 0.0488, 0.0975),
    (0.6500, 0.0000, 3.0000, 0.0525, 0.1050),
    (0.6500, 0.0000, 3.0000, 0.0563, 0.1125),
    (0.6500, 0.0000, 3.0000, 0.0600, 0.1200),
    (0.6500, 0.0000, 3.0000, 0.0630, 0.1260),
    (0.6500, 0.0000, 3.0000, 0.0660, 0.1320),
    (0.6500, 0.0000, 3.0000, 0.0690, 0.1380),
    (0.6500, 0.0000, 3.0000, 0.0720, 0.1440),
    (0.6500, 0.0000, 3.0000, 0.0750, 0.1500),
];

const ULT_PARAMS: [f32; 15] = [
    0.6000, 0.6400, 0.6800, 0.7200, 0.7600, 0.8000, 0.8500, 0.9000, 0.9500, 1.0000, 1.0400, 1.0800,
    1.1200, 1.1600, 1.2000,
];

const TALENT_PARAMS: [(f32, f32, f32); 15] = [
    (0.1000, 0.5000, 0.8000),
    (0.1080, 0.5000, 0.8100),
    (0.1160, 0.5000, 0.8200),
    (0.1240, 0.5000, 0.8300),
    (0.1320, 0.5000, 0.8400),
    (0.1400, 0.5000, 0.8500),
    (0.1500, 0.5000, 0.8625),
    (0.1600, 0.5000, 0.8750),
    (0.1700, 0.5000, 0.8875),
    (0.1800, 0.5000, 0.9000),
    (0.1880, 0.5000, 0.9100),
    (0.1960, 0.5000, 0.9200),
    (0.2040, 0.5000, 0.9300),
    (0.2120, 0.5000, 0.9400),
    (0.2200, 0.5000, 0.9500),
];

const TECH_PARAMS: [(f32, f32); 1] = [(20.0000, 2.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.2500, 0.3000, 0.3500, 0.4000, 0.4500, 0.5000, 0.5500, 0.6000, 0.6500,
];
