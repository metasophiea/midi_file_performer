#![doc = include_str!("doc_lib.md")]

#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]

use std::{thread::JoinHandle, time::Duration};

use midly::Smf;

mod sleep;
mod timer;
use timer::Timer;
mod score;
use score::{Event, Score};
pub use score::Error as ScoreError;
pub use score::MidiEvent;
mod messages;
use messages::{ToConsole, ToEngine};
mod engine;
use engine::engine;
pub use engine::Error as EngineError;
mod error;
pub use error::Error;

/// A struct for playing MIDI scores.
pub struct Performer {
	score: Score,

	engine_thread_handle: Option<JoinHandle<Result<(), EngineError>>>,
	channel_to_engine: crossbeam_channel::Sender<ToEngine>,
	channel_from_engine: crossbeam_channel::Receiver<ToConsole>,

	is_playing: bool,
	position: usize,
	speed: f32,
	looping: bool
}

impl Performer {
	/// Create an instance of a [Performer] using the provided [Smf] data
	///
	/// # Errors
	/// Will return an error if the timing value of the [Smf] data's header is not [`midly::Timing::Metrical`].
	pub fn try_new(standard_midi_file:Smf) -> Result<Performer, Error> {
		let score = Score::new(&standard_midi_file)?;

		let (channel_to_engine, channel_from_console) = crossbeam_channel::unbounded::<ToEngine>();
		let (channel_to_console, channel_from_engine) = crossbeam_channel::unbounded::<ToConsole>();

		let score_clone = score.clone();
		let engine_thread_handle = Some(
			std::thread::spawn(move || {
				engine(
					&channel_from_console,
					&channel_to_console,
					&score_clone,
					standard_midi_file.header.timing
				)
			})
		);

		Ok(
			Performer {
				score,

				engine_thread_handle,
				channel_to_engine,
				channel_from_engine,

				is_playing: false,
				position: 0,
				speed: 1.0,
				looping: false
			}
		)
	
	}

	/// Create an instance of a [Performer] using the provided [Smf] data.
	///
	/// # Panics
	/// Will panic if the timing value of the [Smf] data's header is not [`midly::Timing::Metrical`].
	pub fn new(standard_midi_file:Smf) -> Performer {
		Performer::try_new(standard_midi_file).ok().unwrap()
	}
}

impl Performer {
	pub fn get_track_count(&self) -> usize {
		self.score.get_track_count()
	}

	pub fn is_playing(&self) -> bool {
		self.is_playing
	}
	pub fn get_speed(&self) -> f32 {
		self.speed
	}
	pub fn is_looping(&self) -> bool {
		self.looping
	}

	pub fn get_length_in_ticks(&self) -> usize {
		self.score.len()
	}
	pub fn get_position_in_ticks(&self) -> usize {
		self.position
	}
	pub fn get_length_in_duration(&self) -> Duration {
		self.score.calculate_duration(self.speed)
	}
	pub fn get_position_in_duration(&self) -> Duration {
		self.score.calculate_duration_until(self.speed, self.position)
	}
	
	pub fn get_current_microseconds_per_beat(&self) -> usize {
		let position = if self.position >= self.score.len() {
			self.score.len() - 1
		} else {
			self.position
		};

		u32::from(
			self.score.get_microseconds_per_beat_at(position)
				.unwrap_or_else(|| panic!("position is either beyond the length of the score (which we already checked for) or the score does not contain any tempo messages"))
		) as usize
	}
	pub fn get_current_bpm(&self) -> f32 {
		1.0 / ((self.get_current_microseconds_per_beat() as f32 / 1_000_000.0) / 60.0)
	}
}

impl Performer {
	/// Instruct the engine to begin playing the midi score.
	///
	/// # Errors
	/// Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn play(&mut self) -> Result<(), Error> {
		self.is_playing = true;
		if let Err(err) = self.channel_to_engine.send(ToEngine::Play) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}

	/// Instruct the engine to pause playing the midi score.
	///
	/// # Errors
	/// Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn pause(&mut self) -> Result<(), Error> {
		self.is_playing = false;
		if let Err(err) = self.channel_to_engine.send(ToEngine::Pause) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}

	/// Instruct the engine to stop playing the midi score, returning the playhead to position 0.
	///
	/// # Errors
	/// Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn stop(&mut self) -> Result<(), Error> {
		self.is_playing = false;
		if let Err(err) = self.channel_to_engine.send(ToEngine::Stop) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}

	/// Instruct the engine to jump to a certain position in the midi score.
	///
	/// # Errors
	/// Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn jump_to(&mut self, position:usize) -> Result<(), Error> {
		if position >= self.score.len() {
			return Err(Error::BeyondScoreLength);
		}

		self.position = position;

		if let Err(err) = self.channel_to_engine.send(ToEngine::JumpTo(position)) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}

	/// Set the playback speed (as a multiple of the tempo defined in the midi score)
	///
	/// # Errors
	/// - Will return an [`Error::NegativeSpeed`] if one attempts to set the speed to a negative number.
	/// - Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn set_speed(&mut self, speed:f32) -> Result<(), Error> {
		if speed < 0.0 {
			return Err(Error::NegativeSpeed);
		}

		self.speed = speed;

		if let Err(err) = self.channel_to_engine.send(ToEngine::SetSpeed(speed)) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}

	/// Instruct the engine to return to the beginning of the midi score when it reaches the end.
	///
	/// # Errors
	/// - Will return an [`Error::Communication`] if there is a communication issue with the engine.
	pub fn set_looping(&mut self, looping:bool) -> Result<(), Error> {
		self.looping = looping;

		if let Err(err) = self.channel_to_engine.send(ToEngine::SetLooping(looping)) {
			Err(Error::Communication(err))
		} else {
			Ok(())
		}
	}
}

impl Performer {
	/// Poll for playback messages from the engine.
	/// 
	/// Assuming an "Ok" result, this method returns either;
	/// - A "None" value, indicating that there are no midi messages to be addressed.
	/// - A "Some" value, holding an vector of [`MidiEvent`]s along with the track number they are associated with.
	///
	/// # Errors
	/// - Will return an [`Error::NoEngine`] if there is an issue with the engine thread handle.
	/// - Will return an [`Error::Engine`] if engine has stopped and due to an issue encountered by the engine.
	/// - Will return an [`Error::Thread`] if engine has stopped and there is an issue with "joining" the engine thread handle.
	pub fn poll(&mut self) -> Result<Option<Vec<(usize, MidiEvent)>>, Error> {
		//engine check
			let Some(engine_thread_handle) = &mut self.engine_thread_handle else {
				return Err(Error::NoEngine);
			};
			
			if engine_thread_handle.is_finished() {
				return match std::mem::take(&mut self.engine_thread_handle).expect("we've already established that engine_thread_handle is present").join() {
					Ok(result) => {
						match result {
							Ok(()) => Ok(None),
							Err(err) => Err(Error::Engine(err)),
						}
					},
					Err(err) => Err(Error::Thread(err))
				};
			}

		//deal with messages
			Ok(
				Some(
					self.channel_from_engine
						.try_iter()
						.filter_map(|message| {
							match message {
								ToConsole::Event(track, event) => {
									match event {
										Event::Midi(midi_message) => Some((track, midi_message)),
										_ => None
									}
								}
								ToConsole::Stopped => {
									self.is_playing = false;
									None
								}
								ToConsole::PositionUpdate(new_position) => {
									self.position = new_position;
									None
								}
							}
						})
						.collect()
				)
			)
	}
}

impl Drop for Performer {
	fn drop(&mut self) {
		self.channel_to_engine.send(ToEngine::Halt).ok();
	}
}