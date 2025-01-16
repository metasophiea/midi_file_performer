use crate::score::Event;

pub enum ToConsole {
	Event(usize, Event),
	PositionUpdate(usize),
	Stopped
}

#[derive(Debug)]
pub enum ToEngine {
	Halt,
	Play,
	Pause,
	Stop,
	JumpTo(usize),
	SetLooping(bool),
	SetSpeed(f32)
}