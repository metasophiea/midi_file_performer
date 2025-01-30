use std::{ops::Mul, time::{Duration, Instant}};

use midly::Timing;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Timer {
	ticks_per_beat: u16,
	tick_duration: Duration,
	maximum_sleep_time: Duration,
	number_of_ticks_that_would_fit_into_maximum_sleep_time: usize,
	last_instant: Option<Instant>,
	
	speed: f32,
}

impl Timer {
	/// Create an instance of a [Timer] with the given ticks-per-beat.
	///
	/// Initially the tempo will be set to infinity. This is rarely an issue as a tempo change
	/// message will set it, which is usual found in the first tick of a score.
	pub fn new_with_ticks_per_beat(ticks_per_beat:u16, maximum_sleep_time:Duration) -> Timer {
		Timer {
			ticks_per_beat,
			tick_duration: Duration::ZERO,
			maximum_sleep_time,
			number_of_ticks_that_would_fit_into_maximum_sleep_time: 0,
			last_instant: None,

			speed: 1.0,
		}
	}

	/// Create an instance of a [Timer] with the given [Timing].
	///
	/// Initially the tempo will be set to infinity. This is rarely an issue as a tempo change
	/// message will set it, which is usual found in the first tick of a score.
	pub fn new(timing:Timing, maximum_sleep_time:Duration) -> Result<Timer, ()> {
		let ticks_per_beat = match timing {
			Timing::Metrical(n) => u16::from(n),
			Timing::Timecode(_frames_per_second, _ticks_per_frame) => {
				return Err(())
			},
		};

		Ok(Timer::new_with_ticks_per_beat(ticks_per_beat, maximum_sleep_time))
	}	
}

impl Timer {
	pub fn change_tempo(&mut self, tempo:u32) {
		self.tick_duration = Duration::from_micros(u64::from(tempo / u32::from(self.ticks_per_beat)));
		self.number_of_ticks_that_would_fit_into_maximum_sleep_time = self.maximum_sleep_time.div_duration_f32(self.tick_duration).trunc() as usize;
	}
}

impl Timer {
	pub fn get_number_of_ticks_that_would_fit_into_maximum_sleep_time(&self) -> usize {
		self.number_of_ticks_that_would_fit_into_maximum_sleep_time
	}
	
	pub fn calculate_duration_of_ticks(&self, ticks:usize) -> Duration {
		if self.speed == 0.0 {
			return Duration::MAX;
		}

		self.tick_duration.mul(u32::try_from(ticks).unwrap_or(u32::MAX)).div_f32(self.speed)
	}

	pub fn calculate_sleeping_time(&mut self, ticks:usize) -> Duration {
		let mut duration = self.calculate_duration_of_ticks(ticks);

		match self.last_instant {
			Some(last_instant) => {
				self.last_instant = Some(last_instant + duration);
				duration = duration.checked_sub(last_instant.elapsed()).unwrap_or(duration);
			}
			None => self.last_instant = Some(Instant::now() + duration),
		}

		duration
	}
}

impl Timer {
	pub fn set_speed(&mut self, speed:f32) {
		self.speed = speed;
	}
}