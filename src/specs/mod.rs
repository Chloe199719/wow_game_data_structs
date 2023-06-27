
use crate::spells::Spell;
use crate::talents::Talent;
#[derive(Debug)]
pub enum Specialization {
    Blood,
    Frost,
    Unholy,
    Havoc,
    Vengeance,
    Balance,
    Feral,
    Guardian,
    RestorationDruid,
    BeastMastery,
    Marksmanship,
    Survival,
    Arcane,
    Fire,
    FrostMage,
    Brewmaster,
    Mistweaver,
    Windwalker,
    HolyPaladin,
    ProtectionPaladin,
    Retribution,
    Discipline,
    HolyPriest,
    Shadow,
    Assassination,
    Outlaw,
    Subtlety,
    Elemental,
    Enhancement,
    RestorationShaman,
    Affliction,
    Demonology,
    Destruction,
    Arms,
    Fury,
    ProtectionWarrior,
    Devastation,
    Perservation,
    Augmentation,
}
#[derive(Debug)]
pub enum Role {
    Tank,
    Healer,
    Melee,
    Ranged,
}
#[derive(Debug)]
pub struct SpecializationStruct{
    role: Role,
   pub name: Specialization,
    description: String,
    spells: Vec<Spell>,
    talents: Vec<Talent>,
    active: bool,

}
impl SpecializationStruct {
  pub fn new(spec:Specialization) -> SpecializationStruct {
        match spec {

            Specialization::Subtlety => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Subtlety,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::RestorationShaman => SpecializationStruct {
                role: Role::Healer,
                name: Specialization::RestorationShaman,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Shadow => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Shadow,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },


            Specialization::ProtectionWarrior => {
                SpecializationStruct {
                    role: Role::Tank,
                    name: Specialization::ProtectionWarrior,
                    description: String::from(""),
                    spells: vec![],
                    talents: vec![],
                    active: false,
                }
            }
            Specialization::Perservation => {
                SpecializationStruct {
                    role: Role::Healer,
                    name: Specialization::Perservation,
                    description: String::from(""),
                    spells: vec![],
                    talents: vec![],
                    active: false,
                }
            }

            Specialization::Retribution =>{
                SpecializationStruct {
                    role: Role::Melee,
                    name: Specialization::Retribution,
                    description: String::from(""),
                    spells: vec![],
                    talents: vec![],
                    active: false,
                }
            }

            Specialization::ProtectionPaladin => SpecializationStruct {
                role: Role::Tank,
                name: Specialization::ProtectionPaladin,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Windwalker => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Windwalker,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Outlaw => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Outlaw,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Mistweaver=> SpecializationStruct {
                role: Role::Healer,
                name: Specialization::Mistweaver,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            
            Specialization::Survival => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Survival,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::RestorationDruid => SpecializationStruct {
                role: Role::Healer,
                name: Specialization::RestorationDruid,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Vengeance => SpecializationStruct {
                role: Role::Tank,
                name: Specialization::Vengeance,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Unholy => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Unholy,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            

            Specialization::Marksmanship => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Marksmanship,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Elemental => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Elemental,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Fire => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Fire,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            Specialization::Destruction => SpecializationStruct{
                role: Role::Ranged,
                name: Specialization::Destruction,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,

            },
        


            Specialization::Augmentation => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Augmentation,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Affliction => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Affliction,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Arms => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Arms,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Assassination => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Assassination,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Balance => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Balance,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::BeastMastery => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::BeastMastery,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Blood => SpecializationStruct {
                role: Role::Tank,
                name: Specialization::Blood,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Brewmaster => SpecializationStruct {
                role: Role::Tank,
                name: Specialization::Brewmaster,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Demonology => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::Demonology,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Devastation => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Devastation,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Discipline => SpecializationStruct {
                role: Role::Healer,
                name: Specialization::Discipline,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Enhancement => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Enhancement,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Feral => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Feral,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
           
            Specialization::Frost => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Frost,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::FrostMage => SpecializationStruct {
                role: Role::Ranged,
                name: Specialization::FrostMage,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Fury => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Fury,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Guardian => SpecializationStruct {
                role: Role::Tank,
                name: Specialization::Guardian,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Havoc => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Havoc,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::HolyPaladin => SpecializationStruct {
                role: Role::Healer,
                name: Specialization::HolyPaladin,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::HolyPriest => SpecializationStruct {
                role: Role::Healer,
                name: Specialization::HolyPriest,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },
            Specialization::Arcane => SpecializationStruct {
                role: Role::Melee,
                name: Specialization::Arcane,
                description: String::from(""),
                spells: vec![],
                talents: vec![],
                active: false,
            },

            
        }
    }
}
impl SpecializationStruct {
    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell);
    }
    pub fn get_spells(&self) -> &Vec<Spell> {
        &self.spells
    }
    pub fn get_name(&self) -> &Specialization {
        &self.name
    }
    pub fn get_role(&self) -> &Role {
        &self.role
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn edit_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }
    pub fn add_talent(&mut self, talent: Talent) {
        self.talents.push(talent);
    }
    pub fn get_active(&self) -> &bool {
        &self.active
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    
}
  