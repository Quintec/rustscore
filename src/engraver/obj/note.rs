use crate::engraver::types::duration::Duration;
use crate::engraver::types::notehead::Notehead;
use crate::engraver::types::articulation::Articulation;
use crate::engraver::types::stem_direction::StemDirection;

struct NoteMeta<'a> {
    color: (u8, u8, u8),
    notehead: Notehead,
    articulation: Articulation,
    stemDirection: StemDirection,
    tieTo: Option<&'a NoteEvent<'a>>
}

pub struct Note<'a> {
    pitch: u8,
    duration: Duration,
    meta: NoteMeta<'a>,
}

pub struct NoteEvent<'a> {
    note: Note<'a>,
    time: u32,
}