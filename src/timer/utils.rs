use std::time::Duration;

use super::Timer;

/// Calculate the duration of `n_ticks` ticks, without accounting for the last time this [Timer] ticked.
/// This is useful for calculating the duration of a song, for example.
pub fn sleep_duration_without_readjustment(timer:&Timer, n_ticks:u32) -> Duration {
	let t = timer.micros_per_tick * n_ticks as f64 / timer.speed as f64;

	if t > 0.0 {
		Duration::from_micros(t as u64)
	} else {
		Duration::default()
	}
}