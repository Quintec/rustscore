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