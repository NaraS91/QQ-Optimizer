use enum_map::{Enum, enum_map};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use crate::app::light_cones_store::{self, LightConesStore};

use self::statistics::CHARACTER_INFO;

use super::{basics::{self, Path}, light_cones::{self, LightCone, LightConeKind}, relics::RelicPart};

mod argenti;
mod arlan;
mod asta;
mod bailu;
mod black_swan;
mod blade;
mod bronya;
mod clara;
mod dan_heng;
mod dan_heng_il;
mod dr_ratio;
mod fu_xuan;
mod gepard;
mod guinaifen;
mod hanya;
mod herta;
mod himeko;
mod hook;
mod huohuo;
mod jingliu;
mod jing_yuan;
mod kafka;
mod luka;
mod luocha;
mod lynx;
mod march_7th;
mod misha;
mod natasha;
mod pela;
mod trailblazer_p;
mod trailblazer_f;
mod qingque;
mod ruan_mei;
mod sampo;
mod seele;
mod serval;
mod silver_wolf;
mod sparkle;
mod sushang;
mod tingyun;
mod topaz;
mod welt;
mod xueyi;
mod yanqing;
mod yukong;

mod statistics;

#[allow(non_camel_case_types)]
#[derive(Debug, Enum, EnumIter, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum UnitKind {
    Argenti,
    Arlan,
    Asta,
    Bailu,
    Black_Swan,
    Blade,
    Bronya,
    Clara,
    Dan_Heng,
    Dan_Heng_IL,
    Dr_Ratio,
    Fu_Xuan,
    Gepard,
    Guinaifen,
    Hanya,
    Herta,
    Himeko,
    Hook,
    Huohuo,
    Jingliu,
    Jing_Yuan,
    Kafka,
    Luka,
    Luocha,
    Lynx,
    March_7th,
    Misha,
    Natasha,
    Pela,
    Trailblazer_P,
    Trailblazer_F,
    Qingque,
    Ruan_Mei,
    Sampo,
    Seele,
    Serval,
    Silver_Wolf,
    Sparkle,
    Sushang,
    Tingyun,
    Topaz,
    Welt,
    Xueyi,
    Yanqing,
    Yukong
}

impl ToString for UnitKind {
    fn to_string(&self) -> String {
        format!("{:?}", self).replace("_", " ")
    }
}

impl UnitKind {
    pub fn file_name(&self) -> String {
        match self {
            Self::Trailblazer_F => "Trailblazer_F_F".to_owned(),
            Self::Trailblazer_P => "Trailblazer_F_P".to_owned(),
            _ => self.to_string().replace(" ", "_")
        }
    }

