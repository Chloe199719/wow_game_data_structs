#![allow(dead_code)]
#![allow(unused_variables)]

// Need to Add Resources to Specs

pub mod races;
pub mod specs;
pub mod spells;
pub mod classes;
pub mod talents;
use classes::Class;

use races::RaceTraits;
use spells::Spell;

#[derive(Debug)]
 pub struct Character {
    name: String,
    level: u8,
    experience: u32,
    spells: Vec<Spell>,
    class: Class,
    health: u32,
    race: RaceTraits,
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
impl Character {
    pub fn new(class:Class, race:RaceTraits, name:String) -> Character{
        Character {
            race: race,
            name: name,
            level: 0,
            experience: 0,
            spells: Vec::new(),
            class: class,
            health: 0,
            haste : 0,
            mastery : 0,
            armor : 0,
            crit : 0,
            versatility : 0,
            leech : 0,
            avoidance : 0,
            stamina : 0,        
            inventory: Vec::new(),
            equipped: Vec::new(),
            inventory_size: 100,
        }
    }
}



#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    level: u8,
    item_type: ItemType,
    item_slot: ItemSlot,
    primary_stat: PrimaryStat,
    stamina: u32,
    secondary_stats: Vec<Stats>,
    treciary_stats: Vec<Stats>,
    armor: u32,
    durability: u32,
    durability_max: u32,
    item_level: u32,
    item_quality: ItemQuality,
} 
impl Item {
    pub fn new_placeholder() -> Item {
        Item {
            name: String::new(),
            description: String::new(),
            level: 0,
            item_type: ItemType::None,
            item_slot: ItemSlot::None,
            primary_stat: PrimaryStat::None,
            stamina: 0,
            secondary_stats: Vec::new(),
            treciary_stats: Vec::new(),
            armor: 0,
            durability: 0,
            durability_max: 0,
            item_level: 0,
            item_quality: ItemQuality::None,
        }
    }
    pub fn new(name:String, description:String, level:u8, item_type:ItemType, item_slot:ItemSlot, primary_stat:PrimaryStat, stamina:u32, secondary_stats:Vec<Stats>, treciary_stats:Vec<Stats>, armor:u32, durability:u32, durability_max:u32, item_level:u32, item_quality:ItemQuality) -> Item {
        Item {
            name: name,
            description: description,
            level: level,
            item_type: item_type,
            item_slot: item_slot,
            primary_stat: primary_stat,
            stamina: stamina,
            secondary_stats: secondary_stats,
            treciary_stats: treciary_stats,
            armor: armor,
            durability: durability,
            durability_max: durability_max,
            item_level: item_level,
            item_quality: item_quality,
        }
    }
        
    }

impl Item {
    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn update_level(&mut self, level: u8) {
        self.level = level;
    }
    pub fn update_item_type(&mut self, item_type: ItemType) {
        self.item_type = item_type;
    }
    pub fn update_item_slot(&mut self, item_slot: ItemSlot) {
        self.item_slot = item_slot;
    }
    pub fn update_primary_stat(&mut self, primary_stat: PrimaryStat) {
        self.primary_stat = primary_stat;
    }
    pub fn update_stamina(&mut self, stamina: u32) {
        self.stamina = stamina;
    }

    /// Updates the secondary stats of the item.
    /// Takes a vector of Stats as an argument.\n
    /// The vector should contain all the secondary stats of the item.
    pub fn update_secondary_stats(&mut self, secondary_stats: Vec<Stats>) {
        self.secondary_stats = secondary_stats;
    }

    /// Updates the treciary stats of the item.
    /// Takes a vector of Stats as an argument.
    /// The vector should contain all the treciary stats of the item.
    pub fn update_treciary_stats(&mut self, treciary_stats: Vec<Stats>) {
        self.treciary_stats = treciary_stats;
    }
    pub fn update_armor(&mut self, armor: u32) {
        self.armor = armor;
    }
    pub fn update_durability(&mut self, durability: u32) {
        self.durability = durability;
    }
    pub fn update_durability_max(&mut self, durability_max: u32) {
        self.durability_max = durability_max;
    }
    pub fn update_item_level(&mut self, item_level: u32) {
        self.item_level = item_level;
    }
    pub fn update_item_quality(&mut self, item_quality: ItemQuality) {
        self.item_quality = item_quality;
    }

}
    



#[derive(Debug)]
pub enum ItemQuality {
    Poor,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
    Heirloom,
    None,
}
#[derive(Debug)]
pub enum ItemType {
    Cloth,
    Leather,
    Mail,
    Plate,
    None,
}
#[derive(Debug)]
pub enum ItemSlot {
    Head,
    Neck,
    Shoulder,
    Back,
    Chest,
    Wrist,
    Hands,
    Waist,
    Legs,
    Feet,
    Finger,
    Trinket,
    MainHand,
    OffHand,
    None,
}
#[derive(Debug)]
pub enum Stats {
    Haste(u32),
    Mastery(u32),
    Crit(u32),
    Versatility(u32),
    Leech(u32),
    Avoidance(u32),
    Indestructible(u32),
    None,
}
    




#[derive(Debug)]
pub enum PrimaryStat {
    Strength(u32),
    Agility(u32),
    Intellect(u32),
    None,
}


#[derive(Clone)]
#[derive(Debug)]
pub enum Resource {
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


