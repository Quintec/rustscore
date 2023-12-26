use crate::engraver::obj::measure::Measure;
use crate::engraver::types::clef::Clef;

struct PartMeta {
    name: String,
    instrument: String, // instrument ref?
    clef: Clef,
}
pub struct Part {
    measures: Vec<Measure>,
    meta: PartMeta,
}