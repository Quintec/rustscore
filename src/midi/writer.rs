use midly::{live::LiveEvent, MidiMessage, MidiTrack, MidiFile, Format, Timing, TimingDivision, Header, write};
use crate::engraver::obj::{score::Score, part::Part, measure::Measure, note::Note};

fn write_score(score: Score, out: &mut W) -> WriteResult<W>
where W: Write {
    // TODO calc actual time
    let header = Header::new(Format::Parallel, Timing::Metrical(score.meta.time.beats));
    let mut tracks = Vec::new();
    for part in score.parts {
        tracks.push(write_part(part));
    }

    write::write(header, tracks, out)
}

fn write_part(part: Part) -> Vec<TrackEvent> {
    let mut events = Vec::new();
    for measure in part.measures {
        events.extend(write_measure(measure));
    }
    events
}

fn write_measure(measure: Measure) -> Vec<TrackEvent> {
    let mut events = Vec::new();
    for evt in measure.notes {
        events.extend(write_note(evt.note));
    }
}

fn write_note(note: Note) -> Vec<LiveEvent> {
    
}

/*
fn note_on(channel: u8, key: u8) {
    let ev = LiveEvent::Midi {
        channel: channel.into(),
        message: MidiMessage::NoteOn {
            key: key.into(),
            vel: 127.into(),
        },
    };
    let mut buf = Vec::new();
    ev.write(&mut buf).unwrap();
    write_midi(&buf[..]);
}
 */