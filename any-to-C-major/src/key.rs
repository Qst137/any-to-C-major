use clap::builder::Str;

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

impl Note {
    pub fn from_pitch(pitch: i16) -> Self {
        let field = match (pitch / 12) as i8 {
            0 => Field::Basic,
            1 => Field::High,
            2 => Field::DoubleHigh,
            -1 => Field::Low,
            -2 => Field::DoubleLow,
            other_field => Field::Undefined(other_field),
        };
        let offset = (pitch % 12) as i8;
        Note { offset, field }
    }
}
