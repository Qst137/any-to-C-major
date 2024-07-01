pub struct Key {
    offset: i8,
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
            "g" => 7,
            "gsharp" | "g#" | "aflat" | "ab" => -4,
            "a" => -3,
            "asharp" | "a#" | "bflat" | "bb" => -2,
            "b" => -1,
            _ => unimplemented!(),
        };
        Self { offset }
    }
}

enum Field {
    Basic,
    High,
    Low,
    DoubleHigh,
    DoubleLow,
    Undefined(i8),
}
pub struct Note {
    offset: i8,
    field: Field,
}

impl Field {
    fn from_number(field_num: i8) -> Self {
        match field_num {
            0 => Field::Basic,
            1 => Field::High,
            2 => Field::DoubleHigh,
            -1 => Field::Low,
            -2 => Field::DoubleLow,
            other_field => Field::Undefined(other_field),
        }
    }
}

impl Note {
    pub fn from_pitch(pitch: i16) -> Self {
        let field = Field::from_number((pitch / 12) as i8);
        let offset = (pitch % 12) as i8;
        Note { offset, field }
    }

    pub fn from_flags(field_num: i8, note_num: i8, sharp_flag: bool, flat_flag: bool) -> Self {
        let mut offset = Note::num_to_offset(note_num);
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

    fn num_to_offset(note_num: i8) -> i8 {
        match note_num {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            _ => unimplemented!(),
        }
    }
}
