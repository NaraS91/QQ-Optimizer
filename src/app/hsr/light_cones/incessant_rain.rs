use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, ConfigType, Modifier, ModifierConfig,
        ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::{lc_superimposition, value_with_lc_s},
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new_with_config(
        (wearer.kind, Source::LightCone),
        vec![
            ModifierData::new(
                ModifierTarget::Enemy,
                Stat::Advanced(AdvancedStat::Vulnerability),
                BuffScaling::Additive,
                value_with_lc_s!(|s: usize| SKILL_PARAMS[s].2),
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritRate),
                BuffScaling::Additive,
                |_, unit, config, _, lc_store, _| {
                    if config.unwrap().get_bool() {
                        SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].4
                    } else {
                        0.
                    }
                },
            ),
        ],
        Some(ModifierConfig::new(ConfigType::Flag)),
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 5] = [
    (0.2400, 1.0000, 0.1200, 3.0000, 0.1200),
    (0.2800, 1.0000, 0.1400, 3.0000, 0.1400),
    (0.3200, 1.0000, 0.1600, 3.0000, 0.1600),
    (0.3600, 1.0000, 0.1800, 3.0000, 0.1800),
    (0.4000, 1.0000, 0.2000, 3.0000, 0.2000),
];
