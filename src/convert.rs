// Copyright 2024, Alan Sparrow
//_
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
use crate::settings::{AirType, Format, Settings};
use crate::yaixm::{
    latlon_to_degrees, radius_to_metres, Arc, Boundary, Circle, Feature, IcaoClass, IcaoType, Loa,
    LocalType, Obstacle, Rule, Service, Volume, Yaixm,
};
use chrono::Utc;
use geo::{Bearing, Destination, Geodesic, Point};
use std::collections::{HashMap, HashSet};

impl LocalType {
    fn as_str(&self) -> &'static str {
        match self {
            LocalType::Dz => "DZ",
            LocalType::Glider => "GLIDER",
            LocalType::Gvs => "GVS",
            LocalType::Hirta => "HIRTA",
            LocalType::Ils => "ILS",
            LocalType::Laser => "LASER",
            LocalType::Matz => "MATZ",
            LocalType::NoAtz => "NOATZ",
            LocalType::Obstacle => "OBSTACLE",
            LocalType::Rat => "RAT",
            LocalType::Rmz => "RMZ",
            LocalType::Ul => "UL",
            LocalType::Tmz => "TMZ",
        }
    }
}

impl Rule {
    fn as_str(&self) -> &'static str {
        match self {
            Rule::Intense => "INTENSE",
            Rule::Loa => "LOA",
            Rule::NoSsr => "NOSSR",
            Rule::Notam => "NOTAM",
            Rule::Raz => "RAZ",
            Rule::Rmz => "RMZ",
            Rule::Si => "SI",
            Rule::Tra => "TRA",
            Rule::Tmz => "TMZ",
        }
    }
}

impl AirType {
    fn as_str(&self) -> &'static str {
        match self {
            AirType::ClassA => "A",
            AirType::ClassB => "B",
            AirType::ClassC => "C",
            AirType::ClassD => "D",
            AirType::ClassE => "E",
            AirType::ClassF => "F",
            AirType::ClassG => "G",
            AirType::Prohibited => "P",
            AirType::Danger => "Q",
            AirType::Restricted => "R",
            AirType::Gliding => "W",
            AirType::Cta => "CTA",
            AirType::Ctr => "CTR",
            AirType::Matz => "MATZ",
            AirType::Other => "OTHER",
            AirType::Rmz => "RMZ",
            AirType::Tmz => "RMZ",
        }
    }
}

// Normalise all levels to flight level
fn norm_level(value: &str) -> u16 {
    if let Some(fl) = value.strip_prefix("FL") {
        fl.parse().unwrap()
    } else if value.ends_with(" ft") {
        value.split(' ').next().unwrap().parse::<u16>().unwrap() / 100
    } else {
        0
    }
}

// Openair level format
fn format_level(level: &str) -> String {
    if let Some(alt) = level.strip_suffix(" ft") {
        // Altitude
        alt.to_string() + " ft"
    } else {
        // Flight level or SFC
        level.to_string()
    }
}

fn resolution(airtype: AirType) -> u32 {
    match airtype {
        AirType::ClassA => 72,
        AirType::ClassB => 72,
        AirType::ClassC => 72,
        AirType::ClassD => 72,
        AirType::ClassE => 72,
        AirType::ClassF => 72,
        AirType::ClassG => 72,
        AirType::Prohibited => 72,
        AirType::Danger => 72,
        AirType::Restricted => 72,
        AirType::Gliding => 36,
        AirType::Cta => 72,
        AirType::Ctr => 72,
        AirType::Matz => 36,
        AirType::Other => 36,
        AirType::Rmz => 72,
        AirType::Tmz => 72,
    }
}

// Openair lat/lon format
fn format_latlon(latlon: &str) -> String {
    format!(
        "{}:{}:{} {} {}:{}:{} {}",
        &latlon[..2],
        &latlon[2..4],
        &latlon[4..6],
        &latlon[6..7],
        &latlon[8..11],
        &latlon[11..13],
        &latlon[13..15],
        &latlon[15..16]
    )
}

// Openair distance format
fn format_distance(distance: &str) -> String {
    match distance.split_once(' ') {
        Some((dist, unit)) => {
            if unit == "km" {
                format!("{:.3}", dist.parse::<f32>().unwrap() / 1.852)
            } else {
                dist.to_string()
            }
        }
        _ => "".to_string(),
    }
}

