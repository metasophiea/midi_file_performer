use std::time::Duration;

use midly::{num::u24, Format, Smf, Timing, TrackEvent};

use super::Timer;

mod midi_event;
mod meta_event;
mod event;
mod simultaneous_events;
mod track;
mod error;

#[cfg(test)]
mod tests;

pub use midi_event::MidiEvent;
pub use meta_event::MetaEvent;
pub use event::Event;
pub use simultaneous_events::SimultaneousEvents;
use track::Track;
pub use error::Error;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Score {
	timing: Timing,
	tracks: Vec<Track>,

	microseconds_per_beat_changes: Vec<(usize, u24)>
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

		let tracks = match standard_midi_file.header.format {
			Format::SingleTrack | Format::Parallel=> Score::parallel(&standard_midi_file.tracks),
			Format::Sequential => Score::sequential(&standard_midi_file.tracks),
		};

		let mut microseconds_per_beat_changes:Vec<(usize, u24)> = tracks
			.iter()
			.map(|track| track.get_all_tempos().to_vec())
			.flatten()
			.collect();
		microseconds_per_beat_changes.sort_by_key(|tempo| tempo.0);
		if microseconds_per_beat_changes.is_empty() {
			return Err(Error::NoTempo);
		}

		Ok(
			Score {
				timing: standard_midi_file.header.timing,
				tracks,
				microseconds_per_beat_changes,
			}
		)
	}
}

impl Score {
	pub fn get_track_count(&self) -> usize {
		self.tracks.len()
	}
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
		//protection
			if speed == 0.0 {
				return Duration::MAX;
			}

		//create new timer
			let mut working_timer = Timer::new(self.timing).expect("we ensured that the timing format was compatible in the \"new\" method");
			working_timer.set_speed(speed);

		//calculate
			let mut counter = Duration::default();
			let mut last_index = 0;
			for (index, microseconds_per_beat) in &self.microseconds_per_beat_changes {
				counter += working_timer.calculate_duration_of_ticks(*index - last_index);
				last_index = *index;
				working_timer.change_tempo(u32::from(*microseconds_per_beat));
			}

			counter += working_timer.calculate_duration_of_ticks(self.len() - last_index);

		counter
	}

	pub fn calculate_duration_until(&self, speed:f32, index_limit:usize) -> Duration {
		//protection
			if speed == 0.0 {
				return Duration::MAX;
			}

		//create new timer
			let mut working_timer = Timer::new(self.timing).expect("we ensured that the timing format was compatible in the \"new\" method");
			working_timer.set_speed(speed);

		//calculate
			let mut counter = Duration::default();
			let mut last_index = 0;
			for (index, microseconds_per_beat) in &self.microseconds_per_beat_changes {
				if index >= &index_limit {
					break;
				}

				counter += working_timer.calculate_duration_of_ticks(*index - last_index);
				last_index = *index;
				working_timer.change_tempo(u32::from(*microseconds_per_beat));
			}

			counter += working_timer.calculate_duration_of_ticks(index_limit - last_index);

		counter
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
	pub fn get_microseconds_per_beat_at(&self, index:usize) -> Option<u24> {
		if index == 0 {
			self.microseconds_per_beat_changes.get(0).map(|(_, tempo)| *tempo)
		} else if index >= self.len() {
			None
		} else if self.microseconds_per_beat_changes.len() == 1 {
			self.microseconds_per_beat_changes.get(0).map(|(_, tempo)| *tempo)
		} else {
			self.microseconds_per_beat_changes
				.iter()
				.filter(|(tempo_index, _)| tempo_index <= &index)
				.last()
				.map(|(_, tempo)| *tempo)
		}
	}

	pub fn calculate_ticks_until_next_events_from_index(&self, index:usize) -> Option<usize> {
		self.tracks
			.iter()
			.filter_map(|track| track.calculate_ticks_until_next_events_from_index(index))
			.min()
	}
}