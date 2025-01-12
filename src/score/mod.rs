use std::time::Duration;

use midly::{num::u24, Format, Smf, Timing, TrackEvent};

use super::Timer;

mod midi_event;
mod meta_event;
mod event;
mod simultaneous_events;
mod track;
mod error;

pub use midi_event::MidiEvent;
pub use meta_event::MetaEvent;
pub use event::Event;
pub use simultaneous_events::SimultaneousEvents;
use track::Track;
pub use error::Error;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Score {
	timing: Timing,
	tracks: Vec<Track>
}

impl Score {
	fn parallel(tracks:&[Vec<TrackEvent<'_>>]) -> Vec<Track> {
		tracks.iter().map(std::convert::Into::into).collect()
	}
	fn sequential(tracks:&[Vec<TrackEvent<'_>>]) -> Vec<Track> {
		let output_track:Track = tracks
			.iter()
			.skip(1)
			.fold(
				(&tracks[0]).into(),
				|mut accumulator, track| {
					accumulator.append(&mut track.into());
					accumulator
				}
			);

		vec![output_track]
	}

	pub fn new(standard_midi_file:&Smf) -> Result<Score, Error> {
		if let Timing::Timecode(_, _) = standard_midi_file.header.timing {
			return Err(Error::TimingFormat);
		}

		Ok(
			Score {
				timing: standard_midi_file.header.timing,
				tracks: match standard_midi_file.header.format {
					Format::SingleTrack | Format::Parallel=> Score::parallel(&standard_midi_file.tracks),
					Format::Sequential => Score::sequential(&standard_midi_file.tracks),
				}
			}
		)
	}
}

impl Score {
	pub fn len(&self) -> usize {
		self.tracks	
			.iter()
			.map(Track::len)
			.max()
			.unwrap_or(0)
	}
}

impl Score {
	pub fn calculate_duration(&self, speed:f32) -> Duration {
		//create new timer
			let mut working_timer = Timer::try_from(self.timing).expect("we ensured that the timing format was compatible in the \"new\" method");
			working_timer.set_speed(speed);

		//calculate the length of each track and return the biggest value
			self.tracks
				.iter()
				.map(|track| track.calculate_duration(&mut working_timer))
				.max()
				.unwrap_or(Duration::default())
	}

	pub fn calculate_duration_until(&self, speed:f32, index:usize) -> Duration {
		//create new timer
			let mut working_timer = Timer::try_from(self.timing).expect("we ensured that the timing format was compatible in the \"new\" method");
			working_timer.set_speed(speed);

		//calculate the length of each track and return the biggest value
			self.tracks
				.iter()
				.map(|track| track.calculate_duration_until(&mut working_timer, index))
				.max()
				.unwrap_or(Duration::default())
	}

	pub fn gather_all_events_for_index(&self, index:usize) -> Option<Vec<(usize, &SimultaneousEvents)>> {
		let potential_simultaneous_events_for_index_with_track_index:Vec<(usize, &Option<SimultaneousEvents>)> = self.tracks
			.iter()
			.enumerate()
			.filter_map(|(track_index, track)| 
				track
					.get_events(index)
					.map(|potential_simultaneous_events| (track_index, potential_simultaneous_events))
			)
			.collect();

		if potential_simultaneous_events_for_index_with_track_index.is_empty() {
			None
		} else {
			Some(
				potential_simultaneous_events_for_index_with_track_index
					.into_iter()
					.filter_map(|(track_index, potential_simultaneous_events)| 
						potential_simultaneous_events
							.as_ref()
							.map(|simultaneous_events| (track_index, simultaneous_events))
					)
					.collect()
			)
		}
	}
}

impl Score {
	pub fn get_tempo_at(&self, index:usize) -> Option<&u24> {
		if let Some((_, tempo)) = self.tracks
			.iter()
			.filter_map(|track| track.get_tempo_at(index))
			.max_by_key(|(index, _)| *index)
		{
			Some(tempo)
		} else {
			None
		}
	}

	pub fn calculate_ticks_until_next_events_from_index(&self, index:usize) -> Option<usize> {
		self.tracks
			.iter()
			.filter_map(|track| track.calculate_ticks_until_next_events_from_index(index))
			.min()
	}
}

// impl Score {
// 	pub fn print(&self) {
// 		println!("score: ({})", self.len());
// 		let moments = self.get_moments();
// 		let mut index = 0;
// 		while index < moments.len() {
// 			let mut empty_count = 0;
// 			while moments[index].is_empty() {
// 				empty_count += 1;
// 				index += 1;
// 			}
// 			if empty_count != 0 {
// 				println!("\t{empty_count} empty moments");
// 			}

// 			println!("\t{:?}", moments[index]);

// 			index += 1;
// 		}
// 	}
// }