#![allow(dead_code)]
#![allow(unused_variables)]



pub mod races;
pub mod specs;
pub mod spells;

use self::specs::SpecializationStruct;
use self::spells::Spell;
#[derive(Debug)]
pub struct Class {
    name: ClassName,
    description: String,
    specializations: Vec<SpecializationStruct>,
    spells: Vec<Spell>,
}
impl Class {
   pub fn new (class_name:ClassName) -> Class {
        match class_name {
            ClassName::Evoker => {
                Class {
                    name: ClassName::Evoker,
                    description: String::from(""),
                    specializations: vec![SpecializationStruct::new(specs::Specialization::Augmentation), SpecializationStruct::new(specs::Specialization::Devastation), SpecializationStruct::new(specs::Specialization::Perservation)],
                    spells: vec![],
                }
            },
            ClassName::Warrior => {
                Class {
                    name: ClassName::Warrior,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Paladin => {
                Class {
                    name: ClassName::Paladin,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Hunter => {
                Class {
                    name: ClassName::Hunter,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Rogue => {
                Class {
                    name: ClassName::Rogue,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Priest => {
                Class {
                    name: ClassName::Priest,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::DeathKnight => {
                Class {
                    name: ClassName::DeathKnight,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Shaman => {
                Class {
                    name: ClassName::Shaman,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Mage => {
                Class {
                    name: ClassName::Mage,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Warlock => {
                Class {
                    name: ClassName::Warlock,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Monk => {
                Class {
                    name: ClassName::Monk,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::Druid => {
                Class {
                    name: ClassName::Druid,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            },
            ClassName::DemonHunter => {
                Class {
                    name: ClassName::DemonHunter,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                }
            
        }
    }
}
}

impl Class {
    fn add_specs(&mut self,specs: SpecializationStruct) {
        self.specializations.push(specs);
    }
    fn add_spells(&mut self,spell: Spell) {
        self.spells.push(spell);
    }
    fn get_specs(&self) -> &Vec<SpecializationStruct> {
        &self.specializations
    }
    fn get_spells(&self) -> &Vec<Spell> {
        &self.spells
    }
    fn get_name(&self) -> &ClassName {
        &self.name
    }
    fn get_description(&self) -> &String {
        &self.description
    }
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
        
    
}

#[derive(Debug)]
pub enum ClassName {
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
    Evoker,
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