// Give each volume a type
fn airtype(feature: &Feature, volume: &Volume, settings: &Settings) -> AirType {
    let rules = feature
        .rules
        .iter()
        .chain(volume.rules.iter())
        .flatten()
        .collect::<HashSet<&Rule>>();

    let comp = settings.format == Format::Competition;

    if rules.contains(&Rule::Notam) {
        // NOTAM activated airspace
        AirType::ClassG
    } else {
        match feature.icao_type {
            IcaoType::Atz => settings.atz,
            IcaoType::D => {
                if comp && rules.contains(&Rule::Si) {
                    // Danger area with SI
                    AirType::Prohibited
                } else {
                    // Danger area without SI
                    AirType::Danger
                }
            }
            IcaoType::DOther => {
                if comp
                    && feature.local_type == Some(LocalType::Dz)
                    && rules.contains(&Rule::Intense)
                {
                    // Intense drop zone - competition
                    AirType::Prohibited
                } else {
                    match feature.local_type {
                        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
                            settings.hirta_gvs.unwrap_or(AirType::Other)
                        }
                        Some(LocalType::Glider) => AirType::Gliding,
                        Some(LocalType::Obstacle) => settings.obstacle.unwrap_or(AirType::Other),
                        _ => AirType::Danger,
                    }
                }
            }
            IcaoType::Other => match feature.local_type {
                Some(LocalType::Glider) => {
                    if rules.contains(&Rule::Loa) {
                        AirType::Gliding
                    } else {
                        settings.gliding.unwrap_or(AirType::Other)
                    }
                }
                Some(LocalType::Ils) => settings.ils.unwrap_or(settings.atz),
                Some(LocalType::Matz) => AirType::Matz,
                Some(LocalType::NoAtz) => settings.unlicensed.unwrap_or(AirType::Other),
                Some(LocalType::Rat) => AirType::Prohibited,
                Some(LocalType::Tmz) => AirType::Tmz,
                Some(LocalType::Ul) => settings.microlight.unwrap_or(AirType::Other),
                Some(LocalType::Rmz) => AirType::Rmz,
                _ => AirType::Other,
            },
            IcaoType::P => AirType::Prohibited,
            IcaoType::R => AirType::Restricted,
            _ => {
                if rules.contains(&Rule::Tmz) {
                    AirType::Tmz
                } else if rules.contains(&Rule::Rmz) {
                    AirType::Rmz
                } else {
                    match volume
                        .icao_class
                        .or(feature.icao_class)
                        .unwrap_or(IcaoClass::G)
                    {
                        IcaoClass::A => AirType::ClassA,
                        IcaoClass::B => AirType::ClassB,
                        IcaoClass::C => AirType::ClassC,
                        IcaoClass::D => AirType::ClassD,
                        IcaoClass::E => AirType::ClassE,
                        IcaoClass::F => AirType::ClassF,
                        IcaoClass::G => AirType::ClassG,
                    }
                }
            }
        }
    }
}

// Remove unwanted feature/volume
fn airfilter(feature: &Feature, vol: &Volume, settings: &Settings) -> bool {
    let exclude = match feature.local_type {
        // No-ATZ
        Some(LocalType::NoAtz) => settings.unlicensed.is_none(),
        // Microlight
        Some(LocalType::Ul) => settings.microlight.is_none(),
        // Gliding airspace
        Some(LocalType::Glider) => {
            if feature.icao_type == IcaoType::DOther {
                // Wave box or LOA area
                let rules = feature
                    .rules
                    .iter()
                    .chain(vol.rules.iter())
                    .flatten()
                    .collect::<HashSet<&Rule>>();

                !settings.wave.contains(&feature.name) && !rules.contains(&Rule::Loa)
            } else {
                // Gliding Site
                settings.gliding.is_none() || settings.home.as_ref() == Some(&feature.name)
            }
        }
        // HIRTA/GVS/Laser
        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
            settings.hirta_gvs.is_none()
        }
        _ => false,
    };

    !(exclude || (norm_level(&vol.lower) >= settings.max_level))
}

