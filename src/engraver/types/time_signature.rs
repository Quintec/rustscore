pub struct TimeSignature {
    pub beats: u32,
    pub beat_div: BeatDivision,
}

pub enum BeatDivision {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}