use std::slice::Iter;

use super::{
    basics::Element,
    units::{AdvancedStat, BaseStat, Stat, UnitKind},
};
use enum_map::Enum;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

mod band_of_sizzling_thunder;
mod belobog_of_the_architects;
mod broken_keel;
pub mod buffs;
mod celestial_differentiator;
mod champion_of_streetwise_boxing;
mod eagle_of_twilight_line;
mod firesmith_of_lavaforging;
mod firmament_frontline_glamoth;
mod fleet_of_the_ageless;
mod genius_of_brilliant_stars;
mod guard_of_wuthering_snow;
mod hunter_of_glacial_forest;
mod inert_salsotto;
mod knight_of_purity_palace;
mod longevous_disciple;
mod messenger_traversing_hackerspace;
mod musketeer_of_wild_wheat;
mod pancosmic_commercial_enterprise;
mod passerby_of_wandering_cloud;
mod penacony_land_of_the_dreams;
mod pioneer_diver_of_dead_waters;
mod prisoner_in_deep_confinement;
mod rutilant_arena;
mod space_sealing_station;
mod sprightly_vonwacq;
mod talia_kingdom_of_banditry;
mod the_ashblazing_grand_duke;
mod thief_of_shooting_meteor;
mod wastelander_of_banditry_desert;
mod watchmaker_master_of_dream_machinations;

const RELIC_MAIN_STAT_GROWTH: [(RelicStat, f32, f32); 17] = [
    (RelicStat::Hp, 112.896, 39.5136),
    (RelicStat::Atk, 56.448, 19.7568),
    (RelicStat::AtkP, 0.06912, 0.02419),
    (RelicStat::HpP, 0.06912, 0.02419),
    (RelicStat::DefP, 0.0864, 0.030240001),
    (RelicStat::Cr, 0.05184, 0.018144),
    (RelicStat::Cd, 0.10368, 0.036288),
    (RelicStat::OH, 0.055296, 0.019354),
    (RelicStat::EHR, 0.06912, 0.024192),
    (RelicStat::Spd, 4.032, 1.4),
    (
        RelicStat::ElementalDmg(Element::Fire),
        0.062208,
        0.021773001,
    ),
    (RelicStat::ElementalDmg(Element::Ice), 0.062208, 0.021773001),
    (
        RelicStat::ElementalDmg(Element::Physical),
        0.062208,
        0.021773001,
    ),
    (
        RelicStat::ElementalDmg(Element::Imaginary),
        0.062208,
        0.021773001,
    ),
    (
        RelicStat::ElementalDmg(Element::Lightning),
        0.062208,
        0.021773001,
    ),
    (
        RelicStat::ElementalDmg(Element::Quantum),
        0.062208,
        0.021773001,
    ),
    (
        RelicStat::ElementalDmg(Element::Wind),
        0.062208,
        0.021773001,
    ),
];

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Relic {
    pub id: usize,
    pub level: u8,
    pub part: RelicPart,
    pub set: RelicSet,
    pub main_stat: (RelicStat, f32),
    pub sub: [Option<(RelicStat, f32)>; 4],
    pub equipped: Option<UnitKind>,
    pub opt_main: (Stat, f32, bool),
    pub opt_sub: [(Stat, f32, bool); 4],
}

impl Relic {
    pub fn new(
        level: u8,
        part: RelicPart,
        set: RelicSet,
        main_stat: RelicStat,
        sub: [Option<(RelicStat, f32)>; 4],
        equipped: Option<UnitKind>,
    ) -> Relic {
        Relic {
            id: 0,
            level,
            part,
            set,
            main_stat: (main_stat, Relic::main_stat_value(main_stat, level)),
            sub,
            equipped,
            opt_main: (Stat::Base(BaseStat::Atk), 0., true),
            opt_sub: [(Stat::Base(BaseStat::Atk), 0., true); 4],
        }
    }

    pub fn to_str(&self) -> String {
        let mut result = self.part.to_str() + "\n main: ";
        result += &self.main_stat.0.to_str();
        for sub in self.sub {
            if let Some(sub) = sub {
                result += &format!("\n  {}: {}", sub.0.to_str(), sub.1);
            }
        }

        result
    }

    pub fn optimize(&mut self) {
        let (stat, flat) = self.main_stat.0.to_buff_stat();
        self.opt_main = (stat, self.main_stat.1, flat);
        let mut i = 0;
        for stat_op in self.sub {
            if let Some((stat, value)) = stat_op {
                let (stat, flat) = stat.to_buff_stat();
                self.opt_sub[i] = (stat, value, flat);
            } else {
                self.opt_sub[i] = (Stat::Base(BaseStat::Atk), 0., true);
            }
            i += 1;
        }
    }

    fn main_stat_value(main_stat: RelicStat, level: u8) -> f32 {
        for (stat, base, growth) in RELIC_MAIN_STAT_GROWTH {
            if main_stat == stat {
                return base + level as f32 * growth;
            }
        }

        0.
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Enum, Debug, PartialEq, Eq)]
pub enum RelicSet {
    Cavern(CavernSet),
    Planar(PlanarSet),
}

