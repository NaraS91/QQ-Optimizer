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
            Stat::Advanced(AdvancedStat::EffectHitRate),
            BuffScaling::Additive,
            value_with_lc_s!(|s: usize| SKILL_PARAMS[s].0),
        )],
        true,
    )]
}

const SKILL_PARAMS: [(f32, f32); 5] = [
    (0.2000, 3.0000),
    (0.2500, 3.0000),
    (0.3000, 3.0000),
    (0.3500, 3.0000),
    (0.4000, 3.0000),
];
