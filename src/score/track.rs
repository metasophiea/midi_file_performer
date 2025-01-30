use midly::num::u24;
use midly::{MetaMessage, TrackEvent, TrackEventKind};

use super::meta_event::MetaEvent;
use super::Event;
use super::simultaneous_events::SimultaneousEvents;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Track {
	// eg. [[Event], none, none, none, [Event, Event], none, none, [Event]]
	potential_simultaneous_events_sequence: Vec<Option<SimultaneousEvents>>,
	all_tempos: Vec<(usize, u24)>
}

impl From<&Vec<TrackEvent<'_>>> for Track {
	fn from(track_events:&Vec<TrackEvent<'_>>) -> Track {
		//create vector
			let total_ticks = track_events
				.iter()
				.map(|track_event| {
					//correct "EndOfTrack being too far out" situation
					if TrackEventKind::Meta(MetaMessage::EndOfTrack) == track_event.kind {
						0
					} else {
						u32::from(track_event.delta) as usize
					}
				})
				.sum::<usize>()
				+ 1;
			let mut potential_simultaneous_events_sequence:Vec<Option<SimultaneousEvents>> = vec![None; total_ticks];
			
		//populate vector
			let mut position = 0;
			for track_event in track_events {
				if TrackEventKind::Meta(MetaMessage::EndOfTrack) == track_event.kind {
					continue;
				}

				position += u32::from(track_event.delta) as usize;
				if let Ok(event) = Event::try_from(track_event.kind) {
					if let Some(simultaneous_events) = &mut potential_simultaneous_events_sequence[position] {
						simultaneous_events.push(event);
					} else {
						potential_simultaneous_events_sequence[position] = Some(SimultaneousEvents::new(vec![event]));
					}
				}
			}
		
		//all_tempos
			let all_tempos = potential_simultaneous_events_sequence
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
			potential_simultaneous_events_sequence,
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
	pub(super) fn calculate_ticks_until_next_event_codex(&self) -> Vec<usize> {
		// eg. [[Event], none, none, none, [Event, Event], none, none, [Event]]
		//   > [4, 3, 2, 1, 3, 2, 1, 0]

		let mut ticks_until_next_events_compressed_codex = vec![];
		let mut counter = 0;
		for event in &self.potential_simultaneous_events_sequence {
			if event.is_some() {
				ticks_until_next_events_compressed_codex.push(counter);
				counter = 1;
			} else {
				counter += 1;
			}
		}

		let mut ticks_until_next_events_codex = vec![];
		for distance in ticks_until_next_events_compressed_codex {
			let mut working_distance = distance;
			while working_distance > 0 {
				ticks_until_next_events_codex.push(working_distance);
				working_distance -= 1;
			}
		}

		ticks_until_next_events_codex
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
}