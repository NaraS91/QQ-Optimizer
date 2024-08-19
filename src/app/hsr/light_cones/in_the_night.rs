use crate::app::hsr::{
    units::{
        AdvancedStat, BaseStat, BuffScaling, Modifier, ModifierData, ModifierTarget, Source, Stat,
        Unit,
    },
    utils::{flat_value, lc_superimposition},
};

pub fn modifiers(wearer: &Unit) -> Vec<Modifier> {
    vec![Modifier::new(
        (wearer.kind, Source::LightCone),
        vec![
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritRate),
                BuffScaling::Additive,
                flat_value!(0.18),
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::TotalDmgBoost(
                    AdvancedStat::create_dmg_bonus_flag(1, 0, 1, 0, 0),
                )),
                BuffScaling::Additive,
                |_, unit, _, team, lc_store, relics_store| {
                    let speed =
                        unit.get_effective_base_stat(BaseStat::Spd, team, lc_store, relics_store);
                    (speed - 100.).clamp(0., 60.).div_euclid(10.)
                        * SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].2
                },
            ),
            ModifierData::new(
                ModifierTarget::Caster,
                Stat::Advanced(AdvancedStat::CritDamageCond(
                    AdvancedStat::create_dmg_bonus_flag(0, 1, 0, 0, 0),
                )),
                BuffScaling::Additive,
                |_, unit, _, team, lc_store, relics_store| {
                    let speed =
                        unit.get_effective_base_stat(BaseStat::Spd, team, lc_store, relics_store);
                    (speed - 100.).clamp(0., 60.).div_euclid(10.)
                        * SKILL_PARAMS[lc_superimposition!(unit, lc_store) as usize].3
                },
            ),
        ],
        false,
    )]
}

const SKILL_PARAMS: [(f32, f32, f32, f32, f32); 5] = [
    (0.1800, 10.0000, 0.0600, 0.1200, 6.0000),
    (0.2100, 10.0000, 0.0700, 0.1400, 6.0000),
    (0.2400, 10.0000, 0.0800, 0.1600, 6.0000),
    (0.2700, 10.0000, 0.0900, 0.1800, 6.0000),
    (0.3000, 10.0000, 0.1000, 0.2000, 6.0000),
];
