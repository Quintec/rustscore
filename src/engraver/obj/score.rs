use super::part::Part;
use crate::engraver::types::key::Key;
use crate::engraver::types::time_signature::TimeSignature;
use crate::engraver::types::time_signature::BeatDivision;
struct ScoreMeta {
    pub title: String,
    pub composer: String,
    pub tempo: u32, //bpm
    pub key: Key,
    pub time: TimeSignature,
}
struct Score<'a> {
    pub parts: Vec<Part<'a>>,
    pub meta: ScoreMeta,
   // scoreEvents: Vec<ScoreEvent>, later, for playback, somewhere else
   // ex.
   // pub enum ScoreEvent {
   //   Ritardando{ duration: u32, easing: Easing},
   //   Accelerando { duration: u32, easing: Easing},
   //   TempoChange {tempo: u32},
   // } etc.
}

impl<'a> Default for Score<'a> {
    fn default() -> Score<'a> {
        Score::new(
            String::from("Untitled"),
            String::from("Unnamed"),
            120,
            Key::C { major: true },
            TimeSignature::new(4, BeatDivision::Four),
        )
    }
}

impl<'a> Score<'a> {
    pub fn new(
        title: String,
        composer: String,
        tempo: u32,
        key: Key,
        time: TimeSignature,
    ) -> Score<'a> {
        Score {
            parts: Vec::new(),
            meta: ScoreMeta {
                title,
                composer,
                tempo,
                key,
                time,
            },
        }
    }
}