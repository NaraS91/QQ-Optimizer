use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat,
        Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                |unit, _, _, team, lc_store, relics_store| {
                    let ehr = unit.get_effective_advanced_stat(
                        AdvancedStat::EffectHitRate,
                        team,
                        lc_store,
                        relics_store,
                    );
                    if ehr > 0.5 {
                        0.15
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Def),
                BuffScaling::Multiplicative,
                flat_value!(0.15),
            ),
        ],
        false,
    )]
}
