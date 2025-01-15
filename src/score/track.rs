use std::time::Duration;

use midly::num::u24;
use midly::TrackEvent;

use crate::Timer;

use super::meta_event::MetaEvent;
use super::Event;
use super::simultaneous_events::SimultaneousEvents;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Track {
	potential_simultaneous_events_sequence: Vec<Option<SimultaneousEvents>>
}
// eg. [[Event], none, none, none, [Event, Event], none, none, [Event]]

impl From<&Vec<TrackEvent<'_>>> for Track {
	fn from(track_events:&Vec<TrackEvent<'_>>) -> Track {
		//create vector
			let total_ticks = track_events
				.iter()
				.map(|track_event| u32::from(track_event.delta) as usize)
				.sum::<usize>()
				+ 1;
			let mut result:Vec<Option<SimultaneousEvents>> = vec![None; total_ticks];
			
		//populate vector
			let mut position = 0;
			for track_event in track_events {
				position += u32::from(track_event.delta) as usize;
				if let Ok(event) = Event::try_from(track_event.kind) {
					if let Some(simultaneous_events) = &mut result[position] {
						simultaneous_events.push(event);
					} else {
						result[position] = Some(SimultaneousEvents::new(vec![event]));
					}
				}
			}

		Track {
			potential_simultaneous_events_sequence: result,
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
	pub fn calculate_duration(&self, timer:&mut Timer) -> Duration {
		if timer.get_speed() == 0.0 {
			return Duration::MAX
		}
		
		let mut counter = Duration::default();
		for potential_simultaneous_events in &self.potential_simultaneous_events_sequence {
			counter += timer.calculate_duration_of_ticks(1);

			if let Some(simultaneous_events) = potential_simultaneous_events {
				for event in &simultaneous_events.events {
					if let Event::Meta(MetaEvent::Tempo(val)) = event {
						timer.change_tempo(u32::from(*val));
					}
				}
			}
		}

		counter
	}
	pub fn calculate_duration_until(&self, timer:&mut Timer, index:usize) -> Duration {
		if timer.get_speed() == 0.0 {
			return Duration::MAX
		}

		let mut counter = Duration::default();
		for (event_index, potential_simultaneous_events) in self.potential_simultaneous_events_sequence.iter().enumerate() {
			if event_index == index {
				break;
			}

			counter += timer.calculate_duration_of_ticks(1);

			if let Some(simultaneous_events) = potential_simultaneous_events {
				for event in &simultaneous_events.events {
					if let Event::Meta(MetaEvent::Tempo(val)) = event {
						timer.change_tempo(u32::from(*val));
					}
				}
			}
		}

		counter
	}
}

impl Track {
	pub fn get_tempo_at(&self, index:usize) -> Option<(usize, &u24)> {
		if self.potential_simultaneous_events_sequence.len() <= index {
			return None;
		}

		self.potential_simultaneous_events_sequence[0..index]
			.iter()
			.enumerate()
			.filter_map(|(index, potential_simultaneous_event)| {
				if let Some(simultaneous_event) = potential_simultaneous_event {
					simultaneous_event.events.iter().find_map(|event| {
						if let Event::Meta(MetaEvent::Tempo(microseconds_per_beat)) = event {
							Some((index, microseconds_per_beat))
						} else {
							None
						}
					})
				} else {
					None
				}
			})
			.last()
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