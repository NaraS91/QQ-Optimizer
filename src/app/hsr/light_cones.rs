use enum_map::Enum;
use log::debug;
use serde::{Deserialize, Serialize};

use super::{basics::Path, units::UnitKind};
mod a_secret_vow;
mod adversarial;
mod amber;
mod an_instant_before_a_gaze;
mod arrows;
mod baptism_of_pure_thought;
mod before_dawn;
mod before_the_tutorial_mission_starts;
mod brighter_than_the_sun;
mod but_the_battle_isnt_over;
mod carve_the_moon_weave_the_clouds;
mod chorus;
mod collapsing_sky;
mod cornucopia;
mod cruising_in_the_stellar_sea;
mod dance_dance_dance;
mod darting_arrow;
mod data_bank;
mod day_one_of_my_new_life;
mod defense;
mod destinys_threads_forewoven;
mod dreamville_adventure;
mod earthly_escapade;
mod echoes_of_the_coffin;
mod eyes_of_the_prey;
mod fermata;
mod final_victor;
mod fine_fruit;
mod flames_afar;
mod geniuses_repose;
mod good_night_and_sleep_well;
mod hey_over_here;
mod hidden_shadow;
mod i_shall_be_my_own_sword;
mod in_the_name_of_the_world;
mod in_the_night;
mod incessant_rain;
mod indelible_promise;
mod its_showtime;
mod landaus_choice;
mod looop;
mod make_the_world_clamor;
mod mediation;
mod memories_of_the_past;
mod meshing_cogs;
mod moment_of_victory;
mod multiplication;
mod mutual_demise;
mod night_of_fright;
mod night_on_the_milky_way;
mod nowhere_to_run;
mod on_the_fall_of_an_aeon;
mod only_silence_remains;
mod passkey;
mod past_and_future;
mod past_self_in_mirror;
mod patience_is_all_you_need;
mod perfect_timing;
mod pioneering;
mod planetary_rendezvous;
mod postop_conversation;
mod quid_pro_quo;
mod reforged_remembrance;
mod resolution_shines_as_pearls_of_sweat;
mod return_to_darkness;
mod river_flows_in_spring;
mod sagacity;
mod shared_feeling;
mod shattered_home;
mod she_already_shut_her_eyes;
mod sleep_like_the_dead;
mod solitary_healing;
mod something_irreplaceable;
mod subscribe_for_more;
mod swordplay;
mod texture_of_memories;
mod the_birth_of_the_self;
mod the_day_the_cosmos_fell;
mod the_moles_welcome_you;
mod the_seriousness_of_breakfast;
mod the_unreachable_side;
mod this_is_me;
mod time_waits_for_no_one;
mod today_is_another_peaceful_day;
mod trend_of_the_universal_market;
mod under_the_blue_sky;
mod void;
mod warmth_shortens_cold_nights;
mod we_are_wildfire;
mod we_will_meet_again;
mod what_is_real;
mod woof_walk_time;
mod worrisome_blissful;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct LightCone {
    pub id: usize,
    pub kind: LightConeKind,
    pub path: Path,
    pub rarity: u8,
    pub level: u16,
    pub ascension: u16,
    pub superimposition: u8,
    stats: LightConeStats,
    pub equipped: Option<UnitKind>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Enum)]
pub enum BaseStat {
    Hp,
    Atk,
    Def,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct LightConeStats {
    base: [[f32; BaseStat::LENGTH]; 7],
    growth: [f32; BaseStat::LENGTH],
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Serialize, Deserialize, Enum, Debug)]
pub enum LightConeKind {
    A_Secret_Vow,
    Adversarial,
    Amber,
    An_Instant_Before_A_Gaze,
    Arrows,
    Baptism_Of_Pure_Thought,
    Before_Dawn,
    Before_The_Tutorial_Mission_Starts,
    Brighter_Than_The_Sun,
    But_The_Battle_Isnt_Over,
    Carve_The_Moon_Weave_The_Clouds,
    Chorus,
    Collapsing_Sky,
    Cornucopia,
    Cruising_In_The_Stellar_Sea,
    Dance_Dance_Dance,
    Darting_Arrow,
    Data_Bank,
    Day_One_Of_My_New_Life,
    Defense,
    Destinys_Threads_Forewoven,
    Dreamville_Adventure,
    Earthly_Escapade,
    Echoes_Of_The_Coffin,
    Eyes_Of_The_Prey,
    Fermata,
    Final_Victor,
    Fine_Fruit,
    Flames_Afar,
    Geniuses_Repose,
    Good_Night_And_Sleep_Well,
    Hey_Over_Here,
    Hidden_Shadow,
    I_Shall_Be_My_Own_Sword,
    In_The_Name_Of_The_World,
    In_The_Night,
    Incessant_Rain,
    Indelible_Promise,
    Its_Showtime,
    Landaus_Choice,
    Loop,
    Make_The_World_Clamor,
    Mediation,
    Memories_Of_The_Past,
    Meshing_Cogs,
    Moment_Of_Victory,
    Multiplication,
    Mutual_Demise,
    Night_Of_Fright,
    Night_On_The_Milky_Way,
    Nowhere_To_Run,
    On_The_Fall_Of_An_Aeon,
    Only_Silence_Remains,
    Passkey,
    Past_Self_In_Mirror,
    Past_And_Future,
    Patience_Is_All_You_Need,
    Perfect_Timing,
    Pioneering,
    Planetary_Rendezvous,
    Postop_Conversation,
    Quid_Pro_Quo,
    Reforged_Remembrance,
    Resolution_Shines_As_Pearls_Of_Sweat,
    Return_To_Darkness,
    River_Flows_In_Spring,
    Sagacity,
    Shared_Feeling,
    Shattered_Home,
    She_Already_Shut_Her_Eyes,
    Sleep_Like_The_Dead,
    Solitary_Healing,
    Something_Irreplaceable,
    Subscribe_For_More,
    Swordplay,
    Texture_Of_Memories,
    The_Birth_Of_The_Self,
    The_Day_The_Cosmos_Fell,
    The_Moles_Welcome_You,
    The_Seriousness_Of_Breakfast,
    The_Unreachable_Side,
    This_Is_Me,
    Time_Waits_For_No_One,
    Today_Is_Another_Peaceful_Day,
    Trend_Of_The_Universal_Market,
    Under_The_Blue_Sky,
    Void,
    Warmth_Shortens_Cold_Nights,
    We_Are_Wildfire,
    We_Will_Meet_Again,
    What_Is_Real,
    Woof_Walk_Time,
    Worrisome_Blissful,
}

