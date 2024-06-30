use crate:: key::{ Key, Note};

enum Sign{
    Note(Note),
    Other(char),
}
pub struct Sheet{
    key:Key,
    signs:Vec<Sign>,
}

impl Sheet{
    pub fn new(key:Key)->Self{
        let signs = vec![];
        Self{key,signs}

    }

    pub fn add(mut self,sign:Sign){
        self.signs.append(vec![sign].as_mut());
    }
}