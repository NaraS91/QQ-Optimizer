use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BonusDMGFlag, BuffScaling, Modifier, ModifierData, ModifierTarget,
        Source, Stat, Unit,
    },
    utils::{lc_superimposition, value_with_lc_s},
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![ModifierData::new(
            ModifierTarget::Caster,
            Stat::Advanced(AdvancedStat::TotalDmgBoost(
                AdvancedStat::create_dmg_bonus_flag(0, 1, 0, 0, 0),
            )),
            BuffScaling::Additive,
            value_with_lc_s!(|s: usize| SKILL_PARAMS[s]),
        )],
        true,
    )]
}

const SKILL_PARAMS: [f32; 5] = [0.2800, 0.3500, 0.4200, 0.4900, 0.5600];
