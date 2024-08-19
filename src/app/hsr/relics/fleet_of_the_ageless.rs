use crate::app::hsr::{
    units::{BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit},
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![
            ModifierData::new(
                ModifierTarget::Allies,
                Stat::Base(BaseStat::Atk),
                BuffScaling::Multiplicative,
                |_, buffer, _, team, lc_store, relics_store| {
                    let speed =
                        buffer.get_effective_base_stat(BaseStat::Spd, team, lc_store, relics_store);
                    if speed >= 120. {
                        0.8
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Base(BaseStat::Hp),
                BuffScaling::Multiplicative,
                flat_value!(0.12),
            ),
        ],
        false,
    )]
}
