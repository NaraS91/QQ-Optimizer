use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Path {
    Destruction,
    Hunt,
    Erudition,
    Harmony,
    Nihility,
    Preservation,
    Abundance
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
    Wind,
    Quantum,
    Imaginary,
}