    pub fn get_modifiers(&self) -> Box<dyn Fn(&Unit) -> Vec<ModifierSource>> {
        match self {
            Self::Argenti => Box::new(argenti::modifiers),
            Self::Arlan => Box::new(arlan::modifiers),
            Self::Asta => Box::new(asta::modifiers),
            Self::Bailu => Box::new(bailu::modifiers),
            Self::Black_Swan => Box::new(black_swan::modifiers),
            Self::Blade => Box::new(blade::modifiers),
            Self::Bronya => Box::new(bronya::modifiers),
            Self::Clara => Box::new(clara::modifiers),
            Self::Dan_Heng => Box::new(dan_heng::modifiers),
            Self::Dan_Heng_IL => Box::new(dan_heng_il::modifiers),
            Self::Dr_Ratio => Box::new(dr_ratio::modifiers),
            Self::Fu_Xuan => Box::new(fu_xuan::modifiers),
            Self::Gepard => Box::new(gepard::modifiers),
            Self::Guinaifen => Box::new(guinaifen::modifiers),
            Self::Hanya => Box::new(hanya::modifiers),
            Self::Herta => Box::new(herta::modifiers),
            Self::Himeko => Box::new(himeko::modifiers),
            Self::Hook => Box::new(hook::modifiers),
            Self::Huohuo => Box::new(huohuo::modifiers),
            Self::Jingliu => Box::new(jingliu::modifiers),
            Self::Jing_Yuan => Box::new(jing_yuan::modifiers),
            Self::Kafka => Box::new(kafka::modifiers),
            Self::Luka => Box::new(luka::modifiers),
            Self::Luocha => Box::new(luocha::modifiers),
            Self::Lynx => Box::new(lynx::modifiers),
            Self::March_7th => Box::new(march_7th::modifiers),
            Self::Misha => Box::new(misha::modifiers),
            Self::Natasha => Box::new(natasha::modifiers),
            Self::Pela => Box::new(pela::modifiers),
            Self::Trailblazer_P => Box::new(trailblazer_p::modifiers),
            Self::Trailblazer_F => Box::new(trailblazer_f::modifiers),
            Self::Qingque => Box::new(qingque::modifiers),
            Self::Ruan_Mei => Box::new(ruan_mei::modifiers),
            Self::Sampo => Box::new(sampo::modifiers),
            Self::Seele => Box::new(seele::modifiers),
            Self::Serval => Box::new(serval::modifiers),
            Self::Silver_Wolf => Box::new(silver_wolf::modifiers),
            Self::Sparkle => Box::new(sparkle::modifiers),
            Self::Sushang => Box::new(sushang::modifiers),
            Self::Tingyun => Box::new(tingyun::modifiers),
            Self::Topaz => Box::new(topaz::modifiers),
            Self::Welt => Box::new(welt::modifiers),
            Self::Xueyi => Box::new(xueyi::modifiers),
            Self::Yanqing => Box::new(yanqing::modifiers),
            Self::Yukong => Box::new(yukong::modifiers)
        }
    }
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize)]
enum BaseStat {
    Hp,
    Atk,
    Def,
    Spd,
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize)]
enum AdvancedStat {
    CritRate,
    CritDamage,
    BreakEffect,
    OutgoingHealingBoost,
    MaxEnergy,
    EnergyRegenerationRate,
    EffectHitRate,
    EffectRes,
    TotalDmgBoost,
    ElemDmgBoost(basics::Element),
    ElemDmgRes(basics::Element),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Unit {
    pub kind: UnitKind,
    pub path: Path,
    pub rarity: u8,
    pub level: u32,
    pub ascension: u32,
    base_stats: [[f32; BaseStat::LENGTH];7],
    base_stats_growth: [f32; BaseStat::LENGTH],
    pub trait_buffs: [(BuffStat, f32);10],
    pub relics: [Option<usize>; 6],
    pub light_cone: Option<usize>,
    advanced_stats: enum_map::EnumMap<AdvancedStat, f32>,
    pub ultimate_level: u8,
    pub skill_level: u8,
    pub attack_level: u8,
    pub talent_level: u8,
    pub eidolon: u8,
    pub team: [Option<UnitKind>; 3]
}

pub struct ModifierData {
    target: ModifierTarget,
    stat: BuffStat,
    scaling: BuffScaling,
    value: f32
}

pub enum ModifierSource {
    Ultimate(Vec<ModifierData>),
    Skill(Vec<ModifierData>),
    Trace(u8, Vec<ModifierData>),
    Technique(Vec<ModifierData>),
    Eidolon(Vec<ModifierData>),
    LightCone(Vec<ModifierData>)
}

#[derive(Debug, Clone, Copy)]
pub enum ModifierTarget {
    Ally,
    Team,
    Caster,
    Enemy
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize)]
pub enum BuffStat {
    Base(BaseStat),
    Advanced(AdvancedStat)
}

pub enum BuffScaling {
    Additive,
    Multiplicative
}

impl Unit {
    pub fn new(character: UnitKind) -> Unit {
        if CHARACTER_INFO[character as usize].0 != character {
            panic!("tried to retrieve statistics of {:?}, got {:?}", character, CHARACTER_INFO[character as usize].0);
        }
        let index = character as usize;
        Unit { 
            kind: character, 
            path: CHARACTER_INFO[index].1,
            level: 80,
            ascension: 6,
            rarity: CHARACTER_INFO[index].2.rarity,
            base_stats: CHARACTER_INFO[index].2.base,
            base_stats_growth: CHARACTER_INFO[index].2.growth,
            trait_buffs: CHARACTER_INFO[index].3,
            relics: [None; 6], 
            light_cone: Some(0),
            advanced_stats: enum_map! {_ => 0.0},
            ultimate_level: 1,
            skill_level: 1,
            attack_level: 1,
            talent_level: 1,
            eidolon: 0,
            team: [None; 3]
        }
    }

    pub fn update_relics(&mut self, relic_id: usize, relic_part: RelicPart) {
        self.relics[relic_part.get_index()] = Some(relic_id)
    }

    pub fn max_level(&self) -> u32 {
        self.ascension * 10 + 20
    }

    pub fn min_level(&self) -> u32 {
        if self.ascension == 0 {
            1
        } else {
            self.ascension * 10 + 10
        }
    }

    pub fn base_stats(self, light_cones_store: &LightConesStore) -> [f32; BaseStat::LENGTH] {
        let mut base_stats = self.base_stats[self.ascension as usize];
        for i in 0..BaseStat::LENGTH {
            base_stats[i] += (self.level - self.min_level()) as f32 * self.base_stats_growth[i];
        }

        if self.light_cone.is_none() {
            return base_stats;
        }

        let lc = light_cones_store.get(self.light_cone.expect("checked above"));

        if let Some(lc) = lc {
            let lc_stats = lc.stats();
            for i in 0..light_cones::BaseStat::LENGTH {
                base_stats[i] += lc_stats[i];
            }
            base_stats
        } else {
            base_stats
        }
    }
}