// Give each volume a name
fn do_name(feature: &Feature, vol: &Volume, n: usize, settings: &Settings) -> String {
    let name = if let Some(name) = &vol.name {
        name.clone()
    } else {
        let mut name = feature.name.clone();

        let rules = feature
            .rules
            .iter()
            .chain(vol.rules.iter())
            .flatten()
            .collect::<HashSet<&Rule>>();

        // Base type name
        if let Some(LocalType::NoAtz) | Some(LocalType::Ul) = feature.local_type {
            name += " A/F"
        } else if let Some(LocalType::Matz)
        | Some(LocalType::Dz)
        | Some(LocalType::Gvs)
        | Some(LocalType::Hirta)
        | Some(LocalType::Ils)
        | Some(LocalType::Laser) = feature.local_type
        {
            name.push(' ');
            name += feature.local_type.unwrap().as_str();
        } else if feature.icao_type == IcaoType::Atz {
            name += " ATZ";
        } else if rules.contains(&Rule::Raz) {
            name += " RAZ";
        }

        // Optional sequence number
        if settings.format == Format::Competition && feature.geometry.len() > 1 {
            name.push('-');
            if let Some(seq) = &vol.seq {
                name += seq;
            } else {
                let x = (b'A'..=b'Z').map(|c| c as char).nth(n);
                name.push(x.unwrap());
            }
        }

        // SI & NOTAM qualifiers
        let mut qualifiers = rules
            .into_iter()
            .filter(|&x| *x == Rule::Si || *x == Rule::Notam)
            .map(|x| x.as_str().to_string())
            .collect::<Vec<String>>();

        if !qualifiers.is_empty() {
            qualifiers.sort();
            qualifiers.reverse();
            name.push(' ');
            name += format!("({})", qualifiers.join("/")).as_ref();
        }

        // Optionally append frequency
        if settings.radio {
            if let Some(freq) = vol.frequency {
                name += format!(" {:.3}", freq).as_ref();
            }
        };

        name
    };

    format!("AN {}\n", name)
}

fn do_type(airtype: AirType) -> String {
    format!("AC {}\n", airtype.as_str())
}

fn do_levels(volume: &Volume) -> String {
    format!(
        "AL {}\nAH {}\n",
        format_level(&volume.lower),
        format_level(&volume.upper)
    )
}

fn do_freq(freq: f64) -> String {
    format!("AF {:.3}\n", freq)
}

fn do_point(point: &str) -> String {
    format!("DP {}\n", format_latlon(point))
}

fn do_line(line: &[String]) -> String {
    line.iter()
        .map(|x| do_point(x))
        .collect::<Vec<String>>()
        .join("")
}

fn do_circle(circle: &Circle, resolution: Option<u32>) -> String {
    match resolution {
        None => format!(
            "V X={}\nDC {}\n",
            format_latlon(&circle.centre),
            format_distance(&circle.radius),
        ),
        Some(res) => poly_circle(circle, res),
    }
}

fn do_arc(arc: &Arc, from: &str, resolution: Option<u32>) -> String {
    match resolution {
        None => {
            let dir = if arc.dir == "cw" { "+" } else { "-" };
            format!(
                "V D={}\nV X={}\nDB {}, {}\n",
                dir,
                format_latlon(&arc.centre),
                format_latlon(from),
                format_latlon(&arc.to)
            )
        }
        Some(res) => poly_arc(arc, from, res),
    }
}

fn do_boundary(boundary: &[Boundary], resolution: Option<u32>) -> String {
    let mut out = String::new();
    let mut prev = "";

    for segment in boundary {
        match segment {
            Boundary::Line(line) => {
                out.push_str(&do_line(line));
                prev = line.last().unwrap();
            }
            Boundary::Arc(arc) => {
                out.push_str(&do_arc(arc, prev, resolution));
                prev = &arc.to;
            }
            Boundary::Circle(circle) => out.push_str(&do_circle(circle, resolution)),
        }
    }

    // Close the polygon
    if let Boundary::Line(line) = &boundary[0] {
        if line[0] != prev {
            out.push_str(&do_point(&line[0]));
        }
    }

    out
}

fn poly_circle(circle: &Circle, resolution: u32) -> String {
    let (clat, clon) = latlon_to_degrees(&circle.centre);
    let centre = Point::new(clon, clat);

    let radius = radius_to_metres(&circle.radius);

    let out = (0..(resolution + 1))
        .into_iter()
        .map(|a| {
            let ang = f64::from(a * 360) / f64::from(resolution);
            let dest = Geodesic::destination(centre, ang, radius);
            degrees_to_point(dest.y(), dest.x())
        })
        .collect();
    out
}

