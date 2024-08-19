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
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |unit, _, _, team, lc_store, relics_store| {
                    let ehr = unit.get_effective_advanced_stat(
                        AdvancedStat::EffectHitRate,
                        team,
                        lc_store,
                        relics_store,
                    );
                    f32::min(ehr * 0.25, 0.25)
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::EffectHitRate),
                BuffScaling::Additive,
                flat_value!(0.1),
            ),
        ],
        false,
    )]
}
