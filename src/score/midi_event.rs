use midly::{live::LiveEvent, num::u4, MidiMessage};

/// A struct version of the content of [`midly::live::LiveEvent::Midi`]

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MidiEvent {
	/// The channel this event is to be sent to.
	pub channel: u4,
	/// The message body.
	pub message: MidiMessage,
}

impl MidiEvent {
	pub fn into_live_event(self) -> LiveEvent<'static> {
		LiveEvent::Midi {
			channel: self.channel,
			message: self.message
		}
	}
}

impl MidiEvent {
	/// Encode [`MidiEvent`] as a midi message.
	/// 
	/// # Panics
	/// - Will panic if unable to encode as [`LiveEvent`]
	pub fn encode(&self) -> Vec<u8> {
		let mut buf = Vec::new();
		self.into_live_event().write(&mut buf).unwrap_or_else(|err| panic!("MidiEvent - could not encode self as live event - err: {err}"));
		buf
	}
}