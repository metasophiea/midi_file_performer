use std::time::{Duration, Instant};

use midly::Timing;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Timer {
	ticks_per_beat: u16,
	micros_per_tick: f64,
	last_instant: Option<Instant>,
	
	speed: f32,
}

impl Timer {
	/// Create an instance of a [Timer] with the given ticks-per-beat.
	///
	/// Initially the tempo will be set to infinity. This is rarely an issue as a tempo change
	/// message will set it, which is usual found in the first tick of a score.
	pub fn new(ticks_per_beat:u16) -> Timer {
		Timer {
			ticks_per_beat,
			micros_per_tick: 0.0,
			last_instant: None,

			speed: 1.0,
		}
	}
}

impl TryFrom<Timing> for Timer {
	type Error = ();

	/// Tries to create a [Timer] from the provided [Timing].
	///
	/// # Errors
	/// Will return an error if the given [Timing] is not [`Timing::Metrical`].
	fn try_from(t: Timing) -> Result<Timer, Self::Error> {
		match t {
			Timing::Metrical(n) => Ok(Timer::new(u16::from(n))),
			Timing::Timecode(_frames_per_second, _ticks_per_frame) => Err(()),
		}
	}
}

impl Timer {
	pub(super) fn change_tempo(&mut self, tempo:u32) {
		self.micros_per_tick = f64::from(tempo) / f64::from(self.ticks_per_beat);
	}
}

impl Timer {
	pub fn calculate_duration_of_ticks(&self, ticks:usize) -> Duration {
		if self.speed == 0.0 {
			return Duration::MAX;
		}

		let microseconds = (self.micros_per_tick * (ticks as f64)) / f64::from(self.speed);

		if microseconds > 0.0 {
			Duration::from_micros(microseconds as u64)
		} else {
			Duration::default()
		}
	}

	pub fn calculate_number_of_ticks_that_would_fit_within_duration(&self, duration:Duration) -> usize {
		let duration_of_one_tick = self.calculate_duration_of_ticks(1);

		if duration_of_one_tick >= duration {
			return 0;
		}

		duration.div_duration_f64(duration_of_one_tick).trunc() as usize - 1
	}

	pub fn calculate_sleeping_time(&mut self, ticks:usize) -> Duration {
		let mut duration = self.calculate_duration_of_ticks(ticks);

		match self.last_instant {
			Some(last_instant) => {
				self.last_instant = Some(last_instant + duration);
				duration = duration.checked_sub(last_instant.elapsed()).unwrap_or(duration);
			}
			None => self.last_instant = Some(Instant::now()),
		}

		duration
	}
}

impl Timer {
	pub fn get_speed(&self) -> f32 {
		self.speed
	}
	pub fn set_speed(&mut self, speed:f32) {
		self.speed = speed;
	}
}