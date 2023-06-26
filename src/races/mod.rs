#[derive(Debug)]
 pub struct RaceTraits{
    pub race: Race,
   pub racial:String,
   pub racial_description:String,
   pub racial_ability:String,
   pub racial_ability_description:String,
   pub racial_ability_cooldown:u8,
   pub racial_ability_duration:u8,
   pub faction:Faction,  
}
impl RaceTraits {
   pub fn new(race:Race) -> RaceTraits {
       match race {
            Race::Tauren =>{
                RaceTraits {
                    race: race,
                    racial: String::from("Brawn"),
                    racial_ability: String::from("War Stomp"),
                    racial_ability_cooldown : 150,
                    racial_ability_duration : 1,
                    racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                    racial_description: String::from("Critical strike damage and healing increased by 2%"),
                    faction: Faction::Horde,
                    }
                },
                Race::Dwarf =>{
                    RaceTraits {
                        race:race,
                        racial: String::from("Explorer"),
                        racial_ability: String::from("Stoneform"),
                        racial_ability_cooldown :  120,
                        racial_ability_duration : 8,
                        racial_ability_description: String::from("Removes all harmful Poison, Disease, Curse, Magic, and Bleed effects and reduces all physical damage taken by 10% for 8 sec."),
                        racial_description: String::from("Removes all harmful Poison, Disease, Curse, Magic, and Bleed effects and reduces all physical damage taken by 10% for 8 sec."),
                        faction: Faction::Alliance,
                    }
                }
                Race::Orc =>{
                    RaceTraits {
                        race: race,
                        racial: String::from("Hardiness"),
                        racial_ability: String::from("Blood Fury"),
                        racial_ability_cooldown : 120,
                        racial_ability_duration : 15,
                        racial_ability_description: String::from("Increases your attack power and Intellect by 583 for 15 sec."),
                        racial_description: String::from("Duration of Stun effects reduced by an additional 20%."),
                        faction: Faction::Horde,
                    }
                }
                Race::BloodElf =>{
                    RaceTraits {
                        race :race,
                        racial: String::from("Arcane Acuity"),
                        racial_ability: String::from("Arcane Torrent"),
                        racial_ability_cooldown : 120,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Remove 1 beneficial effect from all enemies within 8 yards and restore 3% of your mana."),
                        racial_description: String::from("Increases critical strike chance by 1%"),
                        faction: Faction::Horde,
                    }
                },
                Race::DarkIronDwarf =>{
                    RaceTraits {
                        race:race,
                        racial: String::from("Forged in Flames"),
                        racial_ability: String::from("Fireblood"),
                        racial_ability_cooldown : 120,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Removes all poison, disease, curse, magic, and bleed effects and increases your primary stat by 876 and an additional 292 for each effect removed. Lasts 8 sec."),
                        racial_description: String::from("Reduces damage taken from Physical attacks by 1%."),
                        faction: Faction::Horde,
                    }
                },
                Race::Draenei =>{
                    RaceTraits {
                        race:race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Reduces damage taken from Physical attacks by 1%."),
                        faction: Faction::Horde,
                    }
                },
                Race::Gnome =>{
                     RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Goblin =>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::HighmountainTauren=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Human=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::KulTiran=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::LightforgedDraenei=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::MagharOrc=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::NightElf=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Nightborne=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Pandaren=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Troll=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::Undead=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::VoidElf =>
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    },
                
                Race::Worgen=>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                },
                Race::ZandalariTroll =>{
                    RaceTraits {
                        race,
                        racial: String::from("Brawn"),
                        racial_ability: String::from("War Stomp"),
                        racial_ability_cooldown : 150,
                        racial_ability_duration : 1,
                        racial_ability_description: String::from("Stuns up to 5 enemies withing 8 yards for 2sec."),
                        racial_description: String::from("Critical strike damage and healing increased by 2%"),
                        faction: Faction::Horde,
                    }
                }
  }
}
}
#[derive(Debug)]
pub enum Race {
    BloodElf,
    Orc,
    Tauren,
    Troll,
    Undead,
    Goblin,
    NightElf,
    Draenei,
    Dwarf,
    DarkIronDwarf,
    Gnome,
    Worgen,
    Human,
    Pandaren,
    Nightborne,
    HighmountainTauren,
    VoidElf,
    LightforgedDraenei,
    MagharOrc,
    ZandalariTroll,
    KulTiran,
}


#[derive(Debug)]
pub enum Faction {
    Alliance,
    Horde,
}