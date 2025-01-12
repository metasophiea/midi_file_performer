use std::any::Any;

use crossbeam_channel::SendError;

use crate::{
	engine::Error as EngineError,
	messages::ToEngine,
	score::Error as ScoreError
};

pub enum Error {
	/// An issue related to the communication channels between the console and engine.
	Communication(SendError<ToEngine>),
	/// An [`EngineError`].
	Engine(EngineError),
	/// Returned when one attempts to set the playback speed to a negative number.
	NegativeSpeed,
	/// The engine thread is missing.
	NoEngine,
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