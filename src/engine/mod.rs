use std::time::Duration;

use midly::Timing;

use crate::{
	messages::{ToConsole, ToEngine},
	score::{Event, MetaEvent, Score},
	timer::Timer
};

mod error;
pub use error::Error;

pub fn engine(
	channel_from_console: &crossbeam_channel::Receiver<ToEngine>,
	channel_to_console: &crossbeam_channel::Sender<ToConsole>,
	score: &Score,
	timing: Timing
) -> Result<(), Error> {
	//timer
		let mut timer = match Timer::try_from(timing) {
			Ok(timer) => timer,
			Err(_err) => {
				return Err(Error::TimerCreation)
			}
		};

	//loop control
		let mut halt = false;

	//performance control
		let mut play = false;
		let mut position:usize = 0;
		let mut speed:f32 = 1.0;

	while !halt {
		channel_from_console.try_iter().for_each(|message| {
			match message {
				ToEngine::Halt => halt = true,
				ToEngine::Play => play = true,
				ToEngine::Pause => play = false,
				ToEngine::Stop => {
					play = false;
					position = 0;
					if let Some(microseconds_per_beat) = score.get_microseconds_per_beat_at(position) {
						timer.change_tempo(u32::from(microseconds_per_beat));
					}
				},
				ToEngine::JumpTo(new_position) => {
					position = new_position;
					if let Some(microseconds_per_beat) = score.get_microseconds_per_beat_at(position) {
						timer.change_tempo(u32::from(microseconds_per_beat));
					}
				},
				ToEngine::SetSpeed(new_speed) => {
					if new_speed > 0.0 {
						speed = new_speed;
						timer.set_speed(speed);
					}
				},
			}
		});

		if play {
			if let Some(simultaneous_events_per_track) = score.gather_all_events_for_index(position) {
				//transmit position
					if let Err(err) = channel_to_console.send(ToConsole::PositionUpdate(position)) {
						return Err(Error::Channel(err))
					}

				//process events for this position
					for (track_index, simultaneous_events) in simultaneous_events_per_track {
						for event in &simultaneous_events.events {
							if let Event::Meta(MetaEvent::Tempo(microseconds_per_beat)) = event {
								timer.change_tempo(u32::from(*microseconds_per_beat));
							}

							if let Err(err) = channel_to_console.send(ToConsole::Event(track_index, event.clone())) {
								return Err(Error::Channel(err))
							}
						}
					}

				//calculate sleep until next event
					let ticks = if let Some(full_ticks) = score.calculate_ticks_until_next_events_from_index(position) {
						let sub_millisecond_ticks = timer.calculate_number_of_ticks_that_would_fit_within_duration(Duration::from_millis(10));
						full_ticks.min(sub_millisecond_ticks + 1)
					} else {
						1
					};

					let sleep_duration = timer.calculate_sleeping_time(ticks);
					position += ticks;
					std::thread::sleep(sleep_duration);

			} else {
				play = false;
				if let Err(err) = channel_to_console.send(ToConsole::Stopped) {
					return Err(Error::Channel(err))
				}
			}
		} else {
			std::thread::sleep(Duration::from_millis(1));
		}
	}

	Ok(())
}