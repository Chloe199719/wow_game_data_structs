use crate::specs::SpecializationStruct;
use crate::specs;
use crate::spells::Spell;
use crate::talents::Talent;
#[derive(Debug)]

pub struct Class {
    name: ClassName,
    description: String,
    specializations: Vec<SpecializationStruct>,
    spells: Vec<Spell>,
    talents: Vec<Talent>,
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
                    talents: vec![],
                }
            },
            ClassName::Warrior => {
                Class {
                    name: ClassName::Warrior,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Paladin => {
                Class {
                    name: ClassName::Paladin,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Hunter => {
                Class {
                    name: ClassName::Hunter,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Rogue => {
                Class {
                    name: ClassName::Rogue,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Priest => {
                Class {
                    name: ClassName::Priest,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::DeathKnight => {
                Class {
                    name: ClassName::DeathKnight,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Shaman => {
                Class {
                    name: ClassName::Shaman,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Mage => {
                Class {
                    name: ClassName::Mage,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Warlock => {
                Class {
                    name: ClassName::Warlock,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Monk => {
                Class {
                    name: ClassName::Monk,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::Druid => {
                Class {
                    name: ClassName::Druid,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            },
            ClassName::DemonHunter => {
                Class {
                    name: ClassName::DemonHunter,
                    description: String::from(""),
                    specializations: vec![],
                    spells: vec![],
                    talents: vec![],

                }
            
        }
    }
}
}

impl Class {
    pub fn add_specs(&mut self,specs: SpecializationStruct) {
        self.specializations.push(specs);
    }
    pub fn add_spells(&mut self,spell: Spell) {
        self.spells.push(spell);
    }
    pub  fn get_specs(&self) -> &Vec<SpecializationStruct> {
        &self.specializations
    }
    pub  fn get_spells(&self) -> &Vec<Spell> {
        &self.spells
    }
    pub  fn get_name(&self) -> &ClassName {
        &self.name
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }
    pub fn add_talents(&mut self, talent: Talent) {
        self.talents.push(talent);
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