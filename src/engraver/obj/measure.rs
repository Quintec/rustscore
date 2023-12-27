use super::note::NoteEvent;
pub struct Measure<'a> {
    notes: Vec<NoteEvent<'a>>,
}