use crate::score::Event;

pub enum ToConsole {
	Event(usize, Event),
	Stopped,
	PositionUpdate(usize)
}

#[derive(Debug)]
pub enum ToEngine {
	Halt,
	Play,
	Pause,
	Stop,
	JumpTo(usize),
	SetSpeed(f32)
}