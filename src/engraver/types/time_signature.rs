pub struct TimeSignature {
    pub beats: u32,
    pub beat_div: BeatDivision,
}

impl Default for TimeSignature {
    fn default() -> TimeSignature {
        TimeSignature::new(4, BeatDivision::Four)
    }
}

impl TimeSignature {
    pub fn new(beats: u32, beat_div: BeatDivision) -> TimeSignature {
        TimeSignature {
            beats,
            beat_div,
        }
    }
}

pub enum BeatDivision {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}