use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![
        Modifier::new(
            (wearer.kind, Source::Relic2p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                flat_value!(0.16),
            )],
            true,
        ),
        Modifier::new(
            (wearer.kind, Source::Relic4p),
            vec![ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::BreakEffect),
                BuffScaling::Additive,
                flat_value!(0.16),
            )],
            true,
        ),
    ]
}
