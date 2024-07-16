#[derive(Debug)]
pub struct Key {
    offset: i32,
}

impl Key {
    pub fn from_letter(input_key: &str) -> Self {
        let input_key = input_key.to_lowercase();
        let offset = match input_key.as_str() {
            "c" => 0,
            "csharp" | "c#" | "dflat" | "db" => 1,
            "d" => 2,
            "dsharp" | "d#" | "eflat" | "eb" => 3,
            "e" => 4,
            "f" => 5,
            "fsharp" | "f#" | "gflat" | "gb" => 6,
            "g" => -5,
            "gsharp" | "g#" | "aflat" | "ab" => -4,
            "a" => -3,
            "asharp" | "a#" | "bflat" | "bb" => -2,
            "b" => -1,
            _ => unimplemented!(),
        };
        Self { offset }
    }

    pub fn to_number(&self) -> i32 {
        self.offset
    }

    pub fn from_number(offset: i32) -> Self {
        Self { offset }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Field {
    Basic,
    High,
    Low,
    DoubleHigh,
    DoubleLow,
    Undefined(i32),
}

impl Field {
    pub fn from_number(field_num: i32) -> Self {
        match field_num {
            0 => Field::Basic,
            1 => Field::High,
            2 => Field::DoubleHigh,
            -1 => Field::Low,
            -2 => Field::DoubleLow,
            other_field => Field::Undefined(other_field),
        }
    }

    fn to_number(&self) -> i32 {
        match self.clone() {
            Field::Basic => 0,
            Field::High => 1,
            Field::DoubleHigh => 2,
            Field::Low => -1,
            Field::DoubleLow => -2,
            Field::Undefined(i) => i,
        }
    }
}

#[derive(Debug)]
pub struct Note {
    offset: i32,
    field: Field,
}

impl Note {
    pub fn from_pitch(pitch: i32) -> Self {
        let (field, offset) = if pitch >= 0 {
            (Field::from_number((pitch / 12) as i32), (pitch % 12) as i32)
        } else {
            (
                Field::from_number((pitch / 12) as i32 - 1),
                (12 + pitch % 12) as i32,
            )
        };
        Note { offset, field }
    }

    pub fn to_pitch(&self) -> i32 {
        self.offset + self.field.to_number() * 12
    }

    pub fn from_flags(field_num: i32, note_char: char, sharp_flag: bool, flat_flag: bool) -> Self {
        let mut offset = Note::char_to_offset(note_char);
        let mut field_num = field_num;
        if sharp_flag {
            offset = offset + 1;
        }
        if flat_flag {
            offset = offset - 1;
        }
        if offset == -1 {
            field_num = field_num - 1;
            offset = 11;
        } else if offset == 12 {
            field_num = field_num + 1;
            offset = 0;
        }
        let field = Field::from_number(field_num);
        Self { offset, field }
    }

    fn char_to_offset(note_char: char) -> i32 {
        match note_char {
            '1' => 0,
            '2' => 2,
            '3' => 4,
            '4' => 5,
            '5' => 7,
            '6' => 9,
            '7' => 11,
            _ => unimplemented!(),
        }
    }

    pub fn get_str(&self) -> &str {
        match self.offset {
            0 => "1",
            1 => "#1",
            2 => "2",
            3 => "#2",
            4 => "3",
            5 => "4",
            6 => "#4",
            7 => "5",
            8 => "#5",
            9 => "6",
            10 => "#6",
            11 => "7",
            _ => unimplemented!(),
        }
    }

    pub fn get_field(&self) -> &Field {
        &self.field
    }
}
