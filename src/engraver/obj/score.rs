use crate::engraver::obj::part::Part;
use crate::engraver::types::key::Key;
use crate::engraver::types::time_signature::TimeSignature;
struct ScoreMeta {
    title: String,
    composer: String,
    tempo: u32, //bpm
    key: Key,
    time: TimeSignature,
}
struct Score {
    parts: Vec<Part>,
    meta: ScoreMeta,
   // scoreEvents: Vec<ScoreEvent>, later, for playback, somewhere else
   // ex.
   // pub enum ScoreEvent {
   //   Ritardando{ duration: u32, easing: Easing},
   //   Accelerando { duration: u32, easing: Easing},
   //   TempoChange {tempo: u32},
   // } etc.
}