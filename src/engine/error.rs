use crossbeam_channel::SendError;

use crate::ToConsole;

pub enum Error {
	/// The timer could not be created due to the use of an incompatible timing value.
	TimerCreation,
	/// An issue related to the communication channels between the console and engine.
	Channel(SendError<ToConsole>)
}