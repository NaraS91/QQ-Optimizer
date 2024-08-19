use crate::app::hsr::{
    units::{
        AdvancedStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat, Unit,
    },
    utils::value_with_lc_s,
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Advanced(AdvancedStat::OutgoingHealingBoost),
            BuffScaling::Additive,
            value_with_lc_s!(|s: usize| SKILL_PARAMS[s]),
        )],
        true,
    )]
}

const SKILL_PARAMS: [f32; 5] = [0.1200, 0.1500, 0.1800, 0.2100, 0.2400];
