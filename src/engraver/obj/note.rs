use crate::engraver::types::notehead::Notehead;
use crate::engraver::types::articulation::Articulation;
use crate::engraver::types::stem_direction::StemDirection;

struct NoteMeta {
    color: (u8, u8, u8),
    notehead: Notehead,
    articulation: Articulation,
    stemDirection: StemDirection,
}

pub struct Note {
    pitch: u8,
    duration: u32,
    meta: NoteMeta,
}

pub struct NoteEvent {
    note: Note,
    time: u32,
}