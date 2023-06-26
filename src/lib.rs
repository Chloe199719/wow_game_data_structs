#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ collections::HashMap};

pub mod races;
pub mod specs;

  

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

#[derive(Debug)]
pub struct Spell {
  pub  id : u32,
 pub name: String,
  pub  description: String,
  pub  level: u8,
  pub  cooldown: u8,
    duration: u8,
    cost: u8,
    range: u8,
    damage: u32,
    healing: u32,
    resource: Resource,
    resource_cost: u32,
    resource_gain: u32,
  

}
impl Spell {
  pub fn create_spell() -> Spell{
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
        
        }
    }
    
}
#[derive(Debug)]
pub enum Value {
    U32(u32),
    String(String),
    Resources(Resource),
}
impl Spell {
    /// Requires a hashmap with the following keys: <&str, Value> Valuu can be either a u32 or a String or Resource the key must be one of the following:
    /// id, name, description, level, cooldown, duration, cost, range, damage, healing, resource, resource_cost, resource_gain
    pub fn update_spell(&mut self, update: HashMap<&str, Value>) {
        for (key, value) in update.iter()    {
            match value {
                Value::U32(value) => {
                    match *key {
                        "id" => self.id = *value,
                        "level" => self.level = *value as u8,
                        "cooldown" => self.cooldown = *value as u8,
                        "duration" => self.duration = *value as u8,
                        "cost" => self.cost = *value as u8,
                        "range" => self.range = *value as u8,
                        "damage" => self.damage = *value,
                        "healing" => self.healing = *value,
                 
                        "resource_cost" => self.resource_cost = *value,
                        "resource_gain" => self.resource_gain = *value,
                        _ => continue
                }}

                Value::String(value) => {
                    match *key {
           
                        "name" => self.name = value.clone(),
                        "description" => self.description = value.clone(),

           
                        _ => continue
                    
                }
                
                }
                Value::Resources(value) =>{
                    match *key {
                        "resource" => self.resource = value.clone(),
                        _ => continue
                    }
                    
                }
 
            }
           


                // _ => continue
    }
}
}