fn poly_arc(arc: &Arc, from: &str, resolution: u32) -> String {
    let (clat, clon) = latlon_to_degrees(&arc.centre);
    let centre = Point::new(clon, clat);

    let (from_lat, from_lon) = latlon_to_degrees(from);
    let from = Point::new(from_lon, from_lat);

    let (to_lat, to_lon) = latlon_to_degrees(&arc.to);
    let to = Point::new(to_lon, to_lat);

    let mut from_ang = Geodesic::bearing(centre, from);
    let mut to_ang = Geodesic::bearing(centre, to);

    let radius = radius_to_metres(&arc.radius);

    if arc.dir == "cw" {
        if from_ang > to_ang {
            to_ang += 360.;
        }
    } else {
        if from_ang < to_ang {
            from_ang += 360.;
        }
    }

    let mut ang_array = (0..(resolution * 2) + 1)
        .into_iter()
        .map(|a| f64::from(a * 360) / f64::from(resolution))
        .filter(|a| {
            f64::from(*a) > (from_ang.min(to_ang) + 0.5)
                && f64::from(*a) < (from_ang.max(to_ang) - 0.5)
        })
        .collect::<Vec<f64>>();

    if arc.dir == "ccw" {
        ang_array.reverse();
    }

    let mut out = ang_array
        .into_iter()
        .map(|a| {
            let dest = Geodesic::destination(centre, a, radius);
            degrees_to_point(dest.y(), dest.x())
        })
        .collect::<String>();

    out.push_str(&degrees_to_point(to_lat, to_lon));

    out
}

fn degrees_to_dms(degrees: f64) -> (u32, u32, u32) {
    let mut sec = (degrees * 3600.0).round() as u32;
    let mut min = sec / 60;
    sec = sec % 60;
    let deg = min / 60;
    min = min % 60;

    (deg, min, sec)
}

fn degrees_to_point(lat: f64, lon: f64) -> String {
    let lat_ns = if lat >= 0.0 { "N" } else { "S" };
    let lon_ew = if lon >= 0.0 { "E" } else { "W" };

    let (lat_deg, lat_min, lat_sec) = degrees_to_dms(lat.abs());
    let (lon_deg, lon_min, lon_sec) = degrees_to_dms(lon.abs());

    format!(
        "DP {:02}:{:02}:{:02} {} {:03}:{:02}:{:02} {}\n",
        lat_deg, lat_min, lat_sec, lat_ns, lon_deg, lon_min, lon_sec, lon_ew
    )
}

// Merge radio frequency data
fn merge_services(airspace: &mut Vec<Feature>, services: &Vec<Service>) {
    // Create frequency map
    let mut frequencies = HashMap::new();
    for service in services {
        for id in &service.controls {
            frequencies.insert(id, service.frequency);
        }
    }

    // Add frequency properties
    for feature in airspace {
        for volume in &mut feature.geometry {
            let volume_freq = if let Some(id) = &volume.id {
                frequencies.get(&id)
            } else {
                None
            };

            let feature_freq = if let Some(id) = &feature.id {
                frequencies.get(&id)
            } else {
                None
            };

            volume.frequency = volume_freq.or(feature_freq).cloned();
        }
    }
}

// Search for volume id and return indices of feature/volume
fn find_volume(airspace: &[Feature], volume_id: &str) -> Option<(usize, usize)> {
    for (f, feature) in airspace.iter().enumerate() {
        for (v, volume) in feature.geometry.iter().enumerate() {
            if volume.id.as_deref() == Some(volume_id) {
                return Some((f, v));
            }
        }
    }
    None
}

// Merge LOAs into main airspace data
fn merge_loa(airspace: &mut Vec<Feature>, loas: &Vec<&Loa>) {
    // Add new features
    for loa in loas {
        for area in &loa.areas {
            for feature in &area.add {
                let mut feature = feature.clone();
                let mut rules = feature.rules.unwrap_or_default();

                // Add LOA rule
                rules.push(Rule::Loa);
                feature.rules = Some(rules);

                airspace.push(feature);
            }
        }
    }

    // Replace volumes
    for loa in loas {
        for area in &loa.areas {
            if let Some(replacements) = &area.replace {
                for replace in replacements {
                    // Find replacement volume
                    if let Some((f, v)) = find_volume(airspace, &replace.id) {
                        let r = (*replace).clone();

                        for vol in r.geometry {
                            airspace[f].geometry.push(vol);
                        }

                        // Delete the exiting volume
                        airspace[f].geometry.remove(v);

                        // Remove feature if no remaining geometry
                        if airspace[f].geometry.is_empty() {
                            airspace.remove(f);
                        }
                    }
                }
            }
        }
    }
}

