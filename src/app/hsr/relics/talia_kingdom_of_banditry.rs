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
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                |unit, _, config, team, lc_store, relics_store| {
                    let spd =
                        unit.get_effective_base_stat(BaseStat::Spd, team, lc_store, relics_store);
                    if spd >= 145. {
                        0.2
                    } else {
                        0.
                    }
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                flat_value!(0.16),
            ),
        ],
        false,
    )]
}