impl LightConeKind {
    pub fn file_name(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }

    pub fn from_str(str: &str) -> Option<LightConeKind> {
        match str {
            "A Secret Vow" => Some(LightConeKind::A_Secret_Vow),
            "Adversarial" => Some(LightConeKind::Adversarial),
            "Amber" => Some(LightConeKind::Amber),
            "An Instant Before A Gaze" => Some(LightConeKind::An_Instant_Before_A_Gaze),
            "Arrows" => Some(LightConeKind::Arrows),
            "Baptism of Pure Thought" => Some(LightConeKind::Baptism_Of_Pure_Thought),
            "Before Dawn" => Some(LightConeKind::Before_Dawn),
            "Before the Tutorial Mission Starts" => {
                Some(LightConeKind::Before_The_Tutorial_Mission_Starts)
            }
            "Brighter Than the Sun" => Some(LightConeKind::Brighter_Than_The_Sun),
            "But the Battle Isn't Over" => Some(LightConeKind::But_The_Battle_Isnt_Over),
            "Carve the Moon, Weave the Clouds" => {
                Some(LightConeKind::Carve_The_Moon_Weave_The_Clouds)
            }
            "Chorus" => Some(LightConeKind::Chorus),
            "Collapsing Sky" => Some(LightConeKind::Collapsing_Sky),
            "Cornucopia" => Some(LightConeKind::Cornucopia),
            "Cruising in the Stellar Sea" => Some(LightConeKind::Cruising_In_The_Stellar_Sea),
            "Dance! Dance! Dance!" => Some(LightConeKind::Dance_Dance_Dance),
            "Darting Arrow" => Some(LightConeKind::Darting_Arrow),
            "Data Bank" => Some(LightConeKind::Data_Bank),
            "Day One of My New Life" => Some(LightConeKind::Day_One_Of_My_New_Life),
            "Defense" => Some(LightConeKind::Defense),
            "Destiny's Threads Forewoven" => Some(LightConeKind::Destinys_Threads_Forewoven),
            "Dreamville Adventure" => Some(LightConeKind::Dreamville_Adventure),
            "Earthly Escapade" => Some(LightConeKind::Earthly_Escapade),
            "Echoes of the Coffin" => Some(LightConeKind::Echoes_Of_The_Coffin),
            "Eyes of the Prey" => Some(LightConeKind::Eyes_Of_The_Prey),
            "Fermata" => Some(LightConeKind::Fermata),
            "Final Victor" => Some(LightConeKind::Final_Victor),
            "Fine Fruit" => Some(LightConeKind::Fine_Fruit),
            "Flames Afar" => Some(LightConeKind::Flames_Afar),
            "Geniuses' Repose" => Some(LightConeKind::Geniuses_Repose),
            "Good Night and Sleep Well" => Some(LightConeKind::Good_Night_And_Sleep_Well),
            "Hey, Over Here" => Some(LightConeKind::Hey_Over_Here),
            "Hidden Shadow" => Some(LightConeKind::Hidden_Shadow),
            "I Shall Be My Own Sword" => Some(LightConeKind::I_Shall_Be_My_Own_Sword),
            "In the Name of the World" => Some(LightConeKind::In_The_Name_Of_The_World),
            "In the Night" => Some(LightConeKind::In_The_Night),
            "Incessant Rain" => Some(LightConeKind::Incessant_Rain),
            "Indelible Promise" => Some(LightConeKind::Indelible_Promise),
            "It's Showtime" => Some(LightConeKind::Its_Showtime),
            "Landau's Choice" => Some(LightConeKind::Landaus_Choice),
            "Loop" => Some(LightConeKind::Loop),
            "Make the World Clamor" => Some(LightConeKind::Make_The_World_Clamor),
            "Mediation" => Some(LightConeKind::Mediation),
            "Memories of the Past" => Some(LightConeKind::Memories_Of_The_Past),
            "Meshing Cogs" => Some(LightConeKind::Meshing_Cogs),
            "Moment of Victory" => Some(LightConeKind::Moment_Of_Victory),
            "Multiplication" => Some(LightConeKind::Multiplication),
            "Mutual Demise" => Some(LightConeKind::Mutual_Demise),
            "Night of Fright" => Some(LightConeKind::Night_Of_Fright),
            "Night on the Milky Way" => Some(LightConeKind::Night_On_The_Milky_Way),
            "Nowhere to Run" => Some(LightConeKind::Nowhere_To_Run),
            "On the Fall of an Aeon" => Some(LightConeKind::On_The_Fall_Of_An_Aeon),
            "Only Silence Remains" => Some(LightConeKind::Only_Silence_Remains),
            "Passkey" => Some(LightConeKind::Passkey),
            "Past Self in Mirror" => Some(LightConeKind::Past_Self_In_Mirror),
            "Past and Future" => Some(LightConeKind::Past_And_Future),
            "Patience Is All You Need" => Some(LightConeKind::Patience_Is_All_You_Need),
            "Perfect Timing" => Some(LightConeKind::Perfect_Timing),
            "Pioneering" => Some(LightConeKind::Pioneering),
            "Planetary Rendezvous" => Some(LightConeKind::Planetary_Rendezvous),
            "Post-Op Conversation" => Some(LightConeKind::Postop_Conversation),
            "Quid Pro Quo" => Some(LightConeKind::Quid_Pro_Quo),
            "Reforged Remembrance" => Some(LightConeKind::Reforged_Remembrance),
            "Resolution Shines As Pearls of Sweat" => {
                Some(LightConeKind::Resolution_Shines_As_Pearls_Of_Sweat)
            }
            "Return to Darkness" => Some(LightConeKind::Return_To_Darkness),
            "River Flows in Spring" => Some(LightConeKind::River_Flows_In_Spring),
            "Sagacity" => Some(LightConeKind::Sagacity),
            "Shared Feeling" => Some(LightConeKind::Shared_Feeling),
            "Shattered Home" => Some(LightConeKind::Shattered_Home),
            "She Already Shut Her Eyes" => Some(LightConeKind::She_Already_Shut_Her_Eyes),
            "Sleep Like the Dead" => Some(LightConeKind::Sleep_Like_The_Dead),
            "Solitary Healing" => Some(LightConeKind::Solitary_Healing),
            "Something Irreplaceable" => Some(LightConeKind::Something_Irreplaceable),
            "Subscribe for More!" => Some(LightConeKind::Subscribe_For_More),
            "Swordplay" => Some(LightConeKind::Swordplay),
            "Texture of Memories" => Some(LightConeKind::Texture_Of_Memories),
            "The Birth of the Self" => Some(LightConeKind::The_Birth_Of_The_Self),
            "The Day The Cosmos Fell" => Some(LightConeKind::The_Day_The_Cosmos_Fell),
            "The Moles Welcome You" => Some(LightConeKind::The_Moles_Welcome_You),
            "The Seriousness of Breakfast" => Some(LightConeKind::The_Seriousness_Of_Breakfast),
            "The Unreachable Side" => Some(LightConeKind::The_Unreachable_Side),
            "This Is Me!" => Some(LightConeKind::This_Is_Me),
            "Time Waits for No One" => Some(LightConeKind::Time_Waits_For_No_One),
            "Today Is Another Peaceful Day" => Some(LightConeKind::Today_Is_Another_Peaceful_Day),
            "Trend of the Universal Market" => Some(LightConeKind::Trend_Of_The_Universal_Market),
            "Under the Blue Sky" => Some(LightConeKind::Under_The_Blue_Sky),
            "Void" => Some(LightConeKind::Void),
            "Warmth Shortens Cold Nights" => Some(LightConeKind::Warmth_Shortens_Cold_Nights),
            "We Are Wildfire" => Some(LightConeKind::We_Are_Wildfire),
            "We Will Meet Again" => Some(LightConeKind::We_Will_Meet_Again),
            "What Is Real?" => Some(LightConeKind::What_Is_Real),
            "Woof! Walk Time!" => Some(LightConeKind::Woof_Walk_Time),
            "Worrisome, Blissful" => Some(LightConeKind::Worrisome_Blissful),
            _ => {
                debug!("not found: {}", str);
                None
            }
        }
    }
}

