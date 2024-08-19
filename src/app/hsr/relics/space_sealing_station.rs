use crate::app::hsr::units::{
    BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Base(BaseStat::Atk),
            BuffScaling::Multiplicative,
            |unit, _, _, team, lc_store, relics_store| {
                let speed =
                    unit.get_effective_base_stat(BaseStat::Spd, team, lc_store, relics_store);
                0.12 + if speed >= 120. { 0.12 } else { 0. }
            },
        )],
        false,
    )]
}
