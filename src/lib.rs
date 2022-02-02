use std::collections::{VecDeque, HashMap, HashSet};

use serde::Deserialize;
use serde_json::de;

use regex::Regex;

use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;

#[derive(Deserialize, Debug)]
struct Nodes {
    nodes: Vec<Node>
}

#[derive(Deserialize, Debug)]
struct Node {
    skill: u16,
    name: String,
    stats: Vec<String>,
    group: u8,
    isNotable: Option<bool>,
}

fn load_nodes() -> HashMap<u16, Node> {
    let nodes_file = include_str!("./nodes.json");
    let nodes: Nodes = serde_json::de::from_str(nodes_file).unwrap();

    let mut nodes_map: HashMap<u16, Node> = HashMap::new();

    for node in nodes.nodes {
        nodes_map.insert(node.skill, node);
    }

    nodes_map
}

fn parse_b64(b64_str: String) -> VecDeque<u8> {
    VecDeque::<u8>::from(base64::decode_config(b64_str, base64::URL_SAFE).unwrap_or_default())
}

fn collapse_stats(mut stats: Vec<String>) -> Vec<String> {
    let additive_strings_re = Regex::new(r"(?P<plus>\+?)(?P<num>[\d\.]+)(?P<percent>%?)(?P<rest>.*)").unwrap();

    let mut last_str: String = String::new();
    let mut addititive_total: f32 = 0.0;

    let mut collapsed_stats = vec!();

    for stat in stats { 
        // Replace numeric values with a placeholder
        let replacement = additive_strings_re.replace(stat.as_str(), "$plus<X>$percent$rest");

        // Write out the last stat if this isn't the first stat
        if replacement != last_str {
            if !last_str.is_empty() {
                let replaced = last_str.replace("<X>", addititive_total.to_string().as_str());
                collapsed_stats.push(replaced);
            }

            addititive_total = 0.0;
        }

        // Get the current value of the string
        if let Some(captures) = additive_strings_re.captures(stat.as_str()) {
            if let Some(num) = captures.get(2) {
                addititive_total += num.as_str().parse::<f32>().unwrap_or(0.0);
            }
        }

        last_str = replacement.to_string();
    }
    let replaced = last_str.replace("<X>", addititive_total.to_string().as_str());
    collapsed_stats.push(replaced);

    collapsed_stats
} 

#[wasm_bindgen]
pub fn poe_parse(b64_str: String, should_collapse: bool) -> String {
    let nodes_map = load_nodes();
    
    let mut bytes = parse_b64(b64_str);

    //let mut bytes: VecDeque<u8> = VecDeque::from([0x00,0x00,0x00,0x06,0x00,0x00,0x7E,0x00,0x79,0x00,0xCE,0x04,0xDB,0x08,0x1B,0x08,0x3F,0x08,0x6C,0x0A,0x3D,0x0B,0x0A,0x0B,0x71,0x0C,0xE0,0x11,0x9A,0x14,0xDA,0x16,0xC4,0x17,0x14,0x17,0x90,0x17,0xCB,0x19,0x82,0x1E,0x5B,0x1F,0xC4,0x21,0x6B,0x24,0xCB,0x28,0xE9,0x2A,0x84,0x2A,0xD1,0x2B,0xD7,0x2F,0x46,0x2F,0x9C,0x30,0x66,0x34,0x88,0x35,0xCD,0x3B,0x9F,0x3E,0x5F,0x41,0x8C,0x42,0x25,0x43,0xA9,0x47,0xCF,0x48,0x22,0x48,0x2C,0x49,0xBC,0x4A,0x5F,0x4D,0xFC,0x51,0x82,0x5A,0xCE,0x5D,0x4F,0x5D,0xB7,0x5E,0x2A,0x5E,0xF7,0x5F,0x53,0x61,0x48,0x65,0x6B,0x66,0xC6,0x66,0xCA,0x68,0xA1,0x69,0xDC,0x6D,0xFD,0x71,0x2B,0x73,0xA5,0x73,0xE2,0x75,0x43,0x76,0xC2,0x77,0x2C,0x79,0x37,0x7A,0x01,0x7B,0xA7,0x7C,0xCF,0x82,0x8A,0x83,0xC3,0x83,0xDD,0x85,0x27,0x87,0xFB,0x88,0x6D,0x8B,0x2C,0x8D,0x29,0x8F,0x1D,0x8F,0xD4,0x90,0xA9,0x91,0x4D,0x91,0x8C,0x97,0x45,0x99,0x91,0x9D,0x5E,0x9D,0x87,0xA0,0x5D,0xA3,0x85,0xA3,0x89,0xA4,0xAC,0xAE,0x02,0xB4,0x86,0xB5,0xFD,0xB7,0xCB,0xB7,0xCC,0xBA,0x17,0xBA,0xEF,0xBE,0x38,0xC6,0xD0,0xC6,0xD9,0xCE,0x7A,0xD0,0xA9,0xD1,0x4D,0xD1,0x63,0xD1,0xA9,0xD6,0xDB,0xD7,0x4D,0xD8,0xD0,0xD9,0x90,0xD9,0xB8,0xDC,0x4D,0xDD,0x68,0xDF,0x23,0xDF,0xF8,0xE0,0x6B,0xE1,0x8B,0xE2,0xA7,0xE5,0x8F,0xE5,0xB7,0xE7,0x28,0xE9,0x1F,0xEA,0x8D,0xEC,0x49,0xEC,0xE5,0xF2,0xE8,0xF7,0xE4,0xFA,0x30,0xFA,0xB2,0xFC,0x04,0xFC,0xCE,0x00,0x00]);

    // try throwing away first byte?
    bytes.pop_front();

    // First 5 shorts are garbage? I am not smart man
    bytes.pop_front();
    bytes.pop_front();

    bytes.pop_front();
    bytes.pop_front();
    
    bytes.pop_front();
    bytes.pop_front();
    
    bytes.pop_front();
    bytes.pop_front();
    
    bytes.pop_front();
    bytes.pop_front();

    let mut shorts: VecDeque<u16> = VecDeque::from([]);

    while !bytes.is_empty() {
        let mut short: u16 = 0;
        if let Some(upper) = bytes.pop_front() {
            short = upper as u16;
        }
        if let Some(lower) = bytes.pop_front() {
            short <<= 8;
            short |= lower as u16;
        }

        shorts.push_back(short);
    }

    let mut all_stats: Vec<String> = vec!();

    for short in shorts {
        if let Some(node) = nodes_map.get(&short) {
            for stat in &node.stats {
                // TODO: don't clone
                all_stats.push(stat.clone());
            }
        }
    }

    all_stats.sort();
    if should_collapse
    {
        all_stats = collapse_stats(all_stats);
    }

    all_stats.iter().fold(String::new(), |acc, x| acc + x + "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_stats() {
        let stats = vec![
            String::from("+0.5% chance for a Synthesis Map to drop from Unique Bosses (Tier 11+)"),
            String::from("+1% to all maximum Elemental Resistances for each Voltaxic Sulphite Vein or Chest found in Areas"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
            String::from("0.5% chance for Map Drops to be Duplicated"),
        ];

        let collapsed = collapse_stats(stats);
        println!("{:?}", collapsed);
    }
}