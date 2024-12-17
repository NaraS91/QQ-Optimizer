use std::str::FromStr;

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
    Abundance,
}

#[derive(Enum, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
    Wind,
    Quantum,
    Imaginary,
}

impl Element {
    pub fn unique_id(&self) -> usize {
        match self {
            Element::Physical => 0,
            Element::Fire => 1,
            Element::Ice => 2,
            Element::Lightning => 3,
            Element::Wind => 4,
            Element::Quantum => 5,
            Element::Imaginary => 6,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Element::Physical => "Physical".to_owned(),
            Element::Fire => "Fire".to_owned(),
            Element::Ice => "Ice".to_owned(),
            Element::Lightning => "Lightning".to_owned(),
            Element::Wind => "Wind".to_owned(),
            Element::Quantum => "Quantum".to_owned(),
            Element::Imaginary => "Imaginary".to_owned(),
        }
    }
}

impl FromStr for Path {
    type Err = String;
    fn from_str(input: &str) -> Result<Path, Self::Err> {
        match input {
            "Destruction" => Ok(Path::Destruction),
            "The Hunt" => Ok(Path::Hunt),
            "Erudition" => Ok(Path::Erudition),
            "Harmony" => Ok(Path::Harmony),
            "Nihility" => Ok(Path::Nihility),
            "Preservation" => Ok(Path::Preservation),
            "Abundance" => Ok(Path::Abundance),
            _ => Err(format!("oopsie, {} not found", input)),
        }
    }
}
