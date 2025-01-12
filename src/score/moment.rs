use super::event::Event;

/// Represents a single moment (tick) in a MIDI track.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Moment {
	/// Events in this moment.
	events: Vec<Event>,
}

impl Moment {
	pub(super) fn destructure(self) -> Vec<Event> {
		self.events
	}
}

impl Moment {
	pub fn is_empty(&self) -> bool {
		self.events.is_empty()
	}
	pub fn get_events(&self) -> &[Event] {
		&self.events
	}
	pub fn push(&mut self, event:Event) {
		self.events.push(event);
	}
}