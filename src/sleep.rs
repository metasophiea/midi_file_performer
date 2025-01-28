use std::time::{Duration, Instant};

use crate::messages::ToEngine;

pub fn sleep_and_message_check(
	duration: Duration,
	channel_from_console: &crossbeam_channel::Receiver<ToEngine>,
) -> Option<Vec<ToEngine>> {
	#[cfg(windows)]
	const LIMIT: Duration = Duration::from_millis(15);
	#[cfg(not(windows))]
	const LIMIT: Duration = Duration::from_millis(3);

	let mut messages = vec![];

	let remaining_duration = if duration < LIMIT {
		duration
	} else {
		let start = Instant::now();
		
		loop {
			match channel_from_console.recv_timeout(Duration::from_millis(1)) {
				Err(err) => {
					if err.is_disconnected() {
						return None;
					} 
				},
				Ok(message) => {
					messages.push(message);
					messages.append(&mut channel_from_console.try_iter().collect());
				}
			}

			let remaining = duration.saturating_sub(start.elapsed());
			if remaining < LIMIT {
				break remaining;
			}
		}
	};

	spin_lock(remaining_duration);

	Some(messages)
}

#[inline]
fn spin_lock(duration:Duration) {
	let start = std::time::Instant::now();
	while start.elapsed() < duration {
		std::hint::spin_loop();
	}
}