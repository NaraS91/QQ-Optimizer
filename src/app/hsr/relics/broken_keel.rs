use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Allies,
                Stat::Advanced(AdvancedStat::CritDamage),
                BuffScaling::Additive,
                |_, buffer, config, team, lc_store, relics_store| {
                    let eres = buffer.get_effective_advanced_stat(
                        AdvancedStat::EffectRes,
                        team,
                        lc_store,
                        relics_store,
                    );
                    if eres >= 0.3 {
                        0.1
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::EffectRes),
                BuffScaling::Additive,
                flat_value!(0.1),
            ),
        ],
        false,
    )]
}
