use crate::engraver::types::time_signature::TimeSignature;
use super::note::NoteEvent;
pub struct Measure<'a> {
    notes: Vec<NoteEvent<'a>>,
    time: TimeSignature
}

impl<'a> Default for Measure<'a> {
    fn default() -> Measure<'a> {
        Measure::new(Default::default())
    }
}

impl<'a> Measure<'a> {
    pub fn new(time: TimeSignature) -> Measure<'a> {
        Measure {
            notes: Vec::new(),
            time
        }
    }
}