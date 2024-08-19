use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, ConfigType, Modifier, ModifierConfig, ModifierData,
        ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new_with_config(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritRate),
                BuffScaling::Additive,
                |unit, _, config, team, lc_store, relics_store| {
                    let cdmg = unit.get_effective_advanced_stat(
                        AdvancedStat::CritDamage,
                        team,
                        lc_store,
                        relics_store,
                    );
                    if config.unwrap().get_bool() && cdmg > 1.2 {
                        0.6
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritDamage),
                BuffScaling::Additive,
                flat_value!(0.16),
            ),
        ],
        Some(ModifierConfig::new(ConfigType::Flag)),
        false,
    )]
}
