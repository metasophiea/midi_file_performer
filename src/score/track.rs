use midly::num::u24;
use midly::{MetaMessage, TrackEvent, TrackEventKind};

use super::meta_event::MetaEvent;
use super::Event;
use super::simultaneous_events::SimultaneousEvents;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Track {
	potential_simultaneous_events_sequence: Vec<Option<SimultaneousEvents>>,
	all_tempos: Vec<(usize, u24)>
}
// eg. [[Event], none, none, none, [Event, Event], none, none, [Event]]

impl From<&Vec<TrackEvent<'_>>> for Track {
	fn from(track_events:&Vec<TrackEvent<'_>>) -> Track {
		//create vector
			let total_ticks = track_events
				.iter()
				.enumerate()
				.map(|(_index, track_event)| {
					//correct "EndOfTrack being too far out" situation
					if TrackEventKind::Meta(MetaMessage::EndOfTrack) == track_event.kind {
						0
					} else {
						u32::from(track_event.delta) as usize
					}
				})
				.sum::<usize>()
				+ 1;
			let mut result:Vec<Option<SimultaneousEvents>> = vec![None; total_ticks];
			
		//populate vector
			let mut position = 0;
			for track_event in track_events {
				if TrackEventKind::Meta(MetaMessage::EndOfTrack) == track_event.kind {
					continue;
				}

				position += u32::from(track_event.delta) as usize;
				if let Ok(event) = Event::try_from(track_event.kind) {
					if let Some(simultaneous_events) = &mut result[position] {
						simultaneous_events.push(event);
					} else {
						result[position] = Some(SimultaneousEvents::new(vec![event]));
					}
				}
			}
		
		//all_tempos
			let all_tempos = result
				.iter()
				.enumerate()
				.filter_map(|(index, potential_simultaneous_event)| 
					potential_simultaneous_event
						.as_ref()
						.map(|simultaneous_event|
							simultaneous_event.events
								.iter()
								.filter_map(|event|
									if let Event::Meta(MetaEvent::Tempo(microseconds_per_beat)) = event {
										Some((index, *microseconds_per_beat))
									} else {
										None
									}
								)
								.collect::<Vec<(usize, u24)>>()
					)
				)
				.flatten()
				.collect();

		Track {
			potential_simultaneous_events_sequence: result,
			all_tempos
		}
	}
}

impl Track {
	pub fn append(&mut self, other:&mut Track) {
		self.potential_simultaneous_events_sequence.append(&mut other.potential_simultaneous_events_sequence);	
	}
}

impl Track {
	pub fn len(&self) -> usize {
		self.potential_simultaneous_events_sequence.len()
	}
	pub fn get_events(&self, index:usize) -> Option<&Option<SimultaneousEvents>> {
		self.potential_simultaneous_events_sequence.get(index)
	}
}

impl Track {
	pub fn get_all_tempos(&self) -> &[(usize, u24)] {
		&self.all_tempos
	}
	pub fn calculate_ticks_until_next_events_from_index(&self, index:usize) -> Option<usize> {
		if self.potential_simultaneous_events_sequence.len() <= index + 1 {
			return None;
		}

		let mut following_ticks = 1;
		while 
			self.potential_simultaneous_events_sequence.len() > index + following_ticks &&
			self.potential_simultaneous_events_sequence[index + following_ticks].is_none()
		{
			following_ticks += 1;
		}

		Some(following_ticks)
	}
}