use std::char;

use crate::key::{Key, Note};

#[derive(Debug)]
enum Sign {
    Note(Note),
    Other(char),
}

impl Sign {
    fn to_sign_inside(self) -> SignInside {
        match self {
            Self::Note(note) => SignInside::Note(note.to_pitch()),
            Self::Other(char) => SignInside::Other(char),
        }
    }
}

#[derive(Debug)]
pub struct Sheet {
    key: Key,
    signs: Vec<Sign>,
}

impl Sheet {
    pub fn new(key: Key) -> Self {
        let signs = vec![];
        Self { key, signs }
    }

    pub fn add_note(&mut self, note: Note) {
        self.signs.push(Sign::Note(note));
    }

    pub fn add_other(&mut self, other_sign: char) {
        self.signs.push(Sign::Other(other_sign));
    }

    pub fn to_sheet_inside(self) -> SheetInside {
        let signs_inside = self
            .signs
            .into_iter()
            .map(|sign| sign.to_sign_inside())
            .collect();
        let key_inside = self.key.to_number();
        SheetInside {
            key_inside,
            signs_inside,
        }
    }
}

#[derive(Debug)]
enum SignInside {
    Note(i32),
    Other(char),
}

impl SignInside {
    pub fn add(&mut self, number: i32) {
        match *self {
            Self::Note(i) => *self = Self::Note(i + number),
            Self::Other(_) => (),
        }
    }

    pub fn to_sign(self) -> Sign {
        match self {
            Self::Note(i) => Sign::Note(Note::from_pitch(i)),
            Self::Other(char) => Sign::Other(char),
        }
    }
}

#[derive(Debug)]
pub struct SheetInside {
    key_inside: i32,
    signs_inside: Vec<SignInside>,
}

impl SheetInside {
    fn all_add(&mut self, number: i32) -> &mut Self {
        self.signs_inside
            .iter_mut()
            .for_each(|one_sign| one_sign.add(number));
        self
    }

    pub fn to_sheet(self) -> Sheet {
        let signs = self
            .signs_inside
            .into_iter()
            .map(|one_sign| one_sign.to_sign())
            .collect();
        let key = Key::from_number(self.key_inside);
        Sheet { signs, key }
    }

    pub fn to_c(&mut self) {
        self.all_add(self.key_inside);
    }
}
