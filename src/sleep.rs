use std::time::{Duration, Instant};

#[cfg(windows)]
const LIMIT:Duration = Duration::from_millis(15);
#[cfg(not(windows))]
const LIMIT:Duration = Duration::from_millis(3);

pub fn sleep(duration:Duration) {
	let remaining_duration = if duration < LIMIT {
		duration
	} else {
		let start = Instant::now();
		
		loop {
			std::thread::sleep(Duration::from_millis(1));
			let remaining = duration.saturating_sub(start.elapsed());
			if remaining < LIMIT {
				break remaining;
			}
		}
	};

	spin_lock(remaining_duration);
}

#[inline]
fn spin_lock(duration:Duration) {
	let start = std::time::Instant::now();
	while start.elapsed() < duration {
		std::hint::spin_loop();
	}
}