use std::char;

use crate:: key::{ Key, Note};

#[derive(Debug)]
enum Sign{
    Note(Note),
    Other(char),
}

#[derive(Debug)]
pub struct Sheet{
    key:Key,
    signs:Vec<Sign>,
}

impl Sheet{
    pub fn new(key:Key)->Self{
        let signs = vec![];
        Self{key,signs}

    }

    pub fn add_note(& mut self,note:Note){
        self.signs.push(Sign::Note(note));
    }

    pub fn add_other(& mut self,other_sign:char){
        self.signs.push(Sign::Other(other_sign));
    }

}