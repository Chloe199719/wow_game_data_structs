#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]






use std::collections::HashMap;

use wow_char::classes::Class;
use wow_char::classes::ClassName;
use wow_char::races::Race;
use wow_char::races::RaceTraits;
use wow_char::specs::SpecializationStruct;
use wow_char::specs::Specialization;
use wow_char::spells::Spell;
use wow_char::spells::Value;
use wow_char::Resource;
use wow_char::Character;
fn main() {
    let bloodelf  = RaceTraits::new(Race::BloodElf);
    let  Evoker = Class::new(ClassName::Evoker);
    let Chloe = Character::new(Evoker, bloodelf, String::from("Chloe"));
    println!("{:#?}", Chloe);
    // println!("{:?}", Evoker);
     // println!("{:?}", bloodelf);
    



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





