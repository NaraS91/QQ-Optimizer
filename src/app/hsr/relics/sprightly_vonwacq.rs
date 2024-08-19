use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::flat_value,
};

fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::Relic2p),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Advanced(AdvancedStat::EnergyRegenerationRate),
            BuffScaling::Additive,
            flat_value!(0.05),
        )],
        true,
    )]
}
