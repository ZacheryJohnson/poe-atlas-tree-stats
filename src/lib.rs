use std::{collections::{VecDeque, HashMap}};

use serde::Deserialize;

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
    group_id: Option<u8>
}

const DELVE_KEYWORDS: &[&'static str] = &["Niko", "Sulphite"];
const DELVE_GROUP_ID: u8 = 1;

const BETRAYAL_KEYWORDS: &[&'static str] = &["Jun", "Syndicate"];
const BETRAYAL_GROUP_ID: u8 = 2;

const BESTIARY_KEYWORDS: &[&'static str] = &["Einhar", "Beast"];
const BESTIARY_GROUP_ID: u8 = 3;

const ESSENCE_KEYWORDS: &[&'static str] = &["Essence", "Imprisoned Monster"];
const ESSENCE_GROUP_ID: u8 = 4;

const STRONGBOX_KEYWORDS: &[&'static str] = &["Strongbox"];
const STRONGBOX_GROUP_ID: u8 = 5;

const INCURSION_KEYWORDS: &[&'static str] = &["Alva", "Incursion", "Architect"];
const INCURSION_GROUP_ID: u8 = 6;

const BLIGHT_KEYWORDS: &[&'static str] = &["Cassia", "Blight", "Oil"];
const BLIGHT_GROUP_ID: u8 = 7;

const METAMORPH_KEYWORDS: &[&'static str] = &["Tane", "Metamorph"];
const METAMORPH_GROUP_ID: u8 = 8;

const MAPS_KEYWORDS: &[&'static str] = &["Maps", "Map Drops", "Map crafting", "Fortune Favours the Brave"];
const MAPS_GROUP_ID: u8 = 9;

const MAP_BOSS_KEYWORDS: &[&'static str] = &["Unique Bosses", "Unique Map Bosses"];
const MAP_BOSS_GROUP_ID: u8 = 10;

const HEIST_KEYWORDS: &[&'static str] = &["Heist", "Blueprint", "Rogue's Markers", "Smuggler's Cache"];
const HEIST_GROUP_ID: u8 = 11;

const SHRINES_KEYWORDS: &[&'static str] = &["Shrine"];
const SHRINES_GROUP_ID: u8 = 12;

const RARE_MONSTERS_KEYWORDS: &[&'static str] = &["Rare Monster"];
const RARE_MONSTERS_GROUP_ID: u8 = 13;

const ABYSS_KEYWORDS: &[&'static str] = &["Abyss"]; //todo
const ABYSS_GROUP_ID: u8 = 14;

const HARBINGER_KEYWORDS: &[&'static str] = &["Harbinger"]; //todo
const HARBINGER_GROUP_ID: u8 = 15;

const LEGION_KEYWORDS: &[&'static str] = &["Legion"]; //todo
const LEGION_GROUP_ID: u8 = 16;

const DELIRIUM_KEYWORDS: &[&'static str] = &["Delirium"]; //todo
const DELIRIUM_GROUP_ID: u8 = 17;

const BEYOND_KEYWORDS: &[&'static str] = &["Beyond"]; //todo
const BEYOND_GROUP_ID: u8 = 18;

const BREACH_KEYWORDS: &[&'static str] = &["Breach"]; //todo
const BREACH_GROUP_ID: u8 = 19;

const RITUAL_KEYWORDS: &[&'static str] = &["Ritual"]; //todo
const RITUAL_GROUP_ID: u8 = 20;

const KIRAC_KEYWORDS: &[&'static str] = &["Kirac"]; //todo
const KIRAC_GROUP_ID: u8 = 21;

const TORMENT_KEYWORDS: &[&'static str] = &["Torment", "Possessed"];
const TORMENT_GROUP_ID: u8 = 22;

const ROGUE_EXILE_KEYWORDS: &[&'static str] = &["Rogue Exile"]; //todo
const ROGUE_EXILE_GROUP_ID: u8 = 23;

const EXPEDITION_KEYWORDS: &[&'static str] = &["Expedition", "Runic"];
const EXPEDITION_GROUP_ID: u8 = 24;

const HARVEST_KEYWORDS: &[&'static str] = &["Harvest", "Sacred Grove"];
const HARVEST_GROUP_ID: u8 = 25;

const UNIQUE_MAPS_KEYWORDS: &[&'static str] = &["Unique Maps"]; //todo
const UNIQUE_MAPS_GROUP_ID: u8 = 26;

const SCARAB_KEYWORDS: &[&'static str] = &["Scarab"]; //todo
const SCARAB_GROUP_ID: u8 = 27;

const SYNTHESIS_KEYWORDS: &[&'static str] = &["Synthesis"]; //todo
const SYNTHESIS_GROUP_ID: u8 = 28;

const SEXTANTS_KEYWORDS: &[&'static str] = &["Sextant"]; //todo
const SEXTANTS_GROUP_ID: u8 = 29;

const ELDERSLAYER_KEYWORDS: &[&'static str] = &["Conqueror", "Sirus"];
const ELDERSLAYER_GROUP_ID: u8 = 30;

const SEARING_EXARCH_KEYWORDS: &[&'static str] = &["Searing Exarch", "Black Star"]; //todo
const SEARING_EXARCH_GROUP_ID: u8 = 31;

const EATER_OF_WORLDS_KEYWORDS: &[&'static str] = &["Eater of Worlds"]; //todo
const EATER_OF_WORLDS_GROUP_ID: u8 = 32;

fn has_keyword(stat_str: &String, keywords: &[&str]) -> bool {
    for keyword in keywords {
        if stat_str.find(keyword).is_some() {
            return true;
        }
    }
    
    false
}

fn get_group_id(node: &Node) -> u8 {
    for stat in &node.stats {
        if has_keyword(stat, DELVE_KEYWORDS) {
            return DELVE_GROUP_ID;
        }

        if has_keyword(stat, BETRAYAL_KEYWORDS) {
            return BETRAYAL_GROUP_ID;
        }

        if has_keyword(stat, BESTIARY_KEYWORDS) {
            return BESTIARY_GROUP_ID;
        }

        if has_keyword(stat, ESSENCE_KEYWORDS) {
            return ESSENCE_GROUP_ID;
        }

        if has_keyword(stat, STRONGBOX_KEYWORDS) {
            return STRONGBOX_GROUP_ID;
        }

        if has_keyword(stat, INCURSION_KEYWORDS) {
            return INCURSION_GROUP_ID;
        }

        if has_keyword(stat, BLIGHT_KEYWORDS) {
            return BLIGHT_GROUP_ID;
        }

        if has_keyword(stat, METAMORPH_KEYWORDS) {
            return METAMORPH_GROUP_ID;
        }

        if has_keyword(stat, MAPS_KEYWORDS) {
            return MAPS_GROUP_ID;
        }

        if has_keyword(stat, MAP_BOSS_KEYWORDS) {
            return MAP_BOSS_GROUP_ID;
        }

        if has_keyword(stat, HEIST_KEYWORDS) {
            return HEIST_GROUP_ID;
        }

        if has_keyword(stat, SHRINES_KEYWORDS) {
            return SHRINES_GROUP_ID;
        }

        if has_keyword(stat, RARE_MONSTERS_KEYWORDS) {
            return RARE_MONSTERS_GROUP_ID;
        }

        if has_keyword(stat, ABYSS_KEYWORDS) {
            return ABYSS_GROUP_ID;
        }

        if has_keyword(stat, HARBINGER_KEYWORDS) {
            return HARBINGER_GROUP_ID;
        }

        if has_keyword(stat, LEGION_KEYWORDS) {
            return LEGION_GROUP_ID;
        }

        if has_keyword(stat, DELIRIUM_KEYWORDS) {
            return DELIRIUM_GROUP_ID;
        }

        if has_keyword(stat, BEYOND_KEYWORDS) {
            return BEYOND_GROUP_ID;
        }

        if has_keyword(stat, BREACH_KEYWORDS) {
            return BREACH_GROUP_ID;
        }

        if has_keyword(stat, RITUAL_KEYWORDS) {
            return RITUAL_GROUP_ID;
        }

        if has_keyword(stat, KIRAC_KEYWORDS) {
            return KIRAC_GROUP_ID;
        }

        if has_keyword(stat, TORMENT_KEYWORDS) {
            return TORMENT_GROUP_ID;
        }

        if has_keyword(stat, ROGUE_EXILE_KEYWORDS) {
            return ROGUE_EXILE_GROUP_ID;
        }

        if has_keyword(stat, EXPEDITION_KEYWORDS) {
            return EXPEDITION_GROUP_ID;
        }

        if has_keyword(stat, HARVEST_KEYWORDS) {
            return HARVEST_GROUP_ID;
        }

        if has_keyword(stat, UNIQUE_MAPS_KEYWORDS) {
            return UNIQUE_MAPS_GROUP_ID;
        }

        if has_keyword(stat, SCARAB_KEYWORDS) {
            return SCARAB_GROUP_ID;
        }

        if has_keyword(stat, SYNTHESIS_KEYWORDS) {
            return SYNTHESIS_GROUP_ID;
        }

        if has_keyword(stat, SEXTANTS_KEYWORDS) {
            return SEXTANTS_GROUP_ID;
        }

        if has_keyword(stat, ELDERSLAYER_KEYWORDS) {
            return ELDERSLAYER_GROUP_ID;
        }

        if has_keyword(stat, SEARING_EXARCH_KEYWORDS) {
            return SEARING_EXARCH_GROUP_ID;
        }

        if has_keyword(stat, EATER_OF_WORLDS_KEYWORDS) {
            return EATER_OF_WORLDS_GROUP_ID;
        }
    }

    0
}

fn load_nodes() -> HashMap<u16, Node> {
    let nodes_file = include_str!("./nodes.json");
    let nodes: Nodes = serde_json::de::from_str(nodes_file).unwrap();

    let mut nodes_map: HashMap<u16, Node> = HashMap::new();    

    for mut node in nodes.nodes {
        let group_id = get_group_id(&node);
        node.group_id = Some(group_id);
        nodes_map.insert(node.skill, node);
    }

    nodes_map
}

fn parse_b64(b64_str: String) -> VecDeque<u8> {
    VecDeque::<u8>::from(base64::decode_config(b64_str, base64::URL_SAFE).unwrap_or_default())
}

fn collapse_stats(stats: Vec<&String>) -> Vec<String> {
    let additive_strings_re = Regex::new(r"^(?P<begin>[^\+\d\.]*)(?P<plus>\+?)(?P<num>[\d\.]+)(?P<percent>%?)(?P<rest>.*)$").unwrap();

    let mut last_str: String = String::new();
    let mut addititive_total: f32 = 0.0;

    let mut collapsed_stats = vec!();

    for stat in stats { 
        // Replace numeric values with a placeholder
        let replacement = additive_strings_re.replace(stat.as_str(), "$begin$plus<X>$percent$rest");

        println!("{}", replacement);
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
            if let Some(num) = captures.get(3) {
                addititive_total += if num.as_str().contains(".") {
                    num.as_str().parse::<f32>().unwrap_or(0.0)
                } else {
                    num.as_str().parse::<u32>().unwrap_or(0) as f32
                };
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

    // try throwing away first byte?
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

    let mut owned_nodes: HashMap<u8, Vec<&String>> = HashMap::new();

    for short in shorts {
        if let Some(node) = nodes_map.get(&short) {
            match owned_nodes.get(&node.group_id.unwrap_or_default()) {
                None => {
                    owned_nodes.insert(node.group_id.unwrap_or_default(), vec!());
                },                
                _ => {}
            }

            let group = owned_nodes.get_mut(&node.group_id.unwrap_or_default()).unwrap();

            for stat in &node.stats {
                group.push(stat);
            }
        }
    }

    // Sort nodes by their group - delve with delve, bestiary with bestiary, etc
    for (_, stats) in &mut owned_nodes {
        stats.sort();
    }

    let mut all_stats: Vec<&String> = vec!();
    for (_, stats) in owned_nodes {
        for stat in stats {
            println!("{}", stat);
            all_stats.push(stat);
        }
    }

    if should_collapse {
        let mut collapsed = collapse_stats(all_stats);
        collapsed.sort();
        collapsed.iter().fold(String::new(), |acc, x| acc + x + "\n")
    } else {
        all_stats.sort();
        all_stats.iter().fold(String::new(), |acc, x| acc + x + "\n")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_bytes() {
        let nodes_map = load_nodes();

        let str = "AAAABgAAfAgbCD8KPQsKC3EM4A8tEOERiRGaFNoWxBcUF8sZghtOHXQeWx9mH_Yj2iTLJf4m5yjQKOkqhCtBK9ouiy-cMAMwNTSIOG8-X0FfQYxCwUOpSl9Nq038UYJWbVjBWn1ayVvZW_Ndt2O1Z-loQGnVbf1xK3bCe6d9E4B4gVqCUYPdiG2M_ZCpla2Xw5lsmZGdXp2HoF2jiaSspSSlrqt8rVyu56_Bsy23SLfLudTF_sYzxpvG0MbZznrOkdBK0KLTONZa2J7Y0NmF2ZDZ59pN3yPf-OGL4evil-Kn4vXlj-co55bq0-zl7pTvrvLo8-P1d_fk-jD6sv3JAAA=";
        let mut parsed = parse_b64(String::from(str));

        let mut shorts: VecDeque<u16> = VecDeque::from([]);

        //println!("{:?}", parsed);
        parsed.pop_front();

        while !parsed.is_empty() {
            let mut short: u16 = 0;
            if let Some(upper) = parsed.pop_front() {
                short = upper as u16;
            }
            if let Some(lower) = parsed.pop_front() {
                short <<= 8;
                short |= lower as u16;
            }
    
            shorts.push_back(short);
        }

        let mut owned_nodes: HashMap<u8, Vec<&String>> = HashMap::new();

        for short in shorts {
            if let Some(node) = nodes_map.get(&short) {
                match owned_nodes.get(&node.group_id.unwrap_or_default()) {
                    None => {
                        owned_nodes.insert(node.group_id.unwrap_or_default(), vec!());
                    },                
                    _ => {}
                }

                let group = owned_nodes.get_mut(&node.group_id.unwrap_or_default()).unwrap();

                for stat in &node.stats {
                    group.push(stat);
                }
            }
        }

        // Sort nodes by their group - delve with delve, bestiary with bestiary, etc
        for (_, stats) in &mut owned_nodes {
            stats.sort();
        }

        let mut all_stats: Vec<&String> = vec!();
        for (group, stats) in owned_nodes {
            for stat in stats {
                //println!("{} ({})", stat, group);
                all_stats.push(stat);
            }
        }
    }

    #[test]
    fn test_collapse() {
        let str = String::from("0.5% chance for Map Drops to be Duplicated");
        let strs = vec![
            &str,
            &str,
            &str,
            &str,
            &str,
            &str,
        ];

        let collapsed = collapse_stats(strs);
        println!("{:?}", collapsed);
    }
}