#![allow(dead_code)]
#![allow(unused_variables)]
struct Character {
    name: String,
    level: u8,
    experience: u32,
    specialization: Specialization,
    specializations: Vec<Specialization>,
    spells: Vec<Spell>,
    class: Class,
    health: u32,
    secondary_resource: Resource,
    secondary_resource_amount: u32,
    secondary_resource_max: u32,
    secondary_resource_regen: u32,
    primary_resource: Resource,
    primary_resource_amount: u32,
    primary_resource_max: u32,
    primary_resource_regen: u32,
    primary_stat: PrimaryStat,
    primary_stat_amount: u32,
    haste : u32,
    mastery : u32,
    crit : u32,
    versatility : u32,
    leech : u32,
    avoidance : u32,
    stamina : u32,
    armor : u32,
    inventory: Vec<Item>,
    equipped: Vec<Item>,
    inventory_size: u8,
} 
struct Class {
    name: ClassName,
    description: String,
    specializations: Vec<Specialization>,
    spells: Vec<Spell>,
    talents: Vec<Talent>,

}
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
struct Specialization{
    role: Role,
    name: String,
    description: String,
    talents: Vec<Talent>,
   
    spells: Vec<Spell>,
}


use wow_char::races::Race;
use wow_char::races::RaceTraits;

fn main() {
    let bloodelf  = RaceTraits::new(Race::BloodElf);
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
enum Resource {
    Rage,
    Energy,
    Mana,
    Focus,
    RunicPower,
    Fury,
    Pain,
    Insanity,
    AstralPower,
    Maelstrom,
    Chi,
    HolyPower,
    SoulShards,
    ArcaneCharges,
    ComboPoints,
    Runes,
    None,
}
enum Role {
    Tank,
    Healer,
    Damage
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

struct Item {
    name: String,
    description: String,
    item_level: u8,
    min_level: u8,
}
struct Spell {
    id : u32,
    name: String,
    description: String,
    level: u8,
    cooldown: u8,
    duration: u8,
    cost: u8,
    range: u8,
    damage: u32,
    healing: u32,
    resource: Resource,
    resource_cost: u32,
    resource_gain: u32,
    resource_gain_on_crit: u32,
    resource_gain_on_hit: u32,
    resource_gain_on_cast: u32,
    resource_gain_on_kill: u32,
    resource_gain_on_spell_cast: u32,
    resource_gain_on_spell_crit: u32,
    resource_gain_on_spell_hit: u32,
    resource_gain_on_spell_kill: u32,
    resource_gain_on_spell_tick: u32,
    resource_gain_on_tick: u32,
    resource_gain_on_hit_taken: u32,
    resource_gain_on_crit_taken: u32,
    resource_gain_on_dodge: u32,
    resource_gain_on_parry: u32,
    resource_gain_on_block: u32,
    resource_gain_on_resist: u32,
    resource_gain_on_evade: u32,
    resource_gain_on_immunity: u32,
    resource_gain_on_deflect: u32,
    resource_gain_on_absorb: u32,
    resource_gain_on_interrupt: u32,
    resource_gain_on_dispel: u32,
    resource_gain_on_death: u32,

    resource_gain_on_assist: u32,
    resource_gain_on_spell_interrupt: u32,
    resource_gain_on_spell_dispel: u32,
    resource_gain_on_spell_death: u32,
   
    resource_gain_on_spell_assist: u32,
    resource_gain_on_spell_resist: u32,
    resource_gain_on_spell_evade: u32,
    resource_gain_on_spell_deflect: u32,
    resource_gain_on_spell_absorb: u32,
    resource_gain_on_spell_immunity: u32,
    resource_gain_on_spell_block: u32,
    resource_gain_on_spell_parry: u32,
    resource_gain_on_spell_dodge: u32,
    resource_gain_on_spell_crit_taken: u32,
    resource_gain_on_spell_hit_taken: u32,

    resource_gain_on_tick_taken: u32,
    resource_gain_on_spell_damage_taken: u32,
    resource_gain_on_spell_healing_taken: u32,
    resource_gain_on_damage_taken: u32,
    resource_gain_on_healing_taken: u32,
    resource_gain_on_damage: u32,
    resource_gain_on_healing: u32,
    resource_gain_on_spell_damage: u32,
    resource_gain_on_spell_healing: u32,

}
impl Spell {
    fn create_spell() -> Spell{
    Spell {
            id : 0,
            name: String::from(""),
            description: String::from(""),
            level: 0,
            cooldown: 0,
            duration: 0,
            cost: 0,
            range: 0,
            damage: 0,
            healing: 0,
            resource: Resource::None,
            resource_cost: 0,
            resource_gain: 0,
            resource_gain_on_crit: 0,
            resource_gain_on_hit: 0,
            resource_gain_on_cast: 0,
            resource_gain_on_kill: 0,
            resource_gain_on_spell_cast: 0,
            resource_gain_on_spell_crit: 0,
            resource_gain_on_spell_hit: 0,
            resource_gain_on_spell_kill: 0,
            resource_gain_on_spell_tick: 0,
            resource_gain_on_tick: 0,
            resource_gain_on_hit_taken: 0,
            resource_gain_on_crit_taken: 0,
            resource_gain_on_dodge: 0,
            resource_gain_on_parry: 0,
            resource_gain_on_block: 0,
            resource_gain_on_resist: 0,
            resource_gain_on_evade: 0,
            resource_gain_on_immunity: 0,
            resource_gain_on_deflect: 0,
            resource_gain_on_absorb: 0,
            resource_gain_on_interrupt: 0,
            resource_gain_on_dispel: 0,
            resource_gain_on_death: 0,
        
            resource_gain_on_assist: 0,
            resource_gain_on_spell_interrupt: 0,
            resource_gain_on_spell_dispel: 0,
            resource_gain_on_spell_death: 0,
           
            resource_gain_on_spell_assist: 0,
            resource_gain_on_spell_resist: 0,
            resource_gain_on_spell_evade: 0,
            resource_gain_on_spell_deflect: 0,
            resource_gain_on_spell_absorb: 0,
            resource_gain_on_spell_immunity: 0,
            resource_gain_on_spell_block: 0,
            resource_gain_on_spell_parry: 0,
            resource_gain_on_spell_dodge: 0,
            resource_gain_on_spell_crit_taken: 0,
            resource_gain_on_spell_hit_taken: 0,
        
            resource_gain_on_tick_taken: 0,
            resource_gain_on_spell_damage_taken: 0,
            resource_gain_on_spell_healing_taken: 0,
            resource_gain_on_damage_taken: 0,
            resource_gain_on_healing_taken: 0,
            resource_gain_on_damage: 0,
            resource_gain_on_healing: 0,
            resource_gain_on_spell_damage: 0,
            resource_gain_on_spell_healing: 0,
        
        }
    }
    
}