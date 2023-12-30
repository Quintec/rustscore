use super::measure::Measure;
use crate::engraver::types::clef::Clef;

struct PartMeta {
    name: String,
    instrument: String, // instrument ref?
    clef: Clef,
}
pub struct Part<'a> {
    measures: Vec<Measure<'a>>,
    meta: PartMeta,
}

impl<'a> Default for Part<'a> {
    fn default() -> Part<'a> {
        Part::new(String::from("Piano"), String::from("Piano"), Clef::Treble)
    }
}

impl<'a> Part<'a> {
    pub fn new(name: String, instrument: String, clef: Clef) -> Part<'a> {
        Part {
            measures: Vec::new(),
            meta: PartMeta {
                name,
                instrument,
                clef,
            },
        }
    }
}