use crate::app::hsr::{
    units::{
        BaseStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData, ModifierTarget,
        Source, Stat, Unit,
    },
    utils::lc_superimposition,
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new_with_config(
        (wearer.kind, Source::LightCone),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Base(BaseStat::Def),
            BuffScaling::Multiplicative,
            |_, unit, config, _, lc_store, _| {
                let s = lc_superimposition!(unit, lc_store) as usize;
                if config.unwrap().get_bool() {
                    SKILL_PARAMS[s].0 + SKILL_PARAMS[s].2
                } else {
                    SKILL_PARAMS[s].0
                }
            },
        )],
        Some(ModifierConfig::new(ConfigType::Flag)),
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32, f32); 5] = [
    (0.1600, 0.5000, 0.1600),
    (0.2000, 0.5000, 0.2000),
    (0.2400, 0.5000, 0.2400),
    (0.2800, 0.5000, 0.2800),
    (0.3200, 0.5000, 0.3200),
];
