use crate::engraver::types::accidental::Accidental;

pub enum Key {
    C{major: bool},
    G{major: bool},
    D{major: bool},
    A{major: bool},
    E{major: bool},
    B{major: bool},
    FSharp{major: bool},
    CSharp{major: bool},
    GSharp{major: bool},
    DSharp{major: bool},
    ASharp{major: bool},
    F{major: bool},
    BFlat{major: bool},
    EFlat{major: bool},
    AFlat{major: bool},
    DFlat{major: bool},
    GFlat{major: bool},
    CFlat{major: bool},
}

impl Key {
    pub const fn signature(&self) -> (u32, Accidental) {
        match self {
            Key::C{major: true} => (0, Accidental::Natural),
            Key::G{major: true} => (1, Accidental::Sharp),
            Key::D{major: true} => (2, Accidental::Sharp),
            Key::A{major: true} => (3, Accidental::Sharp),
            Key::E{major: true} => (4, Accidental::Sharp),
            Key::B{major: true} => (5, Accidental::Sharp),
            Key::FSharp{major: true} => (6, Accidental::Sharp),
            Key::CSharp{major: true} => (7, Accidental::Sharp),
            Key::F{major: true} => (1, Accidental::Flat),
            Key::BFlat{major: true} => (2, Accidental::Flat),
            Key::EFlat{major: true} => (3, Accidental::Flat),
            Key::AFlat{major: true} => (4, Accidental::Flat),
            Key::DFlat{major: true} => (5, Accidental::Flat),
            Key::GFlat{major: true} => (6, Accidental::Flat),
            Key::CFlat{major: true} => (7, Accidental::Flat),
            Key::A{major: false} => (0, Accidental::Natural),
            Key::E{major: false} => (1, Accidental::Sharp),
            Key::B{major: false} => (2, Accidental::Sharp),
            Key::FSharp{major: false} => (3, Accidental::Sharp),
            Key::CSharp{major: false} => (4, Accidental::Sharp),
            Key::GSharp{major: false} => (5, Accidental::Sharp),
            Key::DSharp{major: false} => (6, Accidental::Sharp),
            Key::ASharp{major: false} => (7, Accidental::Sharp),
            Key::D{major: false} => (1, Accidental::Flat),
            Key::G{major: false} => (2, Accidental::Flat),
            Key::C{major: false} => (3, Accidental::Flat),
            Key::F{major: false} => (4, Accidental::Flat),
            Key::BFlat{major: false} => (5, Accidental::Flat),
            Key::EFlat{major: false} => (6, Accidental::Flat),
            Key::AFlat{major: false} => (7, Accidental::Flat),
            _ => {
                panic!("Unsupported key");
            }
        }
    }
}
