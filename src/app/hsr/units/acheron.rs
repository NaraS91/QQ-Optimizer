use super::{ModifierOrDOT, Unit};

pub fn modifiers(unit: &Unit) -> Vec<ModifierOrDOT> {
    vec![]
}

const SKILL_PARAMS: [(f32, f32, f32); 15] = [
    (0.8000, 0.3000, 1.0000),
    (0.8800, 0.3300, 1.0000),
    (0.9600, 0.3600, 1.0000),
    (1.0400, 0.3900, 1.0000),
    (1.1200, 0.4200, 1.0000),
    (1.2000, 0.4500, 1.0000),
    (1.3000, 0.4875, 1.0000),
    (1.4000, 0.5250, 1.0000),
    (1.5000, 0.5625, 1.0000),
    (1.6000, 0.6000, 1.0000),
    (1.6800, 0.6300, 1.0000),
    (1.7600, 0.6600, 1.0000),
    (1.8400, 0.6900, 1.0000),
    (1.9200, 0.7200, 1.0000),
    (2.0000, 0.7500, 1.0000),
];

const ULT_PARAMS: [(f32, f32, f32, f32, f32, f32, f32); 15] = [
    (0.1440, 0.0900, 0.7200, 9.0000, 0.3600, 2.2320, 1.8000),
    (0.1536, 0.0960, 0.7680, 9.0000, 0.3840, 2.3808, 1.9200),
    (0.1632, 0.1020, 0.8160, 9.0000, 0.4080, 2.5296, 2.0400),
    (0.1728, 0.1080, 0.8640, 9.0000, 0.4320, 2.6784, 2.1600),
    (0.1824, 0.1140, 0.9120, 9.0000, 0.4560, 2.8272, 2.2800),
    (0.1920, 0.1200, 0.9600, 9.0000, 0.4800, 2.9760, 2.4000),
    (0.2040, 0.1275, 1.0200, 9.0000, 0.5100, 3.1620, 2.5500),
    (0.2160, 0.1350, 1.0800, 9.0000, 0.5400, 3.3480, 2.7000),
    (0.2280, 0.1425, 1.1400, 9.0000, 0.5700, 3.5340, 2.8500),
    (0.2400, 0.1500, 1.2000, 9.0000, 0.6000, 3.7200, 3.0000),
    (0.2496, 0.1560, 1.2480, 9.0000, 0.6240, 3.8688, 3.1200),
    (0.2592, 0.1620, 1.2960, 9.0000, 0.6480, 4.0176, 3.2400),
    (0.2688, 0.1680, 1.3440, 9.0000, 0.6720, 4.1664, 3.3600),
    (0.2784, 0.1740, 1.3920, 9.0000, 0.6960, 4.3152, 3.4800),
    (0.2880, 0.1800, 1.4400, 9.0000, 0.7200, 4.4640, 3.6000),
];

const TALENT_PARAMS: [(f32, f32); 15] = [
    (9.0000, 0.1000),
    (9.0000, 0.1100),
    (9.0000, 0.1200),
    (9.0000, 0.1300),
    (9.0000, 0.1400),
    (9.0000, 0.1500),
    (9.0000, 0.1625),
    (9.0000, 0.1750),
    (9.0000, 0.1875),
    (9.0000, 0.2000),
    (9.0000, 0.2100),
    (9.0000, 0.2200),
    (9.0000, 0.2300),
    (9.0000, 0.2400),
    (9.0000, 0.2500),
];

const TECH_PARAMS: [(f32, f32); 1] = [(2.0000, 1.0000)];

const BASIC_PARAMS: [f32; 9] = [
    0.5000, 0.6000, 0.7000, 0.8000, 0.9000, 1.0000, 1.1000, 1.2000, 1.3000,
];