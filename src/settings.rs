// Copyright 2023, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

// Airspace types
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumString, PartialEq, Serialize)]
pub enum AirType {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
    ClassF,
    ClassG,
    Danger,
    Cta,
    Ctr,
    Gliding,
    Matz,
    Other,
    Prohibited,
    Restricted,
    Rmz,
    Tmz,
}

// Output format
#[derive(Clone, Debug, Deserialize, Display, EnumString, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

// Altutude layer overlay
#[derive(Clone, Debug, Deserialize, Display, EnumString, PartialEq, Serialize)]
pub enum Overlay {
    FL195,
    FL105,
    AtzDz,
}

// Settings
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub atz: AirType,
    pub ils: Option<AirType>,
    pub unlicensed: Option<AirType>,
    pub microlight: Option<AirType>,
    pub gliding: Option<AirType>,
    pub home: Option<String>,
    pub hirta_gvs: Option<AirType>,
    pub obstacle: Option<AirType>,
    pub max_level: u16,
    pub radio: bool,
    pub format: Format,
    pub overlay: Option<Overlay>,
    #[serde(default)]
    pub loa: HashSet<String>,
    #[serde(default)]
    pub rat: HashSet<String>,
    #[serde(default)]
    pub wave: HashSet<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            atz: AirType::Ctr,
            ils: None,
            unlicensed: None,
            microlight: None,
            gliding: None,
            home: None,
            hirta_gvs: None,
            obstacle: None,
            max_level: 660,
            radio: false,
            format: Format::OpenAir,
            overlay: None,
            loa: HashSet::new(),
            rat: HashSet::new(),
            wave: HashSet::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtraType {
    Rat,
    Loa,
    Wave,
}

impl Settings {
    pub fn update(&mut self, name: &str, value: &str) {
        match name {
            "atz" => self.atz = AirType::from_str(value).unwrap_or(AirType::Cta),
            "ils" => self.ils = AirType::from_str(value).ok(),
            "unlicensed" => self.unlicensed = AirType::from_str(value).ok(),
            "microlight" => self.microlight = AirType::from_str(value).ok(),
            "gliding" => self.gliding = AirType::from_str(value).ok(),
            "hirta_gvs" => self.hirta_gvs = AirType::from_str(value).ok(),
            "obstacle" => self.obstacle = AirType::from_str(value).ok(),
            "format" => self.format = Format::from_str(value).unwrap_or(Format::OpenAir),
            "max_level" => self.max_level = value.parse().unwrap_or(660),
            "radio" => self.radio = value == "yes",
            "overlay" => self.overlay = Overlay::from_str(value).ok(),
            "home" => {
                self.home = if value == "no" {
                    None
                } else {
                    Some(value.to_string())
                }
            }
            _ => (),
        }
    }

    pub fn set_extra(&mut self, id: ExtraType, value: String, add: bool) {
        let x = match id {
            ExtraType::Rat => &mut self.rat,
            ExtraType::Loa => &mut self.loa,
            ExtraType::Wave => &mut self.wave,
        };

        if add {
            x.insert(value);
        } else {
            x.remove(&value);
        }
    }

    pub fn get_extra(&self, id: ExtraType) -> &HashSet<String> {
        match id {
            ExtraType::Rat => &self.rat,
            ExtraType::Loa => &self.loa,
            ExtraType::Wave => &self.wave,
        }
    }

    pub fn clear_extra(&mut self, id: ExtraType) {
        match id {
            ExtraType::Rat => &self.rat.clear(),
            ExtraType::Loa => &self.loa.clear(),
            ExtraType::Wave => &self.wave.clear(),
        };
    }
}
