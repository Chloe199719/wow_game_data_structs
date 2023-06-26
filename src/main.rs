#![allow(dead_code)]
#![allow(unused_variables)]
// struct Character {
//     name: String,
//     level: u8,
//     experience: u32,
//     specialization: Specialization,
//     specializations: Vec<Specialization>,
//     spells: Vec<Spell>,
//     class: Class,
//     health: u32,
//     secondary_resource: Resource,
//     secondary_resource_amount: u32,
//     secondary_resource_max: u32,
//     secondary_resource_regen: u32,
//     primary_resource: Resource,
//     primary_resource_amount: u32,
//     primary_resource_max: u32,
//     primary_resource_regen: u32,
//     primary_stat: PrimaryStat,
//     primary_stat_amount: u32,
//     haste : u32,
//     mastery : u32,
//     crit : u32,
//     versatility : u32,
//     leech : u32,
//     avoidance : u32,
//     stamina : u32,
//     armor : u32,
//     inventory: Vec<Item>,
//     equipped: Vec<Item>,
//     inventory_size: u8,
// } 

// enum ClassName {
//     Warrior,
//     Paladin,
//     Hunter,
//     Rogue,
//     Priest,
//     DeathKnight,
//     Shaman,
//     Mage,
//     Warlock,
//     Monk,
//     Druid,
//     DemonHunter,
// } 




use std::collections::HashMap;

use wow_char::Class;
use wow_char::ClassName;
use wow_char::races::Race;
use wow_char::races::RaceTraits;
use wow_char::specs::SpecializationStruct;
use wow_char::specs::Specialization;
use wow_char::spells::Spell;
use wow_char::spells::Value;
use wow_char::Resource;
fn main() {
    let bloodelf  = RaceTraits::new(Race::BloodElf);
    let mut Evoker = Class::new(ClassName::Evoker);
    println!("{:?}", Evoker);
   
    println!("{:?}", bloodelf);
    



//    let mut power_infusion = Spell::create_spell();
//     power_infusion.name = "Power Infusion".to_string();
//     power_infusion.description = "Infuses the target with power, increasing haste by 25% for 20 sec.".to_string();
//     power_infusion.level = 30;
//     power_infusion.cooldown = 120;
//     power_infusion.duration = 20;
//     power_infusion.cost = 0;
//     power_infusion.range = 40;
//     power_infusion.id = 1231


}

enum PrimaryStat {
    Strength,
    Agility,
    Intellect,
}

struct Talent {
    name: String,
    description: String,
    level: u8,
    selected: bool,
}

