use super::{AdvancedStat, BaseStat, ModifierOrDOT, OptimizationTarget, Unit};
use crate::app::{
    hsr::{basics::Element, damange_calculator},
    light_cones_store::LightConesStore,
    relics_store::RelicsStore,
};

#[macro_export]
macro_rules! qq_enhanced_auto {
    ($x: expr) => {{
        OptimizationTarget {
            name: format!("qq enhanced auto {}", $x),
            value: |unit: &Unit,
                    team: &[Option<&Unit>; 3],
                    relics_store: &RelicsStore,
                    lc_store: &LightConesStore| {
                let team = &[Some(unit), team[0], team[1], team[2]];
                let attack =
                    unit.get_effective_base_stat(BaseStat::Atk, team, lc_store, relics_store);
                let crit_rate = unit.get_effective_advanced_stat(
                    AdvancedStat::CritRate,
                    team,
                    lc_store,
                    relics_store,
                );
                let crit_dmg = unit.get_effective_advanced_stat(
                    AdvancedStat::CritDamage,
                    team,
                    lc_store,
                    relics_store,
                );
                let quantum_boost = unit.get_effective_advanced_stat(
                    AdvancedStat::ElemDmgBoost(Element::Quantum),
                    team,
                    lc_store,
                    relics_store,
                );
                let total_dmg_boost = unit.get_effective_advanced_stat(
                    AdvancedStat::TotalDmgBoost(1),
                    team,
                    lc_store,
                    relics_store,
                );
                let res_penetration = unit.get_effective_advanced_stat(
                    AdvancedStat::TotalResPen,
                    team,
                    lc_store,
                    relics_store,
                );

                let skill_level = unit.unique_data.skill_level
                    + if unit.unique_data.eidolon >= 5 { 2 } else { 0 };
                let trait_bonus = 0.10;
                damange_calculator::standard_attack(
                    ENHANCED_PARAMS[unit.unique_data.attack_level as usize],
                    unit.unique_data.level as f32,
                    attack,
                    crit_rate,
                    crit_dmg,
                    quantum_boost
                        + total_dmg_boost
                        + ($x) as f32 * (SKILL_PARAMS[skill_level as usize].1 + trait_bonus),
                    0.,
                    res_penetration,
                    true,
                )
            },
        }
    }};
}

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

pub fn optimization_targets() -> Vec<OptimizationTarget> {
    vec![
        qq_enhanced_auto!(0),
        qq_enhanced_auto!(1),
        qq_enhanced_auto!(2),
        qq_enhanced_auto!(3),
        qq_enhanced_auto!(4),
    ]
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (2.0000, 0.1400, 4.0000),
    (2.0000, 0.1540, 4.0000),
    (2.0000, 0.1680, 4.0000),
    (2.0000, 0.1820, 4.0000),
    (2.0000, 0.1960, 4.0000),
    (2.0000, 0.2100, 4.0000),
    (2.0000, 0.2275, 4.0000),
    (2.0000, 0.2450, 4.0000),
    (2.0000, 0.2625, 4.0000),
    (2.0000, 0.2800, 4.0000),
    (2.0000, 0.2940, 4.0000),
    (2.0000, 0.3080, 4.0000),
    (2.0000, 0.3220, 4.0000),
    (2.0000, 0.3360, 4.0000),
    (2.0000, 0.3500, 4.0000),
];

const ULT_PARAMS: [f32; 15] = [
    1.2000, 1.2800, 1.3600, 1.4400, 1.5200, 1.6000, 1.7000, 1.8000, 1.9000, 2.0000, 2.0800, 2.1600,
    2.2400, 2.3200, 2.4000,
];

const TALENT_PARAMS: [f32; 15] = [
    0.3600, 0.3960, 0.4320, 0.4680, 0.5040, 0.5400, 0.5850, 0.6300, 0.6750, 0.7200, 0.7560, 0.7920,
    0.8280, 0.8640, 0.9000,
];

const TECH_PARAMS: [f32; 1] = [2.0000];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];

const ENHANCED_PARAMS: [f32; 9] = [
    1.2000, 1.4400, 1.6800, 1.9200, 2.1600, 2.4000, 2.6400, 2.8800, 3.1200,
];