impl RelicSet {
    pub fn file_name(&self) -> String {
        match self {
            RelicSet::Cavern(cavern) => cavern.file_name(),
            RelicSet::Planar(planar) => planar.file_name(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Enum, EnumIter, PartialEq, Eq)]
pub enum RelicPart {
    Head,
    Hands,
    Body,
    Feet,
    Sphere,
    Rope,
}

impl RelicPart {
    pub fn get_index(&self) -> usize {
        match self {
            Self::Head => 0,
            Self::Hands => 1,
            Self::Body => 2,
            Self::Feet => 3,
            Self::Sphere => 4,
            Self::Rope => 5,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::Head => "Head".to_owned(),
            Self::Hands => "Hands".to_owned(),
            Self::Body => "Body".to_owned(),
            Self::Feet => "Feet".to_owned(),
            Self::Sphere => "Sphere".to_owned(),
            Self::Rope => "Rope".to_owned(),
        }
    }

    pub fn file_name(&self) -> String {
        match self {
            Self::Head => "head".to_owned(),
            Self::Hands => "hand".to_owned(),
            Self::Body => "body".to_owned(),
            Self::Feet => "feet".to_owned(),
            Self::Sphere => "sphere".to_owned(),
            Self::Rope => "rope".to_owned(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Serialize, Deserialize, Enum, EnumIter, Debug, PartialEq, Eq)]
pub enum CavernSet {
    Passerby_Of_Wandering_Cloud,
    Musketeer_Of_Wild_Wheat,
    Knight_Of_Purity_Palace,
    Hunter_Of_Glacial_Forest,
    Champion_Of_Streetwise_Boxing,
    Guard_Of_Wuthering_Snow,
    Firesmith_Of_Lavaforging,
    Genius_Of_Brilliant_Stars,
    Band_Of_Sizzling_Thunder,
    Eagle_Of_Twilight_Line,
    Thief_Of_Shooting_Meteor,
    Wastelander_Of_Banditry_Desert,
    Longevous_Disciple,
    Messenger_Traversing_Hackerspace,
    The_Ashblazing_Grand_Duke,
    Prisoner_In_Deep_Confinement,
    Pioneer_Diver_Of_Dead_Waters,
    Watchmaker_Master_Of_Dream_Machinations,
}

impl CavernSet {
    pub fn file_name(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Serialize, Deserialize, Enum, EnumIter, Debug, PartialEq, Eq)]
pub enum PlanarSet {
    Space_Sealing_Station,
    Fleet_Of_The_Ageless,
    Pancosmic_Commercial_Enterprise,
    Belobog_Of_The_Architects,
    Celestial_Differentiator,
    Inert_Salsotto,
    Talia_Kingdom_Of_Banditry,
    Sprightly_Vonwacq,
    Rutilant_Arena,
    Broken_Keel,
    Firmament_Frontline_Glamoth,
    Penacony_Land_Of_The_Dreams,
}

impl PlanarSet {
    pub fn file_name(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelicStat {
    Atk,
    AtkP,
    Def,
    DefP,
    HpP,
    Hp,
    Err,
    Cr,
    Cd,
    BE,
    Spd,
    EHR,
    ERes,
    OH,
    ElementalDmg(Element),
}

impl RelicStat {
    // return true for flat bonuses and false for %
    pub fn to_buff_stat(&self) -> (Stat, bool) {
        match self {
            RelicStat::Atk => (Stat::Base(BaseStat::Atk), true),
            RelicStat::AtkP => (Stat::Base(BaseStat::Atk), false),
            RelicStat::Def => (Stat::Base(BaseStat::Def), true),
            RelicStat::DefP => (Stat::Base(BaseStat::Def), false),
            RelicStat::HpP => (Stat::Base(BaseStat::Hp), false),
            RelicStat::Hp => (Stat::Base(BaseStat::Hp), true),
            RelicStat::Err => (Stat::Advanced(AdvancedStat::EnergyRegenerationRate), true),
            RelicStat::Cr => (Stat::Advanced(AdvancedStat::CritRate), true),
            RelicStat::Cd => (Stat::Advanced(AdvancedStat::CritDamage), true),
            RelicStat::BE => (Stat::Advanced(AdvancedStat::BreakEffect), true),
            RelicStat::Spd => (Stat::Base(BaseStat::Spd), true),
            RelicStat::EHR => (Stat::Advanced(AdvancedStat::EffectHitRate), true),
            RelicStat::ERes => (Stat::Advanced(AdvancedStat::EffectRes), true),
            RelicStat::OH => (Stat::Advanced(AdvancedStat::OutgoingHealingBoost), true),
            RelicStat::ElementalDmg(element) => {
                (Stat::Advanced(AdvancedStat::ElemDmgBoost(*element)), true)
            }
        }
    }

    pub fn unique_id(&self) -> usize {
        match self {
            RelicStat::Atk => 0,
            RelicStat::AtkP => 1,
            RelicStat::Def => 2,
            RelicStat::DefP => 3,
            RelicStat::HpP => 4,
            RelicStat::Hp => 5,
            RelicStat::Err => 6,
            RelicStat::Cr => 7,
            RelicStat::Cd => 8,
            RelicStat::BE => 9,
            RelicStat::Spd => 10,
            RelicStat::EHR => 11,
            RelicStat::ERes => 12,
            RelicStat::OH => 13,
            RelicStat::ElementalDmg(element) => 14 + element.unique_id(),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            RelicStat::Atk => "Atk".to_owned(),
            RelicStat::AtkP => "Atk%".to_owned(),
            RelicStat::Def => "Def".to_owned(),
            RelicStat::DefP => "Def%".to_owned(),
            RelicStat::HpP => "Hp%".to_owned(),
            RelicStat::Hp => "Hp".to_owned(),
            RelicStat::Err => "Energy Regeneration Rate".to_owned(),
            RelicStat::Cr => "Crit Rate".to_owned(),
            RelicStat::Cd => "Crit DMG".to_owned(),
            RelicStat::BE => "Break Effect".to_owned(),
            RelicStat::Spd => "Spd".to_owned(),
            RelicStat::EHR => "Effect Hit Rate".to_owned(),
            RelicStat::ERes => "Effect RES".to_owned(),
            RelicStat::OH => "Outgoing Healing Boost".to_owned(),
            RelicStat::ElementalDmg(element) => element.to_str() + "DMG Boost",
        }
    }
}
