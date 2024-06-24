use enum_map::Enum;
use crate::app::hsr::basics::{Element, Path};
use super::{AdvancedStat, BaseStat, Stat, UnitKind};

pub struct UnitStats{
    pub rarity: u8,
    pub base: [[f32; BaseStat::LENGTH];7],
    pub growth: [f32; BaseStat::LENGTH]
}

pub const CHARACTER_INFO: [(UnitKind, Path, UnitStats, [(Stat, f32);10]); UnitKind::LENGTH] = [
    (
        UnitKind::Argenti,
        Path::Erudition,
        UnitStats{
            rarity: 5,
            base: [
                [
                    142.5600,
                    100.3200,
                    49.5000,
                    103.0000,
                ],
                [
                    199.5840,
                    140.4480,
                    69.3000,
                    103.0000,
                ],
                [
                    256.6080,
                    180.5760,
                    89.1000,
                    103.0000,
                ],
                [
                    313.6320,
                    220.7040,
                    108.9000,
                    103.0000,
                ],
                [
                    370.6560,
                    260.8320,
                    128.7000,
                    103.0000,
                ],
                [
                    427.6800,
                    300.9600,
                    148.5000,
                    103.0000,
                ],
                [
                    484.7040,
                    341.0880,
                    168.3000,
                    103.0000,
                ],
            ],
            growth: [
                7.1280,
                5.0160,
                2.4750,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0640),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0480),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0320),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Arlan,
        Path::Destruction,
        UnitStats{
            rarity: 4,
            base: [
                [
                    163.2000,
                    81.6000,
                    45.0000,
                    102.0000,
                ],
                [
                    228.4800,
                    114.2400,
                    63.0000,
                    102.0000,
                ],
                [
                    293.7600,
                    146.8800,
                    81.0000,
                    102.0000,
                ],
                [
                    359.0400,
                    179.5200,
                    99.0000,
                    102.0000,
                ],
                [
                    424.3200,
                    212.1600,
                    117.0000,
                    102.0000,
                ],
                [
                    489.6000,
                    244.8000,
                    135.0000,
                    102.0000,
                ],
                [
                    554.8800,
                    277.4400,
                    153.0000,
                    102.0000,
                ],
            ],
            growth: [
                8.1600,
                4.0800,
                2.2500,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Asta,
        Path::Harmony,
        UnitStats{
            rarity: 4,
            base: [
                [
                    139.2000,
                    69.6000,
                    63.0000,
                    106.0000,
                ],
                [
                    194.8800,
                    97.4400,
                    88.2000,
                    106.0000,
                ],
                [
                    250.5600,
                    125.2800,
                    113.4000,
                    106.0000,
                ],
                [
                    306.2400,
                    153.1200,
                    138.6000,
                    106.0000,
                ],
                [
                    361.9200,
                    180.9600,
                    163.8000,
                    106.0000,
                ],
                [
                    417.6000,
                    208.8000,
                    189.0000,
                    106.0000,
                ],
                [
                    473.2800,
                    236.6400,
                    214.2000,
                    106.0000,
                ],
            ],
            growth: [
                6.9600,
                3.4800,
                3.1500,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0640),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
        ]
    ),
    (
        UnitKind::Bailu,
        Path::Abundance,
        UnitStats{
            rarity: 5,
            base: [
                [
                    179.5200,
                    76.5600,
                    66.0000,
                    98.0000,
                ],
                [
                    251.3280,
                    107.1840,
                    92.4000,
                    98.0000,
                ],
                [
                    323.1360,
                    137.8080,
                    118.8000,
                    98.0000,
                ],
                [
                    394.9440,
                    168.4320,
                    145.2000,
                    98.0000,
                ],
                [
                    466.7520,
                    199.0560,
                    171.6000,
                    98.0000,
                ],
                [
                    538.5600,
                    229.6800,
                    198.0000,
                    98.0000,
                ],
                [
                    610.3680,
                    260.3040,
                    224.4000,
                    98.0000,
                ],
            ],
            growth: [
                8.9760,
                3.8280,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
        ]
    ),
    (
        UnitKind::Black_Swan,
        Path::Nihility,
        UnitStats{
            rarity: 5,
            base: [
                [
                    147.8400,
                    89.7600,
                    66.0000,
                    102.0000,
                ],
                [
                    206.9760,
                    125.6640,
                    92.4000,
                    102.0000,
                ],
                [
                    266.1120,
                    161.5680,
                    118.8000,
                    102.0000,
                ],
                [
                    325.2480,
                    197.4720,
                    145.2000,
                    102.0000,
                ],
                [
                    384.3840,
                    233.3760,
                    171.6000,
                    102.0000,
                ],
                [
                    443.5200,
                    269.2800,
                    198.0000,
                    102.0000,
                ],
                [
                    502.6560,
                    305.1840,
                    224.4000,
                    102.0000,
                ],
            ],
            growth: [
                7.3920,
                4.4880,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0640),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
        ]
    ),
    (
        UnitKind::Blade,
        Path::Destruction,
        UnitStats{
            rarity: 5,
            base: [
                [
                    184.8000,
                    73.9200,
                    66.0000,
                    97.0000,
                ],
                [
                    258.7200,
                    103.4880,
                    92.4000,
                    97.0000,
                ],
                [
                    332.6400,
                    133.0560,
                    118.8000,
                    97.0000,
                ],
                [
                    406.5600,
                    162.6240,
                    145.2000,
                    97.0000,
                ],
                [
                    480.4800,
                    192.1920,
                    171.6000,
                    97.0000,
                ],
                [
                    554.4000,
                    221.7600,
                    198.0000,
                    97.0000,
                ],
                [
                    628.3200,
                    251.3280,
                    224.4000,
                    97.0000,
                ],
            ],
            growth: [
                9.2400,
                3.6960,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
        ]
    ),
    (
        UnitKind::Bronya,
        Path::Harmony,
        UnitStats{
            rarity: 5,
            base: [
                [
                    168.9600,
                    79.2000,
                    72.6000,
                    99.0000,
                ],
                [
                    236.5440,
                    110.8800,
                    101.6400,
                    99.0000,
                ],
                [
                    304.1280,
                    142.5600,
                    130.6800,
                    99.0000,
                ],
                [
                    371.7120,
                    174.2400,
                    159.7200,
                    99.0000,
                ],
                [
                    439.2960,
                    205.9200,
                    188.7600,
                    99.0000,
                ],
                [
                    506.8800,
                    237.6000,
                    217.8000,
                    99.0000,
                ],
                [
                    574.4640,
                    269.2800,
                    246.8400,
                    99.0000,
                ],
            ],
            growth: [
                8.4480,
                3.9600,
                3.6300,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0480),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0640),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.1070),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0480),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0320),
        ]
    ),
    (
        UnitKind::Clara,
        Path::Destruction,
        UnitStats{
            rarity: 5,
            base: [
                [
                    168.9600,
                    100.3200,
                    66.0000,
                    90.0000,
                ],
                [
                    236.5440,
                    140.4480,
                    92.4000,
                    90.0000,
                ],
                [
                    304.1280,
                    180.5760,
                    118.8000,
                    90.0000,
                ],
                [
                    371.7120,
                    220.7040,
                    145.2000,
                    90.0000,
                ],
                [
                    439.2960,
                    260.8320,
                    171.6000,
                    90.0000,
                ],
                [
                    506.8800,
                    300.9600,
                    198.0000,
                    90.0000,
                ],
                [
                    574.4640,
                    341.0880,
                    224.4000,
                    90.0000,
                ],
            ],
            growth: [
                8.4480,
                5.0160,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0640),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Physical)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Dan_Heng,
        Path::Hunt,
        UnitStats{
            rarity: 4,
            base: [
                [
                    120.0000,
                    74.4000,
                    54.0000,
                    110.0000,
                ],
                [
                    168.0000,
                    104.1600,
                    75.6000,
                    110.0000,
                ],
                [
                    216.0000,
                    133.9200,
                    97.2000,
                    110.0000,
                ],
                [
                    264.0000,
                    163.6800,
                    118.8000,
                    110.0000,
                ],
                [
                    312.0000,
                    193.4400,
                    140.4000,
                    110.0000,
                ],
                [
                    360.0000,
                    223.2000,
                    162.0000,
                    110.0000,
                ],
                [
                    408.0000,
                    252.9600,
                    183.6000,
                    110.0000,
                ],
            ],
            growth: [
                6.0000,
                3.7200,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0320),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0640),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Wind)), 0.0320),
        ]
    ),
    (
        UnitKind::Dan_Heng_IL,
        Path::Destruction,
        UnitStats{
            rarity: 5,
            base: [
                [
                    168.9600,
                    95.0400,
                    49.5000,
                    102.0000,
                ],
                [
                    236.5440,
                    133.0560,
                    69.3000,
                    102.0000,
                ],
                [
                    304.1280,
                    171.0720,
                    89.1000,
                    102.0000,
                ],
                [
                    371.7120,
                    209.0880,
                    108.9000,
                    102.0000,
                ],
                [
                    439.2960,
                    247.1040,
                    128.7000,
                    102.0000,
                ],
                [
                    506.8800,
                    285.1200,
                    148.5000,
                    102.0000,
                ],
                [
                    574.4640,
                    323.1360,
                    168.3000,
                    102.0000,
                ],
            ],
            growth: [
                8.4480,
                4.7520,
                2.4750,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0320),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0640),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0480),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0480),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0320),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Dr_Ratio,
        Path::Hunt,
        UnitStats{
            rarity: 5,
            base: [
                [
                    142.5600,
                    105.6000,
                    62.7000,
                    103.0000,
                ],
                [
                    199.5840,
                    147.8400,
                    87.7800,
                    103.0000,
                ],
                [
                    256.6080,
                    190.0800,
                    112.8600,
                    103.0000,
                ],
                [
                    313.6320,
                    232.3200,
                    137.9400,
                    103.0000,
                ],
                [
                    370.6560,
                    274.5600,
                    163.0200,
                    103.0000,
                ],
                [
                    427.6800,
                    316.8000,
                    188.1000,
                    103.0000,
                ],
                [
                    484.7040,
                    359.0400,
                    213.1800,
                    103.0000,
                ],
            ],
            growth: [
                7.1280,
                5.2800,
                3.1350,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
        ]
    ),
    (
        UnitKind::Fu_Xuan,
        Path::Preservation,
        UnitStats{
            rarity: 5,
            base: [
                [
                    200.6400,
                    63.3600,
                    82.5000,
                    100.0000,
                ],
                [
                    280.8960,
                    88.7040,
                    115.5000,
                    100.0000,
                ],
                [
                    361.1520,
                    114.0480,
                    148.5000,
                    100.0000,
                ],
                [
                    441.4080,
                    139.3920,
                    181.5000,
                    100.0000,
                ],
                [
                    521.6640,
                    164.7360,
                    214.5000,
                    100.0000,
                ],
                [
                    601.9200,
                    190.0800,
                    247.5000,
                    100.0000,
                ],
                [
                    682.1760,
                    215.4240,
                    280.5000,
                    100.0000,
                ],
            ],
            growth: [
                10.0320,
                3.1680,
                4.1250,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Base(BaseStat::Hp), 0.0800),
        ]
    ),
    (
        UnitKind::Gepard,
        Path::Preservation,
        UnitStats{
            rarity: 5,
            base: [
                [
                    190.0800,
                    73.9200,
                    89.1000,
                    92.0000,
                ],
                [
                    266.1120,
                    103.4880,
                    124.7400,
                    92.0000,
                ],
                [
                    342.1440,
                    133.0560,
                    160.3800,
                    92.0000,
                ],
                [
                    418.1760,
                    162.6240,
                    196.0200,
                    92.0000,
                ],
                [
                    494.2080,
                    192.1920,
                    231.6600,
                    92.0000,
                ],
                [
                    570.2400,
                    221.7600,
                    267.3000,
                    92.0000,
                ],
                [
                    646.2720,
                    251.3280,
                    302.9400,
                    92.0000,
                ],
            ],
            growth: [
                9.5040,
                3.6960,
                4.4550,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0800),
        ]
    ),
    (
        UnitKind::Guinaifen,
        Path::Nihility,
        UnitStats{
            rarity: 4,
            base: [
                [
                    120.0000,
                    79.2000,
                    60.0000,
                    106.0000,
                ],
                [
                    168.0000,
                    110.8800,
                    84.0000,
                    106.0000,
                ],
                [
                    216.0000,
                    142.5600,
                    108.0000,
                    106.0000,
                ],
                [
                    264.0000,
                    174.2400,
                    132.0000,
                    106.0000,
                ],
                [
                    312.0000,
                    205.9200,
                    156.0000,
                    106.0000,
                ],
                [
                    360.0000,
                    237.6000,
                    180.0000,
                    106.0000,
                ],
                [
                    408.0000,
                    269.2800,
                    204.0000,
                    106.0000,
                ],
            ],
            growth: [
                6.0000,
                3.9600,
                3.0000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0640),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.1070),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0530),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
        ]
    ),
    (
        UnitKind::Hanya,
        Path::Harmony,
        UnitStats{
            rarity: 4,
            base: [
                [
                    124.8000,
                    76.8000,
                    48.0000,
                    110.0000,
                ],
                [
                    174.7200,
                    107.5200,
                    67.2000,
                    110.0000,
                ],
                [
                    224.6400,
                    138.2400,
                    86.4000,
                    110.0000,
                ],
                [
                    274.5600,
                    168.9600,
                    105.6000,
                    110.0000,
                ],
                [
                    324.4800,
                    199.6800,
                    124.8000,
                    110.0000,
                ],
                [
                    374.4000,
                    230.4000,
                    144.0000,
                    110.0000,
                ],
                [
                    424.3200,
                    261.1200,
                    163.2000,
                    110.0000,
                ],
            ],
            growth: [
                6.2400,
                3.8400,
                2.4000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Spd), 4.0000),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Spd), 3.0000),
            (Stat::Base(BaseStat::Spd), 2.0000),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Herta,
        Path::Erudition,
        UnitStats{
            rarity: 4,
            base: [
                [
                    129.6000,
                    79.2000,
                    54.0000,
                    100.0000,
                ],
                [
                    181.4400,
                    110.8800,
                    75.6000,
                    100.0000,
                ],
                [
                    233.2800,
                    142.5600,
                    97.2000,
                    100.0000,
                ],
                [
                    285.1200,
                    174.2400,
                    118.8000,
                    100.0000,
                ],
                [
                    336.9600,
                    205.9200,
                    140.4000,
                    100.0000,
                ],
                [
                    388.8000,
                    237.6000,
                    162.0000,
                    100.0000,
                ],
                [
                    440.6400,
                    269.2800,
                    183.6000,
                    100.0000,
                ],
            ],
            growth: [
                6.4800,
                3.9600,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
        ]
    ),
    (
        UnitKind::Himeko,
        Path::Erudition,
        UnitStats{
            rarity: 5,
            base: [
                [
                    142.5600,
                    102.9600,
                    59.4000,
                    96.0000,
                ],
                [
                    199.5840,
                    144.1440,
                    83.1600,
                    96.0000,
                ],
                [
                    256.6080,
                    185.3280,
                    106.9200,
                    96.0000,
                ],
                [
                    313.6320,
                    226.5120,
                    130.6800,
                    96.0000,
                ],
                [
                    370.6560,
                    267.6960,
                    154.4400,
                    96.0000,
                ],
                [
                    427.6800,
                    308.8800,
                    178.2000,
                    96.0000,
                ],
                [
                    484.7040,
                    350.0640,
                    201.9600,
                    96.0000,
                ],
            ],
            growth: [
                7.1280,
                5.1480,
                2.9700,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0640),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
        ]
    ),
    (
        UnitKind::Hook,
        Path::Destruction,
        UnitStats{
            rarity: 4,
            base: [
                [
                    182.4000,
                    84.0000,
                    48.0000,
                    94.0000,
                ],
                [
                    255.3600,
                    117.6000,
                    67.2000,
                    94.0000,
                ],
                [
                    328.3200,
                    151.2000,
                    86.4000,
                    94.0000,
                ],
                [
                    401.2800,
                    184.8000,
                    105.6000,
                    94.0000,
                ],
                [
                    474.2400,
                    218.4000,
                    124.8000,
                    94.0000,
                ],
                [
                    547.2000,
                    252.0000,
                    144.0000,
                    94.0000,
                ],
                [
                    620.1600,
                    285.6000,
                    163.2000,
                    94.0000,
                ],
            ],
            growth: [
                9.1200,
                4.2000,
                2.4000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
        ]
    ),
    (
        UnitKind::Huohuo,
        Path::Abundance,
        UnitStats{
            rarity: 5,
            base: [
                [
                    184.8000,
                    81.8400,
                    69.3000,
                    98.0000,
                ],
                [
                    258.7200,
                    114.5760,
                    97.0200,
                    98.0000,
                ],
                [
                    332.6400,
                    147.3120,
                    124.7400,
                    98.0000,
                ],
                [
                    406.5600,
                    180.0480,
                    152.4600,
                    98.0000,
                ],
                [
                    480.4800,
                    212.7840,
                    180.1800,
                    98.0000,
                ],
                [
                    554.4000,
                    245.5200,
                    207.9000,
                    98.0000,
                ],
                [
                    628.3200,
                    278.2560,
                    235.6200,
                    98.0000,
                ],
            ],
            growth: [
                9.2400,
                4.0920,
                3.4650,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0800),
            (Stat::Base(BaseStat::Spd), 3.0000),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Spd), 2.0000),
        ]
    ),
    (
        UnitKind::Jingliu,
        Path::Destruction,
        UnitStats{
            rarity: 5,
            base: [
                [
                    195.3600,
                    92.4000,
                    66.0000,
                    96.0000,
                ],
                [
                    273.5040,
                    129.3600,
                    92.4000,
                    96.0000,
                ],
                [
                    351.6480,
                    166.3200,
                    118.8000,
                    96.0000,
                ],
                [
                    429.7920,
                    203.2800,
                    145.2000,
                    96.0000,
                ],
                [
                    507.9360,
                    240.2400,
                    171.6000,
                    96.0000,
                ],
                [
                    586.0800,
                    277.2000,
                    198.0000,
                    96.0000,
                ],
                [
                    664.2240,
                    314.1600,
                    224.4000,
                    96.0000,
                ],
            ],
            growth: [
                9.7680,
                4.6200,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.1070),
            (Stat::Base(BaseStat::Spd), 4.0000),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Base(BaseStat::Spd), 3.0000),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Base(BaseStat::Spd), 2.0000),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Jing_Yuan,
        Path::Erudition,
        UnitStats{
            rarity: 5,
            base: [
                [
                    158.4000,
                    95.0400,
                    66.0000,
                    99.0000,
                ],
                [
                    221.7600,
                    133.0560,
                    92.4000,
                    99.0000,
                ],
                [
                    285.1200,
                    171.0720,
                    118.8000,
                    99.0000,
                ],
                [
                    348.4800,
                    209.0880,
                    145.2000,
                    99.0000,
                ],
                [
                    411.8400,
                    247.1040,
                    171.6000,
                    99.0000,
                ],
                [
                    475.2000,
                    285.1200,
                    198.0000,
                    99.0000,
                ],
                [
                    538.5600,
                    323.1360,
                    224.4000,
                    99.0000,
                ],
            ],
            growth: [
                7.9200,
                4.7520,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Kafka,
        Path::Nihility,
        UnitStats{
            rarity: 5,
            base: [
                [
                    147.8400,
                    92.4000,
                    66.0000,
                    100.0000,
                ],
                [
                    206.9760,
                    129.3600,
                    92.4000,
                    100.0000,
                ],
                [
                    266.1120,
                    166.3200,
                    118.8000,
                    100.0000,
                ],
                [
                    325.2480,
                    203.2800,
                    145.2000,
                    100.0000,
                ],
                [
                    384.3840,
                    240.2400,
                    171.6000,
                    100.0000,
                ],
                [
                    443.5200,
                    277.2000,
                    198.0000,
                    100.0000,
                ],
                [
                    502.6560,
                    314.1600,
                    224.4000,
                    100.0000,
                ],
            ],
            growth: [
                7.3920,
                4.6200,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Luka,
        Path::Nihility,
        UnitStats{
            rarity: 4,
            base: [
                [
                    124.8000,
                    79.2000,
                    66.0000,
                    103.0000,
                ],
                [
                    174.7200,
                    110.8800,
                    92.4000,
                    103.0000,
                ],
                [
                    224.6400,
                    142.5600,
                    118.8000,
                    103.0000,
                ],
                [
                    274.5600,
                    174.2400,
                    145.2000,
                    103.0000,
                ],
                [
                    324.4800,
                    205.9200,
                    171.6000,
                    103.0000,
                ],
                [
                    374.4000,
                    237.6000,
                    198.0000,
                    103.0000,
                ],
                [
                    424.3200,
                    269.2800,
                    224.4000,
                    103.0000,
                ],
            ],
            growth: [
                6.2400,
                3.9600,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0800),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
        ]
    ),
    (
        UnitKind::Luocha,
        Path::Abundance,
        UnitStats{
            rarity: 5,
            base: [
                [
                    174.2400,
                    102.9600,
                    49.5000,
                    101.0000,
                ],
                [
                    243.9360,
                    144.1440,
                    69.3000,
                    101.0000,
                ],
                [
                    313.6320,
                    185.3280,
                    89.1000,
                    101.0000,
                ],
                [
                    383.3280,
                    226.5120,
                    108.9000,
                    101.0000,
                ],
                [
                    453.0240,
                    267.6960,
                    128.7000,
                    101.0000,
                ],
                [
                    522.7200,
                    308.8800,
                    148.5000,
                    101.0000,
                ],
                [
                    592.4160,
                    350.0640,
                    168.3000,
                    101.0000,
                ],
            ],
            growth: [
                8.7120,
                5.1480,
                2.4750,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
        ]
    ),
    (
        UnitKind::Lynx,
        Path::Abundance,
        UnitStats{
            rarity: 4,
            base: [
                [
                    144.0000,
                    67.2000,
                    75.0000,
                    100.0000,
                ],
                [
                    201.6000,
                    94.0800,
                    105.0000,
                    100.0000,
                ],
                [
                    259.2000,
                    120.9600,
                    135.0000,
                    100.0000,
                ],
                [
                    316.8000,
                    147.8400,
                    165.0000,
                    100.0000,
                ],
                [
                    374.4000,
                    174.7200,
                    195.0000,
                    100.0000,
                ],
                [
                    432.0000,
                    201.6000,
                    225.0000,
                    100.0000,
                ],
                [
                    489.6000,
                    228.4800,
                    255.0000,
                    100.0000,
                ],
            ],
            growth: [
                7.2000,
                3.3600,
                3.7500,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
        ]
    ),
    (
        UnitKind::March_7th,
        Path::Preservation,
        UnitStats{
            rarity: 4,
            base: [
                [
                    144.0000,
                    69.6000,
                    78.0000,
                    101.0000,
                ],
                [
                    201.6000,
                    97.4400,
                    109.2000,
                    101.0000,
                ],
                [
                    259.2000,
                    125.2800,
                    140.4000,
                    101.0000,
                ],
                [
                    316.8000,
                    153.1200,
                    171.6000,
                    101.0000,
                ],
                [
                    374.4000,
                    180.9600,
                    202.8000,
                    101.0000,
                ],
                [
                    432.0000,
                    208.8000,
                    234.0000,
                    101.0000,
                ],
                [
                    489.6000,
                    236.6400,
                    265.2000,
                    101.0000,
                ],
            ],
            growth: [
                7.2000,
                3.4800,
                3.9000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
        ]
    ),
    (
        UnitKind::Misha,
        Path::Destruction,
        UnitStats{
            rarity: 4,
            base: [
                [
                    172.8000,
                    81.6000,
                    54.0000,
                    96.0000,
                ],
                [
                    241.9200,
                    114.2400,
                    75.6000,
                    96.0000,
                ],
                [
                    311.0400,
                    146.8800,
                    97.2000,
                    96.0000,
                ],
                [
                    380.1600,
                    179.5200,
                    118.8000,
                    96.0000,
                ],
                [
                    449.2800,
                    212.1600,
                    140.4000,
                    96.0000,
                ],
                [
                    518.4000,
                    244.8000,
                    162.0000,
                    96.0000,
                ],
                [
                    587.5200,
                    277.4400,
                    183.6000,
                    96.0000,
                ],
            ],
            growth: [
                8.6400,
                4.0800,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
        ]
    ),
    (
        UnitKind::Natasha,
        Path::Abundance,
        UnitStats{
            rarity: 4,
            base: [
                [
                    158.4000,
                    64.8000,
                    69.0000,
                    98.0000,
                ],
                [
                    221.7600,
                    90.7200,
                    96.6000,
                    98.0000,
                ],
                [
                    285.1200,
                    116.6400,
                    124.2000,
                    98.0000,
                ],
                [
                    348.4800,
                    142.5600,
                    151.8000,
                    98.0000,
                ],
                [
                    411.8400,
                    168.4800,
                    179.4000,
                    98.0000,
                ],
                [
                    475.2000,
                    194.4000,
                    207.0000,
                    98.0000,
                ],
                [
                    538.5600,
                    220.3200,
                    234.6000,
                    98.0000,
                ],
            ],
            growth: [
                7.9200,
                3.2400,
                3.4500,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0800),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
        ]
    ),
    (
        UnitKind::Pela,
        Path::Nihility,
        UnitStats{
            rarity: 4,
            base: [
                [
                    134.4000,
                    74.4000,
                    63.0000,
                    105.0000,
                ],
                [
                    188.1600,
                    104.1600,
                    88.2000,
                    105.0000,
                ],
                [
                    241.9200,
                    133.9200,
                    113.4000,
                    105.0000,
                ],
                [
                    295.6800,
                    163.6800,
                    138.6000,
                    105.0000,
                ],
                [
                    349.4400,
                    193.4400,
                    163.8000,
                    105.0000,
                ],
                [
                    403.2000,
                    223.2000,
                    189.0000,
                    105.0000,
                ],
                [
                    456.9600,
                    252.9600,
                    214.2000,
                    105.0000,
                ],
            ],
            growth: [
                6.7200,
                3.7200,
                3.1500,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
        ]
    ),
    (
        UnitKind::Trailblazer_P,
        Path::Destruction,
        UnitStats{
            rarity: 5,
            base: [
                [
                    163.6800,
                    84.4800,
                    62.7000,
                    100.0000,
                ],
                [
                    229.1520,
                    118.2720,
                    87.7800,
                    100.0000,
                ],
                [
                    294.6240,
                    152.0640,
                    112.8600,
                    100.0000,
                ],
                [
                    360.0960,
                    185.8560,
                    137.9400,
                    100.0000,
                ],
                [
                    425.5680,
                    219.6480,
                    163.0200,
                    100.0000,
                ],
                [
                    491.0400,
                    253.4400,
                    188.1000,
                    100.0000,
                ],
                [
                    556.5120,
                    287.2320,
                    213.1800,
                    100.0000,
                ],
            ],
            growth: [
                8.1840,
                4.2240,
                3.1350,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
        ]
    ),
    (
        UnitKind::Trailblazer_F,
        Path::Preservation,
        UnitStats{
            rarity: 5,
            base: [
                [
                    168.9600,
                    81.8400,
                    82.5000,
                    95.0000,
                ],
                [
                    236.5440,
                    114.5760,
                    115.5000,
                    95.0000,
                ],
                [
                    304.1280,
                    147.3120,
                    148.5000,
                    95.0000,
                ],
                [
                    371.7120,
                    180.0480,
                    181.5000,
                    95.0000,
                ],
                [
                    439.2960,
                    212.7840,
                    214.5000,
                    95.0000,
                ],
                [
                    506.8800,
                    245.5200,
                    247.5000,
                    95.0000,
                ],
                [
                    574.4640,
                    278.2560,
                    280.5000,
                    95.0000,
                ],
            ],
            growth: [
                8.4480,
                4.0920,
                4.1250,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Base(BaseStat::Atk), 0.0800),
        ]
    ),
    (
        UnitKind::Qingque,
        Path::Erudition,
        UnitStats{
            rarity: 4,
            base: [
                [
                    139.2000,
                    88.8000,
                    60.0000,
                    98.0000,
                ],
                [
                    194.8800,
                    124.3200,
                    84.0000,
                    98.0000,
                ],
                [
                    250.5600,
                    159.8400,
                    108.0000,
                    98.0000,
                ],
                [
                    306.2400,
                    195.3600,
                    132.0000,
                    98.0000,
                ],
                [
                    361.9200,
                    230.8800,
                    156.0000,
                    98.0000,
                ],
                [
                    417.6000,
                    266.4000,
                    180.0000,
                    98.0000,
                ],
                [
                    473.2800,
                    301.9200,
                    204.0000,
                    98.0000,
                ],
            ],
            growth: [
                6.9600,
                4.4400,
                3.0000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0640),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0480),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0320),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Ruan_Mei,
        Path::Harmony,
        UnitStats{
            rarity: 5,
            base: [
                [
                    147.8400,
                    89.7600,
                    66.0000,
                    104.0000,
                ],
                [
                    206.9760,
                    125.6640,
                    92.4000,
                    104.0000,
                ],
                [
                    266.1120,
                    161.5680,
                    118.8000,
                    104.0000,
                ],
                [
                    325.2480,
                    197.4720,
                    145.2000,
                    104.0000,
                ],
                [
                    384.3840,
                    233.3760,
                    171.6000,
                    104.0000,
                ],
                [
                    443.5200,
                    269.2800,
                    198.0000,
                    104.0000,
                ],
                [
                    502.6560,
                    305.1840,
                    224.4000,
                    104.0000,
                ],
            ],
            growth: [
                7.3920,
                4.4880,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0530),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0800),
            (Stat::Base(BaseStat::Spd), 2.0000),
            (Stat::Base(BaseStat::Spd), 3.0000),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.1070),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0800),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0530),
        ]
    ),
    (
        UnitKind::Sampo,
        Path::Nihility,
        UnitStats{
            rarity: 4,
            base: [
                [
                    139.2000,
                    84.0000,
                    54.0000,
                    102.0000,
                ],
                [
                    194.8800,
                    117.6000,
                    75.6000,
                    102.0000,
                ],
                [
                    250.5600,
                    151.2000,
                    97.2000,
                    102.0000,
                ],
                [
                    306.2400,
                    184.8000,
                    118.8000,
                    102.0000,
                ],
                [
                    361.9200,
                    218.4000,
                    140.4000,
                    102.0000,
                ],
                [
                    417.6000,
                    252.0000,
                    162.0000,
                    102.0000,
                ],
                [
                    473.2800,
                    285.6000,
                    183.6000,
                    102.0000,
                ],
            ],
            growth: [
                6.9600,
                4.2000,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
        ]
    ),
    (
        UnitKind::Seele,
        Path::Hunt,
        UnitStats{
            rarity: 5,
            base: [
                [
                    126.7200,
                    87.1200,
                    49.5000,
                    115.0000,
                ],
                [
                    177.4080,
                    121.9680,
                    69.3000,
                    115.0000,
                ],
                [
                    228.0960,
                    156.8160,
                    89.1000,
                    115.0000,
                ],
                [
                    278.7840,
                    191.6640,
                    108.9000,
                    115.0000,
                ],
                [
                    329.4720,
                    226.5120,
                    128.7000,
                    115.0000,
                ],
                [
                    380.1600,
                    261.3600,
                    148.5000,
                    115.0000,
                ],
                [
                    430.8480,
                    296.2080,
                    168.3000,
                    115.0000,
                ],
            ],
            growth: [
                6.3360,
                4.3560,
                2.4750,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.1070),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Serval,
        Path::Erudition,
        UnitStats{
            rarity: 4,
            base: [
                [
                    124.8000,
                    88.8000,
                    51.0000,
                    104.0000,
                ],
                [
                    174.7200,
                    124.3200,
                    71.4000,
                    104.0000,
                ],
                [
                    224.6400,
                    159.8400,
                    91.8000,
                    104.0000,
                ],
                [
                    274.5600,
                    195.3600,
                    112.2000,
                    104.0000,
                ],
                [
                    324.4800,
                    230.8800,
                    132.6000,
                    104.0000,
                ],
                [
                    374.4000,
                    266.4000,
                    153.0000,
                    104.0000,
                ],
                [
                    424.3200,
                    301.9200,
                    173.4000,
                    104.0000,
                ],
            ],
            growth: [
                6.2400,
                4.4400,
                2.5500,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0800),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
        ]
    ),
    (
        UnitKind::Silver_Wolf,
        Path::Nihility,
        UnitStats{
            rarity: 5,
            base: [
                [
                    142.5600,
                    87.1200,
                    62.7000,
                    107.0000,
                ],
                [
                    199.5840,
                    121.9680,
                    87.7800,
                    107.0000,
                ],
                [
                    256.6080,
                    156.8160,
                    112.8600,
                    107.0000,
                ],
                [
                    313.6320,
                    191.6640,
                    137.9400,
                    107.0000,
                ],
                [
                    370.6560,
                    226.5120,
                    163.0200,
                    107.0000,
                ],
                [
                    427.6800,
                    261.3600,
                    188.1000,
                    107.0000,
                ],
                [
                    484.7040,
                    296.2080,
                    213.1800,
                    107.0000,
                ],
            ],
            growth: [
                7.1280,
                4.3560,
                3.1350,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectHitRate), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
        ]
    ),
    (
        UnitKind::Sparkle,
        Path::Harmony,
        UnitStats{
            rarity: 5,
            base: [
                [
                    190.0800,
                    71.2800,
                    66.0000,
                    101.0000,
                ],
                [
                    266.1120,
                    99.7920,
                    92.4000,
                    101.0000,
                ],
                [
                    342.1440,
                    128.3040,
                    118.8000,
                    101.0000,
                ],
                [
                    418.1760,
                    156.8160,
                    145.2000,
                    101.0000,
                ],
                [
                    494.2080,
                    185.3280,
                    171.6000,
                    101.0000,
                ],
                [
                    570.2400,
                    213.8400,
                    198.0000,
                    101.0000,
                ],
                [
                    646.2720,
                    242.3520,
                    224.4000,
                    101.0000,
                ],
            ],
            growth: [
                9.5040,
                3.5640,
                3.3000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.1070),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0800),
            (Stat::Advanced(AdvancedStat::CritDamage), 0.0530),
            (Stat::Base(BaseStat::Hp), 0.0400),
        ]
    ),
    (
        UnitKind::Sushang,
        Path::Hunt,
        UnitStats{
            rarity: 4,
            base: [
                [
                    124.8000,
                    76.8000,
                    57.0000,
                    107.0000,
                ],
                [
                    174.7200,
                    107.5200,
                    79.8000,
                    107.0000,
                ],
                [
                    224.6400,
                    138.2400,
                    102.6000,
                    107.0000,
                ],
                [
                    274.5600,
                    168.9600,
                    125.4000,
                    107.0000,
                ],
                [
                    324.4800,
                    199.6800,
                    148.2000,
                    107.0000,
                ],
                [
                    374.4000,
                    230.4000,
                    171.0000,
                    107.0000,
                ],
                [
                    424.3200,
                    261.1200,
                    193.8000,
                    107.0000,
                ],
            ],
            growth: [
                6.2400,
                3.8400,
                2.8500,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Tingyun,
        Path::Harmony,
        UnitStats{
            rarity: 4,
            base: [
                [
                    115.2000,
                    72.0000,
                    54.0000,
                    112.0000,
                ],
                [
                    161.2800,
                    100.8000,
                    75.6000,
                    112.0000,
                ],
                [
                    207.3600,
                    129.6000,
                    97.2000,
                    112.0000,
                ],
                [
                    253.4400,
                    158.4000,
                    118.8000,
                    112.0000,
                ],
                [
                    299.5200,
                    187.2000,
                    140.4000,
                    112.0000,
                ],
                [
                    345.6000,
                    216.0000,
                    162.0000,
                    112.0000,
                ],
                [
                    391.6800,
                    244.8000,
                    183.6000,
                    112.0000,
                ],
            ],
            growth: [
                5.7600,
                3.6000,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Def), 0.0750),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Lightning)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Lightning)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Base(BaseStat::Def), 0.1000),
            (Stat::Base(BaseStat::Def), 0.0500),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Topaz,
        Path::Hunt,
        UnitStats{
            rarity: 5,
            base: [
                [
                    126.7200,
                    84.4800,
                    56.1000,
                    110.0000,
                ],
                [
                    177.4080,
                    118.2720,
                    78.5400,
                    110.0000,
                ],
                [
                    228.0960,
                    152.0640,
                    100.9800,
                    110.0000,
                ],
                [
                    278.7840,
                    185.8560,
                    123.4200,
                    110.0000,
                ],
                [
                    329.4720,
                    219.6480,
                    145.8600,
                    110.0000,
                ],
                [
                    380.1600,
                    253.4400,
                    168.3000,
                    110.0000,
                ],
                [
                    430.8480,
                    287.2320,
                    190.7400,
                    110.0000,
                ],
            ],
            growth: [
                6.3360,
                4.2240,
                2.8050,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0640),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0530),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0480),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0400),
            (Stat::Advanced(AdvancedStat::CritRate), 0.0270),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Fire)), 0.0320),
        ]
    ),
    (
        UnitKind::Welt,
        Path::Nihility,
        UnitStats{
            rarity: 5,
            base: [
                [
                    153.1200,
                    84.4800,
                    69.3000,
                    102.0000,
                ],
                [
                    214.3680,
                    118.2720,
                    97.0200,
                    102.0000,
                ],
                [
                    275.6160,
                    152.0640,
                    124.7400,
                    102.0000,
                ],
                [
                    336.8640,
                    185.8560,
                    152.4600,
                    102.0000,
                ],
                [
                    398.1120,
                    219.6480,
                    180.1800,
                    102.0000,
                ],
                [
                    459.3600,
                    253.4400,
                    207.9000,
                    102.0000,
                ],
                [
                    520.6080,
                    287.2320,
                    235.6200,
                    102.0000,
                ],
            ],
            growth: [
                7.6560,
                4.2240,
                3.4650,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0640),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Advanced(AdvancedStat::EffectRes), 0.0400),
        ]
    ),
    (
        UnitKind::Xueyi,
        Path::Destruction,
        UnitStats{
            rarity: 4,
            base: [
                [
                    144.0000,
                    81.6000,
                    54.0000,
                    103.0000,
                ],
                [
                    201.6000,
                    114.2400,
                    75.6000,
                    103.0000,
                ],
                [
                    259.2000,
                    146.8800,
                    97.2000,
                    103.0000,
                ],
                [
                    316.8000,
                    179.5200,
                    118.8000,
                    103.0000,
                ],
                [
                    374.4000,
                    212.1600,
                    140.4000,
                    103.0000,
                ],
                [
                    432.0000,
                    244.8000,
                    162.0000,
                    103.0000,
                ],
                [
                    489.6000,
                    277.4400,
                    183.6000,
                    103.0000,
                ],
            ],
            growth: [
                7.2000,
                4.0800,
                2.7000,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0480),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.1070),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0530),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0800),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::BreakEffect), 0.0530),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Quantum)), 0.0320),
        ]
    ),
    (
        UnitKind::Yanqing,
        Path::Hunt,
        UnitStats{
            rarity: 5,
            base: [
                [
                    121.4400,
                    92.4000,
                    56.1000,
                    109.0000,
                ],
                [
                    170.0160,
                    129.3600,
                    78.5400,
                    109.0000,
                ],
                [
                    218.5920,
                    166.3200,
                    100.9800,
                    109.0000,
                ],
                [
                    267.1680,
                    203.2800,
                    123.4200,
                    109.0000,
                ],
                [
                    315.7440,
                    240.2400,
                    145.8600,
                    109.0000,
                ],
                [
                    364.3200,
                    277.2000,
                    168.3000,
                    109.0000,
                ],
                [
                    412.8960,
                    314.1600,
                    190.7400,
                    109.0000,
                ],
            ],
            growth: [
                6.0720,
                4.6200,
                2.8050,
                0.0000,
            ]
        },
        [
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Atk), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0640),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0480),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Ice)), 0.0320),
            (Stat::Base(BaseStat::Atk), 0.0400),
        ]
    ),
    (
        UnitKind::Yukong,
        Path::Harmony,
        UnitStats{
            rarity: 4,
            base: [
                [
                    124.8000,
                    81.6000,
                    51.0000,
                    107.0000,
                ],
                [
                    174.7200,
                    114.2400,
                    71.4000,
                    107.0000,
                ],
                [
                    224.6400,
                    146.8800,
                    91.8000,
                    107.0000,
                ],
                [
                    274.5600,
                    179.5200,
                    112.2000,
                    107.0000,
                ],
                [
                    324.4800,
                    212.1600,
                    132.6000,
                    107.0000,
                ],
                [
                    374.4000,
                    244.8000,
                    153.0000,
                    107.0000,
                ],
                [
                    424.3200,
                    277.4400,
                    173.4000,
                    107.0000,
                ],
            ],
            growth: [
                6.2400,
                4.0800,
                2.5500,
                0.0000,
            ]
        },
        [
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0320),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0480),
            (Stat::Base(BaseStat::Atk), 0.0400),
            (Stat::Base(BaseStat::Atk), 0.0600),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0640),
            (Stat::Base(BaseStat::Hp), 0.0800),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0480),
            (Stat::Base(BaseStat::Hp), 0.0600),
            (Stat::Base(BaseStat::Hp), 0.0400),
            (Stat::Advanced(AdvancedStat::ElemDmgBoost(Element::Imaginary)), 0.0320),
        ]
    ),
];