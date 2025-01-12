use super::Event;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SimultaneousEvents {
	pub events: Vec<Event>
}
impl SimultaneousEvents {
	pub fn new(events:Vec<Event>) -> SimultaneousEvents {
		SimultaneousEvents {
			events 
		}
	}
	pub fn push(&mut self, event:Event) {
		self.events.push(event);
	}
}