fn add_obstacles(airspace: &mut Vec<Feature>, obstacles: &Vec<Obstacle>) {
    for obstacle in obstacles {
        let feature = Feature {
            name: obstacle.name.clone(),
            icao_type: IcaoType::DOther,
            icao_class: None,
            id: None,
            local_type: Some(LocalType::Obstacle),
            rules: None,
            geometry: vec![Volume {
                upper: obstacle.elevation.clone(),
                lower: "SFC".to_string(),
                boundary: vec![Boundary::Circle(Circle {
                    centre: obstacle.position.clone(),
                    radius: "0.5 nm".to_string(),
                })],
                icao_class: None,
                frequency: None,
                id: None,
                name: None,
                rules: None,
                seq: None,
            }],
        };
        airspace.push(feature);
    }
}

// File header
fn header(note: &str, airac: &str, commit: &str, user_agent: &str, settings: &Settings) -> String {
    let mut hdr = "UK Airspace\n\
        Alan Sparrow (airspace@asselect.uk)\n\
        \n\
        I have tried to make this data as accurate as possible but\n\
        there will still be errors. Don't blame me if you go somewhere you\n\
        should not have gone while using this data.\n\
        \n\
        To the extent possible under law, Alan Sparrow has waived all\n\
        copyright and related or neighbouring rights to this file. The data\n\
        in this file is based on the work of others including: George Knight,\n\
        Geoff Brown, Peter Desmond and Rory O'Connor.  The data is originally\n\
        sourced from the UK Aeronautical Information Package (AIP).\n\
        \n"
    .to_string();

    hdr.push_str(note);
    hdr.push_str(&format!("\nAIRAC: {}\n", &airac[..10]));
    hdr.push_str(&format!("Commit: {}\n", commit));
    hdr.push_str(&format!("Produced: {}\n", Utc::now().to_rfc3339()));
    hdr.push_str(&format!("User agent: {}\n", user_agent));
    hdr.push_str(&textwrap::fill(format!("{:?}", settings).as_str(), 72));

    // Prepend "*" to lines
    hdr.split('\n')
        .map(|x| {
            if x.is_empty() {
                "*".to_string()
            } else {
                "* ".to_owned() + x
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

// Generate OpenAir data
pub fn openair(yaixm: &Yaixm, settings: &Settings, user_agent: &str) -> String {
    let mut airspace = yaixm.airspace.clone();

    if settings.format == Format::RatOnly {
        airspace = yaixm
            .rat
            .clone()
            .into_iter()
            .filter(|rat| settings.rat.contains(&rat.name))
            .collect();
    } else {
        // Merge LOAs
        let loas = yaixm
            .loa
            .iter()
            .filter(|&x| (x.default == Some(true)) | settings.loa.contains(&x.name))
            .collect::<Vec<&Loa>>();
        merge_loa(&mut airspace, &loas);

        // Add obstacles
        if settings.obstacle.is_some() {
            add_obstacles(&mut airspace, &yaixm.obstacle);
        }

        // Append RA(T)s
        airspace.append(
            &mut yaixm
                .rat
                .iter()
                .filter(|rat| settings.rat.contains(&rat.name))
                .cloned()
                .collect::<Vec<Feature>>(),
        );

        // Merge radio frequencies
        merge_services(&mut airspace, &yaixm.service);
    }

    // Build OpenAir data
    let rel = &yaixm.release;
    let mut output = header(
        &rel.note,
        &rel.airac_date,
        &rel.commit,
        user_agent,
        settings,
    );
    for feature in airspace {
        for (n, volume) in feature.geometry.iter().enumerate() {
            let atype = airtype(&feature, &volume, &settings);
            let res = if settings.format == Format::Competition {
                Some(resolution(atype))
            } else {
                None
            };
            if airfilter(&feature, volume, settings) {
                output.push_str("*\n");
                output.push_str(&do_type(atype));
                output.push_str(&do_name(&feature, volume, n, settings));
                if let Some(freq) = volume.frequency {
                    output.push_str(&do_freq(freq));
                }
                output.push_str(&do_levels(volume));
                output.push_str(&do_boundary(&volume.boundary, res));
            }
        }
    }
    output
}
