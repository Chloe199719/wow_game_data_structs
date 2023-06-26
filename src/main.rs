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
// struct Class {
//     name: ClassName,
//     description: String,
//     specializations: Vec<Specialization>,
//     spells: Vec<Spell>,
//     talents: Vec<Talent>,

// }
enum ClassName {
    Warrior,
    Paladin,
    Hunter,
    Rogue,
    Priest,
    DeathKnight,
    Shaman,
    Mage,
    Warlock,
    Monk,
    Druid,
    DemonHunter,
} 



use wow_char::races::Race;
use wow_char::races::RaceTraits;
use wow_char::SpecializationStruct;
use wow_char::Specialization;
use wow_char::Spell;
fn main() {
    let bloodelf  = RaceTraits::new(Race::BloodElf);
    
    let mut outlaw = SpecializationStruct::new(Specialization::Outlaw);
    let mut  blade_flurry = Spell::create_spell();
    blade_flurry.name = "Blade Flurry".to_string();
    blade_flurry.description = "Increases your attack speed by 20% and your damage by 20% for 12 sec. While Blade Flurry is active, each time you strike with a melee attack, you will also strike with both weapons, dealing 50% Physical damage to the struck enemy.".to_string();
    blade_flurry.level = 0;
    blade_flurry.cooldown = 120;
    

    println!("{:?}", outlaw);
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

