use crate::engraver::types::duration::Duration;
use crate::engraver::types::notehead::Notehead;
use crate::engraver::types::articulation::Articulation;
use crate::engraver::types::stem_direction::StemDirection;

struct NoteMeta<'a> {
    color: (u8, u8, u8),
    notehead: Notehead,
    articulations: Vec<Articulation>,
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

impl<'a> Default for NoteMeta<'a> {
    fn default() -> NoteMeta<'a> {
        NoteMeta::new(
            (0, 0, 0),
            Notehead::Circle,
            Vec::new(),
            StemDirection::Auto,
            Option::None,
        )
    }
}

impl<'a> NoteMeta<'a> {
    pub fn new(
        color: (u8, u8, u8),
        notehead: Notehead,
        articulations: Vec<Articulation>,
        stemDirection: StemDirection,
        tieTo: Option<&'a NoteEvent<'a>>,
    ) -> NoteMeta<'a> {
        NoteMeta {
            color,
            notehead,
            articulations,
            stemDirection,
            tieTo,
        }
    }
}

impl<'a> NoteEvent<'a> {
    pub fn new(note: Note<'a>, time: u32) -> NoteEvent<'a> {
        NoteEvent {
            note,
            time,
        }
    }
}

impl<'a> Note<'a> {
    pub fn new(pitch: u8, duration: Duration) -> Note<'a> {
        Note {
            pitch,
            duration,
            meta: Default::default(),
        }
    }
}