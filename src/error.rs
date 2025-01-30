use std::any::Any;

use crossbeam_channel::SendError;

use crate::{
	engine::Error as EngineError,
	messages::ToEngine,
	score::Error as ScoreError
};

#[derive(Debug)]
pub enum Error {
	/// The selected position is beyond the bounds of the midi score.
	BeyondScoreLength,
	/// An issue related to the communication channels between the console and engine.
	Communication(SendError<ToEngine>),
	/// An [`EngineError`].
	Engine(EngineError),
	/// Returned when one attempts to set the playback speed to a negative number.
	NegativeSpeed,
	/// The engine thread is missing.
	NoEngine,
	/// The midi score does not contain any tempo messages.
	NoTempo,
	/// A [`ScoreError`].
	Score(ScoreError),
	/// An error returned by the [`std::thread::JoinHandle::join`] method of the thread holding the engine.
	Thread(Box<dyn Any + Send>),
}

impl From<ScoreError> for Error {
	fn from(score_error:ScoreError) -> Error {
		Error::Score(score_error)
	}
}