impl LightCone {
    pub fn new(
        kind: LightConeKind,
        level: u16,
        ascension: u16,
        superimposition: u8,
        id: usize,
    ) -> LightCone {
        let info = LIGHT_CONE_INFO[kind as usize];
        if !matches!(info.0, _kind) {
            panic!("light cone info formatted incorrectly!")
        }

        LightCone {
            id,
            kind,
            path: info.1,
            rarity: info.2,
            level,
            ascension,
            superimposition,
            stats: info.3,
            equipped: None,
        }
    }

    pub fn max_level(&self) -> u16 {
        self.ascension * 10 + 20
    }

    pub fn min_level(&self) -> u16 {
        if self.ascension == 0 {
            1
        } else {
            self.ascension * 10 + 10
        }
    }

    pub fn stats(&self) -> [f32; 3] {
        let mut stats = self.stats.base[self.ascension as usize];
        for i in 0..BaseStat::LENGTH {
            stats[i] += (self.level - self.min_level()) as f32 * self.stats.growth[i];
        }
        stats
    }
}

const LIGHT_CONE_INFO: [(LightConeKind, Path, u8, LightConeStats); LightConeKind::LENGTH] = [
    (
        LightConeKind::A_Secret_Vow,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [48.0000, 21.6000, 12.0000],
                [105.6000, 47.5200, 26.4000],
                [182.4000, 82.0800, 45.6000],
                [259.2000, 116.6400, 64.8000],
                [336.0000, 151.2000, 84.0000],
                [412.8000, 185.7600, 103.2000],
                [489.6000, 220.3200, 122.4000],
            ],
            growth: [7.2000, 3.2400, 1.8000],
        },
    ),
    (
        LightConeKind::Adversarial,
        Path::Hunt,
        3,
        LightConeStats {
            base: [
                [33.6000, 16.8000, 12.0000],
                [73.9200, 36.9600, 26.4000],
                [127.6800, 63.8400, 45.6000],
                [181.4400, 90.7200, 64.8000],
                [235.2000, 117.6000, 84.0000],
                [288.9600, 144.4800, 103.2000],
                [342.7200, 171.3600, 122.4000],
            ],
            growth: [5.0400, 2.5200, 1.8000],
        },
    ),
    (
        LightConeKind::Amber,
        Path::Preservation,
        3,
        LightConeStats {
            base: [
                [38.4000, 12.0000, 15.0000],
                [84.4800, 26.4000, 33.0000],
                [145.9200, 45.6000, 57.0000],
                [207.3600, 64.8000, 81.0000],
                [268.8000, 84.0000, 105.0000],
                [330.2400, 103.2000, 129.0000],
                [391.6800, 122.4000, 153.0000],
            ],
            growth: [5.7600, 1.8000, 2.2500],
        },
    ),
    (
        LightConeKind::An_Instant_Before_A_Gaze,
        Path::Erudition,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Arrows,
        Path::Hunt,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Baptism_Of_Pure_Thought,
        Path::Hunt,
        5,
        LightConeStats {
            base: [
                [43.2000, 26.4000, 24.0000],
                [95.0400, 58.0800, 52.8000],
                [164.1600, 100.3200, 91.2000],
                [233.2800, 142.5600, 129.6000],
                [302.4000, 184.8000, 168.0000],
                [371.5200, 227.0400, 206.4000],
                [440.6400, 269.2800, 244.8000],
            ],
            growth: [6.4800, 3.9600, 3.6000],
        },
    ),
    (
        LightConeKind::Before_Dawn,
        Path::Erudition,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Before_The_Tutorial_Mission_Starts,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Brighter_Than_The_Sun,
        Path::Destruction,
        5,
        LightConeStats {
            base: [
                [48.0000, 28.8000, 18.0000],
                [105.6000, 63.3600, 39.6000],
                [182.4000, 109.4400, 68.4000],
                [259.2000, 155.5200, 97.2000],
                [336.0000, 201.6000, 126.0000],
                [412.8000, 247.6800, 154.8000],
                [489.6000, 293.7600, 183.6000],
            ],
            growth: [7.2000, 4.3200, 2.7000],
        },
    ),
    (
        LightConeKind::But_The_Battle_Isnt_Over,
        Path::Harmony,
        5,
        LightConeStats {
            base: [
                [52.8000, 24.0000, 21.0000],
                [116.1600, 52.8000, 46.2000],
                [200.6400, 91.2000, 79.8000],
                [285.1200, 129.6000, 113.4000],
                [369.6000, 168.0000, 147.0000],
                [454.0800, 206.4000, 180.6000],
                [538.5600, 244.8000, 214.2000],
            ],
            growth: [7.9200, 3.6000, 3.1500],
        },
    ),
    (
        LightConeKind::Carve_The_Moon_Weave_The_Clouds,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Chorus,
        Path::Harmony,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Collapsing_Sky,
        Path::Destruction,
        3,
        LightConeStats {
            base: [
                [38.4000, 16.8000, 9.0000],
                [84.4800, 36.9600, 19.8000],
                [145.9200, 63.8400, 34.2000],
                [207.3600, 90.7200, 48.6000],
                [268.8000, 117.6000, 63.0000],
                [330.2400, 144.4800, 77.4000],
                [391.6800, 171.3600, 91.8000],
            ],
            growth: [5.7600, 2.5200, 1.3500],
        },
    ),
    (
        LightConeKind::Cornucopia,
        Path::Abundance,
        3,
        LightConeStats {
            base: [
                [43.2000, 12.0000, 12.0000],
                [95.0400, 26.4000, 26.4000],
                [164.1600, 45.6000, 45.6000],
                [233.2800, 64.8000, 64.8000],
                [302.4000, 84.0000, 84.0000],
                [371.5200, 103.2000, 103.2000],
                [440.6400, 122.4000, 122.4000],
            ],
            growth: [6.4800, 1.8000, 1.8000],
        },
    ),
    (
        LightConeKind::Cruising_In_The_Stellar_Sea,
        Path::Hunt,
        5,
        LightConeStats {
            base: [
                [43.2000, 24.0000, 21.0000],
                [95.0400, 52.8000, 46.2000],
                [164.1600, 91.2000, 79.8000],
                [233.2800, 129.6000, 113.4000],
                [302.4000, 168.0000, 147.0000],
                [371.5200, 206.4000, 180.6000],
                [440.6400, 244.8000, 214.2000],
            ],
            growth: [6.4800, 3.6000, 3.1500],
        },
    ),
    (
        LightConeKind::Dance_Dance_Dance,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Darting_Arrow,
        Path::Hunt,
        3,
        LightConeStats {
            base: [
                [33.6000, 16.8000, 12.0000],
                [73.9200, 36.9600, 26.4000],
                [127.6800, 63.8400, 45.6000],
                [181.4400, 90.7200, 64.8000],
                [235.2000, 117.6000, 84.0000],
                [288.9600, 144.4800, 103.2000],
                [342.7200, 171.3600, 122.4000],
            ],
            growth: [5.0400, 2.5200, 1.8000],
        },
    ),
    (
        LightConeKind::Data_Bank,
        Path::Erudition,
        3,
        LightConeStats {
            base: [
                [33.6000, 16.8000, 12.0000],
                [73.9200, 36.9600, 26.4000],
                [127.6800, 63.8400, 45.6000],
                [181.4400, 90.7200, 64.8000],
                [235.2000, 117.6000, 84.0000],
                [288.9600, 144.4800, 103.2000],
                [342.7200, 171.3600, 122.4000],
            ],
            growth: [5.0400, 2.5200, 1.8000],
        },
    ),
    (
        LightConeKind::Day_One_Of_My_New_Life,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [43.2000, 16.8000, 21.0000],
                [95.0400, 36.9600, 46.2000],
                [164.1600, 63.8400, 79.8000],
                [233.2800, 90.7200, 113.4000],
                [302.4000, 117.6000, 147.0000],
                [371.5200, 144.4800, 180.6000],
                [440.6400, 171.3600, 214.2000],
            ],
            growth: [6.4800, 2.5200, 3.1500],
        },
    ),
    (
        LightConeKind::Defense,
        Path::Preservation,
        3,
        LightConeStats {
            base: [
                [43.2000, 12.0000, 12.0000],
                [95.0400, 26.4000, 26.4000],
                [164.1600, 45.6000, 45.6000],
                [233.2800, 64.8000, 64.8000],
                [302.4000, 84.0000, 84.0000],
                [371.5200, 103.2000, 103.2000],
                [440.6400, 122.4000, 122.4000],
            ],
            growth: [6.4800, 1.8000, 1.8000],
        },
    ),
    (
        LightConeKind::Destinys_Threads_Forewoven,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [43.2000, 16.8000, 21.0000],
                [95.0400, 36.9600, 46.2000],
                [164.1600, 63.8400, 79.8000],
                [233.2800, 90.7200, 113.4000],
                [302.4000, 117.6000, 147.0000],
                [371.5200, 144.4800, 180.6000],
                [440.6400, 171.3600, 214.2000],
            ],
            growth: [6.4800, 2.5200, 3.1500],
        },
    ),
    (
        LightConeKind::Dreamville_Adventure,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Earthly_Escapade,
        Path::Harmony,
        5,
        LightConeStats {
            base: [
                [52.8000, 24.0000, 21.0000],
                [116.1600, 52.8000, 46.2000],
                [200.6400, 91.2000, 79.8000],
                [285.1200, 129.6000, 113.4000],
                [369.6000, 168.0000, 147.0000],
                [454.0800, 206.4000, 180.6000],
                [538.5600, 244.8000, 214.2000],
            ],
            growth: [7.9200, 3.6000, 3.1500],
        },
    ),
    (
        LightConeKind::Echoes_Of_The_Coffin,
        Path::Abundance,
        5,
        LightConeStats {
            base: [
                [52.8000, 26.4000, 18.0000],
                [116.1600, 58.0800, 39.6000],
                [200.6400, 100.3200, 68.4000],
                [285.1200, 142.5600, 97.2000],
                [369.6000, 184.8000, 126.0000],
                [454.0800, 227.0400, 154.8000],
                [538.5600, 269.2800, 183.6000],
            ],
            growth: [7.9200, 3.9600, 2.7000],
        },
    ),
    (
        LightConeKind::Eyes_Of_The_Prey,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Fermata,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Final_Victor,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Fine_Fruit,
        Path::Abundance,
        3,
        LightConeStats {
            base: [
                [43.2000, 14.4000, 9.0000],
                [95.0400, 31.6800, 19.8000],
                [164.1600, 54.7200, 34.2000],
                [233.2800, 77.7600, 48.6000],
                [302.4000, 100.8000, 63.0000],
                [371.5200, 123.8400, 77.4000],
                [440.6400, 146.8800, 91.8000],
            ],
            growth: [6.4800, 2.1600, 1.3500],
        },
    ),
    (
        LightConeKind::Flames_Afar,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [48.0000, 21.6000, 12.0000],
                [105.6000, 47.5200, 26.4000],
                [182.4000, 82.0800, 45.6000],
                [259.2000, 116.6400, 64.8000],
                [336.0000, 151.2000, 84.0000],
                [412.8000, 185.7600, 103.2000],
                [489.6000, 220.3200, 122.4000],
            ],
            growth: [7.2000, 3.2400, 1.8000],
        },
    ),
    (
        LightConeKind::Geniuses_Repose,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [38.4000, 21.6000, 18.0000],
                [84.4800, 47.5200, 39.6000],
                [145.9200, 82.0800, 68.4000],
                [207.3600, 116.6400, 97.2000],
                [268.8000, 151.2000, 126.0000],
                [330.2400, 185.7600, 154.8000],
                [391.6800, 220.3200, 183.6000],
            ],
            growth: [5.7600, 3.2400, 2.7000],
        },
    ),
    (
        LightConeKind::Good_Night_And_Sleep_Well,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Hey_Over_Here,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Hidden_Shadow,
        Path::Nihility,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::I_Shall_Be_My_Own_Sword,
        Path::Destruction,
        5,
        LightConeStats {
            base: [
                [52.8000, 26.4000, 18.0000],
                [116.1600, 58.0800, 39.6000],
                [200.6400, 100.3200, 68.4000],
                [285.1200, 142.5600, 97.2000],
                [369.6000, 184.8000, 126.0000],
                [454.0800, 227.0400, 154.8000],
                [538.5600, 269.2800, 183.6000],
            ],
            growth: [7.9200, 3.9600, 2.7000],
        },
    ),
    (
        LightConeKind::In_The_Name_Of_The_World,
        Path::Nihility,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::In_The_Night,
        Path::Hunt,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Incessant_Rain,
        Path::Nihility,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Indelible_Promise,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Its_Showtime,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [48.0000, 21.6000, 12.0000],
                [105.6000, 47.5200, 26.4000],
                [182.4000, 82.0800, 45.6000],
                [259.2000, 116.6400, 64.8000],
                [336.0000, 151.2000, 84.0000],
                [412.8000, 185.7600, 103.2000],
                [489.6000, 220.3200, 122.4000],
            ],
            growth: [7.2000, 3.2400, 1.8000],
        },
    ),
    (
        LightConeKind::Landaus_Choice,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Loop,
        Path::Nihility,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Make_The_World_Clamor,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [38.4000, 21.6000, 18.0000],
                [84.4800, 47.5200, 39.6000],
                [145.9200, 82.0800, 68.4000],
                [207.3600, 116.6400, 97.2000],
                [268.8000, 151.2000, 126.0000],
                [330.2400, 185.7600, 154.8000],
                [391.6800, 220.3200, 183.6000],
            ],
            growth: [5.7600, 3.2400, 2.7000],
        },
    ),
    (
        LightConeKind::Mediation,
        Path::Harmony,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Memories_Of_The_Past,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Meshing_Cogs,
        Path::Harmony,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Moment_Of_Victory,
        Path::Preservation,
        5,
        LightConeStats {
            base: [
                [48.0000, 21.6000, 27.0000],
                [105.6000, 47.5200, 59.4000],
                [182.4000, 82.0800, 102.6000],
                [259.2000, 116.6400, 145.8000],
                [336.0000, 151.2000, 189.0000],
                [412.8000, 185.7600, 232.2000],
                [489.6000, 220.3200, 275.4000],
            ],
            growth: [7.2000, 3.2400, 4.0500],
        },
    ),
    (
        LightConeKind::Multiplication,
        Path::Abundance,
        3,
        LightConeStats {
            base: [
                [43.2000, 14.4000, 9.0000],
                [95.0400, 31.6800, 19.8000],
                [164.1600, 54.7200, 34.2000],
                [233.2800, 77.7600, 48.6000],
                [302.4000, 100.8000, 63.0000],
                [371.5200, 123.8400, 77.4000],
                [440.6400, 146.8800, 91.8000],
            ],
            growth: [6.4800, 2.1600, 1.3500],
        },
    ),
    (
        LightConeKind::Mutual_Demise,
        Path::Destruction,
        3,
        LightConeStats {
            base: [
                [38.4000, 16.8000, 9.0000],
                [84.4800, 36.9600, 19.8000],
                [145.9200, 63.8400, 34.2000],
                [207.3600, 90.7200, 48.6000],
                [268.8000, 117.6000, 63.0000],
                [330.2400, 144.4800, 77.4000],
                [391.6800, 171.3600, 91.8000],
            ],
            growth: [5.7600, 2.5200, 1.3500],
        },
    ),
    (
        LightConeKind::Night_Of_Fright,
        Path::Abundance,
        5,
        LightConeStats {
            base: [
                [52.8000, 21.6000, 24.0000],
                [116.1600, 47.5200, 52.8000],
                [200.6400, 82.0800, 91.2000],
                [285.1200, 116.6400, 129.6000],
                [369.6000, 151.2000, 168.0000],
                [454.0800, 185.7600, 206.4000],
                [538.5600, 220.3200, 244.8000],
            ],
            growth: [7.9200, 3.2400, 3.6000],
        },
    ),
    (
        LightConeKind::Night_On_The_Milky_Way,
        Path::Erudition,
        5,
        LightConeStats {
            base: [
                [52.8000, 26.4000, 18.0000],
                [116.1600, 58.0800, 39.6000],
                [200.6400, 100.3200, 68.4000],
                [285.1200, 142.5600, 97.2000],
                [369.6000, 184.8000, 126.0000],
                [454.0800, 227.0400, 154.8000],
                [538.5600, 269.2800, 183.6000],
            ],
            growth: [7.9200, 3.9600, 2.7000],
        },
    ),
    (
        LightConeKind::Nowhere_To_Run,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [43.2000, 24.0000, 12.0000],
                [95.0400, 52.8000, 26.4000],
                [164.1600, 91.2000, 45.6000],
                [233.2800, 129.6000, 64.8000],
                [302.4000, 168.0000, 84.0000],
                [371.5200, 206.4000, 103.2000],
                [440.6400, 244.8000, 122.4000],
            ],
            growth: [6.4800, 3.6000, 1.8000],
        },
    ),
    (
        LightConeKind::On_The_Fall_Of_An_Aeon,
        Path::Destruction,
        5,
        LightConeStats {
            base: [
                [48.0000, 24.0000, 18.0000],
                [105.6000, 52.8000, 39.6000],
                [182.4000, 91.2000, 68.4000],
                [259.2000, 129.6000, 97.2000],
                [336.0000, 168.0000, 126.0000],
                [412.8000, 206.4000, 154.8000],
                [489.6000, 244.8000, 183.6000],
            ],
            growth: [7.2000, 3.6000, 2.7000],
        },
    ),
    (
        LightConeKind::Only_Silence_Remains,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Passkey,
        Path::Erudition,
        3,
        LightConeStats {
            base: [
                [33.6000, 16.8000, 12.0000],
                [73.9200, 36.9600, 26.4000],
                [127.6800, 63.8400, 45.6000],
                [181.4400, 90.7200, 64.8000],
                [235.2000, 117.6000, 84.0000],
                [288.9600, 144.4800, 103.2000],
                [342.7200, 171.3600, 122.4000],
            ],
            growth: [5.0400, 2.5200, 1.8000],
        },
    ),
    (
        LightConeKind::Past_Self_In_Mirror,
        Path::Harmony,
        5,
        LightConeStats {
            base: [
                [48.0000, 24.0000, 24.0000],
                [105.6000, 52.8000, 52.8000],
                [182.4000, 91.2000, 91.2000],
                [259.2000, 129.6000, 129.6000],
                [336.0000, 168.0000, 168.0000],
                [412.8000, 206.4000, 206.4000],
                [489.6000, 244.8000, 244.8000],
            ],
            growth: [7.2000, 3.6000, 3.6000],
        },
    ),
    (
        LightConeKind::Past_And_Future,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Patience_Is_All_You_Need,
        Path::Nihility,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Perfect_Timing,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Pioneering,
        Path::Preservation,
        3,
        LightConeStats {
            base: [
                [43.2000, 12.0000, 12.0000],
                [95.0400, 26.4000, 26.4000],
                [164.1600, 45.6000, 45.6000],
                [233.2800, 64.8000, 64.8000],
                [302.4000, 84.0000, 84.0000],
                [371.5200, 103.2000, 103.2000],
                [440.6400, 122.4000, 122.4000],
            ],
            growth: [6.4800, 1.8000, 1.8000],
        },
    ),
    (
        LightConeKind::Planetary_Rendezvous,
        Path::Harmony,
        4,
        LightConeStats {
            base: [
                [48.0000, 19.2000, 15.0000],
                [105.6000, 42.2400, 33.0000],
                [182.4000, 72.9600, 57.0000],
                [259.2000, 103.6800, 81.0000],
                [336.0000, 134.4000, 105.0000],
                [412.8000, 165.1200, 129.0000],
                [489.6000, 195.8400, 153.0000],
            ],
            growth: [7.2000, 2.8800, 2.2500],
        },
    ),
    (
        LightConeKind::Postop_Conversation,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [48.0000, 19.2000, 15.0000],
                [105.6000, 42.2400, 33.0000],
                [182.4000, 72.9600, 57.0000],
                [259.2000, 103.6800, 81.0000],
                [336.0000, 134.4000, 105.0000],
                [412.8000, 165.1200, 129.0000],
                [489.6000, 195.8400, 153.0000],
            ],
            growth: [7.2000, 2.8800, 2.2500],
        },
    ),
    (
        LightConeKind::Quid_Pro_Quo,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Reforged_Remembrance,
        Path::Nihility,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Resolution_Shines_As_Pearls_Of_Sweat,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Return_To_Darkness,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [38.4000, 24.0000, 15.0000],
                [84.4800, 52.8000, 33.0000],
                [145.9200, 91.2000, 57.0000],
                [207.3600, 129.6000, 81.0000],
                [268.8000, 168.0000, 105.0000],
                [330.2400, 206.4000, 129.0000],
                [391.6800, 244.8000, 153.0000],
            ],
            growth: [5.7600, 3.6000, 2.2500],
        },
    ),
    (
        LightConeKind::River_Flows_In_Spring,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [38.4000, 21.6000, 18.0000],
                [84.4800, 47.5200, 39.6000],
                [145.9200, 82.0800, 68.4000],
                [207.3600, 116.6400, 97.2000],
                [268.8000, 151.2000, 126.0000],
                [330.2400, 185.7600, 154.8000],
                [391.6800, 220.3200, 183.6000],
            ],
            growth: [5.7600, 3.2400, 2.7000],
        },
    ),
    (
        LightConeKind::Sagacity,
        Path::Erudition,
        3,
        LightConeStats {
            base: [
                [33.6000, 16.8000, 12.0000],
                [73.9200, 36.9600, 26.4000],
                [127.6800, 63.8400, 45.6000],
                [181.4400, 90.7200, 64.8000],
                [235.2000, 117.6000, 84.0000],
                [288.9600, 144.4800, 103.2000],
                [342.7200, 171.3600, 122.4000],
            ],
            growth: [5.0400, 2.5200, 1.8000],
        },
    ),
    (
        LightConeKind::Shared_Feeling,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [43.2000, 19.2000, 18.0000],
                [95.0400, 42.2400, 39.6000],
                [164.1600, 72.9600, 68.4000],
                [233.2800, 103.6800, 97.2000],
                [302.4000, 134.4000, 126.0000],
                [371.5200, 165.1200, 154.8000],
                [440.6400, 195.8400, 183.6000],
            ],
            growth: [6.4800, 2.8800, 2.7000],
        },
    ),
    (
        LightConeKind::Shattered_Home,
        Path::Destruction,
        3,
        LightConeStats {
            base: [
                [38.4000, 16.8000, 9.0000],
                [84.4800, 36.9600, 19.8000],
                [145.9200, 63.8400, 34.2000],
                [207.3600, 90.7200, 48.6000],
                [268.8000, 117.6000, 63.0000],
                [330.2400, 144.4800, 77.4000],
                [391.6800, 171.3600, 91.8000],
            ],
            growth: [5.7600, 2.5200, 1.3500],
        },
    ),
    (
        LightConeKind::She_Already_Shut_Her_Eyes,
        Path::Preservation,
        5,
        LightConeStats {
            base: [
                [57.6000, 19.2000, 24.0000],
                [126.7200, 42.2400, 52.8000],
                [218.8800, 72.9600, 91.2000],
                [311.0400, 103.6800, 129.6000],
                [403.2000, 134.4000, 168.0000],
                [495.3600, 165.1200, 206.4000],
                [587.5200, 195.8400, 244.8000],
            ],
            growth: [8.6400, 2.8800, 3.6000],
        },
    ),
    (
        LightConeKind::Sleep_Like_The_Dead,
        Path::Hunt,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
    (
        LightConeKind::Solitary_Healing,
        Path::Nihility,
        5,
        LightConeStats {
            base: [
                [48.0000, 24.0000, 18.0000],
                [105.6000, 52.8000, 39.6000],
                [182.4000, 91.2000, 68.4000],
                [259.2000, 129.6000, 97.2000],
                [336.0000, 168.0000, 126.0000],
                [412.8000, 206.4000, 154.8000],
                [489.6000, 244.8000, 183.6000],
            ],
            growth: [7.2000, 3.6000, 2.7000],
        },
    ),
    (
        LightConeKind::Something_Irreplaceable,
        Path::Destruction,
        5,
        LightConeStats {
            base: [
                [52.8000, 26.4000, 18.0000],
                [116.1600, 58.0800, 39.6000],
                [200.6400, 100.3200, 68.4000],
                [285.1200, 142.5600, 97.2000],
                [369.6000, 184.8000, 126.0000],
                [454.0800, 227.0400, 154.8000],
                [538.5600, 269.2800, 183.6000],
            ],
            growth: [7.9200, 3.9600, 2.7000],
        },
    ),
    (
        LightConeKind::Subscribe_For_More,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Swordplay,
        Path::Hunt,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Texture_Of_Memories,
        Path::Preservation,
        5,
        LightConeStats {
            base: [
                [48.0000, 19.2000, 24.0000],
                [105.6000, 42.2400, 52.8000],
                [182.4000, 72.9600, 91.2000],
                [259.2000, 103.6800, 129.6000],
                [336.0000, 134.4000, 168.0000],
                [412.8000, 165.1200, 206.4000],
                [489.6000, 195.8400, 244.8000],
            ],
            growth: [7.2000, 2.8800, 3.6000],
        },
    ),
    (
        LightConeKind::The_Birth_Of_The_Self,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::The_Day_The_Cosmos_Fell,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::The_Moles_Welcome_You,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [48.0000, 21.6000, 12.0000],
                [105.6000, 47.5200, 26.4000],
                [182.4000, 82.0800, 45.6000],
                [259.2000, 116.6400, 64.8000],
                [336.0000, 151.2000, 84.0000],
                [412.8000, 185.7600, 103.2000],
                [489.6000, 220.3200, 122.4000],
            ],
            growth: [7.2000, 3.2400, 1.8000],
        },
    ),
    (
        LightConeKind::The_Seriousness_Of_Breakfast,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [38.4000, 21.6000, 18.0000],
                [84.4800, 47.5200, 39.6000],
                [145.9200, 82.0800, 68.4000],
                [207.3600, 116.6400, 97.2000],
                [268.8000, 151.2000, 126.0000],
                [330.2400, 185.7600, 154.8000],
                [391.6800, 220.3200, 183.6000],
            ],
            growth: [5.7600, 3.2400, 2.7000],
        },
    ),
    (
        LightConeKind::The_Unreachable_Side,
        Path::Destruction,
        5,
        LightConeStats {
            base: [
                [57.6000, 26.4000, 15.0000],
                [126.7200, 58.0800, 33.0000],
                [218.8800, 100.3200, 57.0000],
                [311.0400, 142.5600, 81.0000],
                [403.2000, 184.8000, 105.0000],
                [495.3600, 227.0400, 129.0000],
                [587.5200, 269.2800, 153.0000],
            ],
            growth: [8.6400, 3.9600, 2.2500],
        },
    ),
    (
        LightConeKind::This_Is_Me,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [38.4000, 16.8000, 24.0000],
                [84.4800, 36.9600, 52.8000],
                [145.9200, 63.8400, 91.2000],
                [207.3600, 90.7200, 129.6000],
                [268.8000, 117.6000, 168.0000],
                [330.2400, 144.4800, 206.4000],
                [391.6800, 171.3600, 244.8000],
            ],
            growth: [5.7600, 2.5200, 3.6000],
        },
    ),
    (
        LightConeKind::Time_Waits_For_No_One,
        Path::Abundance,
        5,
        LightConeStats {
            base: [
                [57.6000, 21.6000, 21.0000],
                [126.7200, 47.5200, 46.2000],
                [218.8800, 82.0800, 79.8000],
                [311.0400, 116.6400, 113.4000],
                [403.2000, 151.2000, 147.0000],
                [495.3600, 185.7600, 180.6000],
                [587.5200, 220.3200, 214.2000],
            ],
            growth: [8.6400, 3.2400, 3.1500],
        },
    ),
    (
        LightConeKind::Today_Is_Another_Peaceful_Day,
        Path::Erudition,
        4,
        LightConeStats {
            base: [
                [38.4000, 24.0000, 15.0000],
                [84.4800, 52.8000, 33.0000],
                [145.9200, 91.2000, 57.0000],
                [207.3600, 129.6000, 81.0000],
                [268.8000, 168.0000, 105.0000],
                [330.2400, 206.4000, 129.0000],
                [391.6800, 244.8000, 153.0000],
            ],
            growth: [5.7600, 3.6000, 2.2500],
        },
    ),
    (
        LightConeKind::Trend_Of_The_Universal_Market,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [48.0000, 16.8000, 18.0000],
                [105.6000, 36.9600, 39.6000],
                [182.4000, 63.8400, 68.4000],
                [259.2000, 90.7200, 97.2000],
                [336.0000, 117.6000, 126.0000],
                [412.8000, 144.4800, 154.8000],
                [489.6000, 171.3600, 183.6000],
            ],
            growth: [7.2000, 2.5200, 2.7000],
        },
    ),
    (
        LightConeKind::Under_The_Blue_Sky,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Void,
        Path::Nihility,
        3,
        LightConeStats {
            base: [
                [38.4000, 14.4000, 12.0000],
                [84.4800, 31.6800, 26.4000],
                [145.9200, 54.7200, 45.6000],
                [207.3600, 77.7600, 64.8000],
                [268.8000, 100.8000, 84.0000],
                [330.2400, 123.8400, 103.2000],
                [391.6800, 146.8800, 122.4000],
            ],
            growth: [5.7600, 2.1600, 1.8000],
        },
    ),
    (
        LightConeKind::Warmth_Shortens_Cold_Nights,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [48.0000, 16.8000, 18.0000],
                [105.6000, 36.9600, 39.6000],
                [182.4000, 63.8400, 68.4000],
                [259.2000, 90.7200, 97.2000],
                [336.0000, 117.6000, 126.0000],
                [412.8000, 144.4800, 154.8000],
                [489.6000, 171.3600, 183.6000],
            ],
            growth: [7.2000, 2.5200, 2.7000],
        },
    ),
    (
        LightConeKind::We_Are_Wildfire,
        Path::Preservation,
        4,
        LightConeStats {
            base: [
                [33.6000, 21.6000, 21.0000],
                [73.9200, 47.5200, 46.2000],
                [127.6800, 82.0800, 79.8000],
                [181.4400, 116.6400, 113.4000],
                [235.2000, 151.2000, 147.0000],
                [288.9600, 185.7600, 180.6000],
                [342.7200, 220.3200, 214.2000],
            ],
            growth: [5.0400, 3.2400, 3.1500],
        },
    ),
    (
        LightConeKind::We_Will_Meet_Again,
        Path::Nihility,
        4,
        LightConeStats {
            base: [
                [38.4000, 24.0000, 15.0000],
                [84.4800, 52.8000, 33.0000],
                [145.9200, 91.2000, 57.0000],
                [207.3600, 129.6000, 81.0000],
                [268.8000, 168.0000, 105.0000],
                [330.2400, 206.4000, 129.0000],
                [391.6800, 244.8000, 153.0000],
            ],
            growth: [5.7600, 3.6000, 2.2500],
        },
    ),
    (
        LightConeKind::What_Is_Real,
        Path::Abundance,
        4,
        LightConeStats {
            base: [
                [48.0000, 19.2000, 15.0000],
                [105.6000, 42.2400, 33.0000],
                [182.4000, 72.9600, 57.0000],
                [259.2000, 103.6800, 81.0000],
                [336.0000, 134.4000, 105.0000],
                [412.8000, 165.1200, 129.0000],
                [489.6000, 195.8400, 153.0000],
            ],
            growth: [7.2000, 2.8800, 2.2500],
        },
    ),
    (
        LightConeKind::Woof_Walk_Time,
        Path::Destruction,
        4,
        LightConeStats {
            base: [
                [43.2000, 21.6000, 15.0000],
                [95.0400, 47.5200, 33.0000],
                [164.1600, 82.0800, 57.0000],
                [233.2800, 116.6400, 81.0000],
                [302.4000, 151.2000, 105.0000],
                [371.5200, 185.7600, 129.0000],
                [440.6400, 220.3200, 153.0000],
            ],
            growth: [6.4800, 3.2400, 2.2500],
        },
    ),
    (
        LightConeKind::Worrisome_Blissful,
        Path::Hunt,
        5,
        LightConeStats {
            base: [
                [48.0000, 26.4000, 21.0000],
                [105.6000, 58.0800, 46.2000],
                [182.4000, 100.3200, 79.8000],
                [259.2000, 142.5600, 113.4000],
                [336.0000, 184.8000, 147.0000],
                [412.8000, 227.0400, 180.6000],
                [489.6000, 269.2800, 214.2000],
            ],
            growth: [7.2000, 3.9600, 3.1500],
        },
    ),
];
