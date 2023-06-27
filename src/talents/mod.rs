#[derive(Debug)]
pub struct Talent {
    name: String,
    description: String,
    level: u8,
    selected: bool,
    id: u32,
}
impl Talent {
    pub fn new(name: String, description: String, level: u8 , id:u32) -> Talent {
        Talent {
            id: 0,
            name,
            description,
            level,
            selected: false,
        }
    }
    pub fn new_placeholder() -> Talent {
        Talent {
            id: 0,
            name: String::from(""),
            description: String::from(""),
            level: 0,
            selected: false,
        }
    }
}
impl Talent {
    pub fn toggle_select(&mut self) {
        self.selected = !self.selected;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_description(&self) -> &String {
        &self.description
    }
    pub fn get_level(&self) -> &u8 {
        &self.level
    }
    pub fn get_selected(&self) -> &bool {
        &self.selected
    }
    pub fn get_id(&self) -> &u32 {
        &self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

}
    
