use midly::TrackEventKind;

use super::midi_event::MidiEvent;
use super::meta_event::MetaEvent;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Event {
	Midi(MidiEvent),
	SysEx(Vec<u8>),
	Escape(Vec<u8>),
	Meta(MetaEvent)
}

impl TryFrom<TrackEventKind<'_>> for Event {
	type Error = &'static str;

	fn try_from(event:TrackEventKind<'_>) -> Result<Event, Self::Error> {
		Ok(
			match event {
				TrackEventKind::Midi { channel, message } => Event::Midi(MidiEvent { channel, message }),
				TrackEventKind::SysEx(data) => Event::SysEx(data.to_vec()),
				TrackEventKind::Escape(data) => Event::Escape(data.to_vec()),
				TrackEventKind::Meta(meta_message) => Event::Meta(meta_message.into()),
			}
		)
	}
}