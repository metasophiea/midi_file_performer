mod constant_tempo {
	use std::time::Duration;

	use midly::num::u24;
	
	use super::super::Score;

	static MID_FILE_DATA:&[u8] = include_bytes!("../../test_midi_files/constant_tempo.mid");

	#[test]
	pub fn len() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();
		assert_eq!(score.len(), 15361);
	}

	#[test]
	pub fn get_microseconds_per_beat_changes() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();
		
		assert_eq!(
			score.microseconds_per_beat_changes,
			[
				(0, u24::new(500000)), //120
			]
		);
	}

	#[test]
	pub fn get_tempo_at_zero() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(0), Some(u24::new(500000)));
	}

	#[test]
	pub fn get_tempo_at_beyond() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(1_000_000), None);
	}

	#[test]
	pub fn get_tempo_at_intermediate() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(0 + 1), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(3840), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(5760), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(7680), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(8640), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(9600), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(10560), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(11520), Some(u24::new(500000)));
	}

	#[test]
	pub fn calculate_duration() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_duration(1.0),
			Duration::from_secs_f64(15.990800858)
		);
	}

	#[test]
	pub fn calculate_duration_until() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_duration_until(1.0, 3841),
			Duration::from_secs_f64(3.998481035)
		);
	}

	#[test]
	pub fn calculate_ticks_until_next_events_from_index_1() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_ticks_until_next_events_from_index(0),
			Some(960)
		);
	}

	#[test]
	pub fn calculate_ticks_until_next_events_from_index_2() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_ticks_until_next_events_from_index(1),
			Some(959)
		);
	}

	#[test]
	pub fn calculate_ticks_until_next_events_from_index_3() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_ticks_until_next_events_from_index(959),
			Some(1)
		);
	}

	#[test]
	pub fn calculate_ticks_until_next_events_from_index_4() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_ticks_until_next_events_from_index(960),
			Some(960)
		);
	}
}

mod changing_tempo {
	use std::time::Duration;

	use midly::num::u24;
	
	use super::super::Score;

	static MID_FILE_DATA:&[u8] = include_bytes!("../../test_midi_files/changing_tempo.mid");

	#[test]
	pub fn len() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();
		assert_eq!(score.len(), 15361);
	}

	#[test]
	pub fn get_microseconds_per_beat_changes() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();
		
		assert_eq!(
			score.microseconds_per_beat_changes,
			[
				(0, u24::new(500000)), //120
				(3840, u24::new(428571)), //140
				(5760, u24::new(500000)), //120
				(7680, u24::new(600000)), //100
				(8640, u24::new(545455)), //110
				(9600, u24::new(500000)), //120
				(10560, u24::new(461538)), //130
				(11520, u24::new(500000)), //120
			]
		);
	}

	#[test]
	pub fn get_tempo_at_zero() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(0), Some(u24::new(500000)));
	}

	#[test]
	pub fn get_tempo_at_beyond() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(1_000_000), None);
	}

	#[test]
	pub fn get_tempo_at_exact() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(0), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(3840), Some(u24::new(428571)));
		assert_eq!(score.get_microseconds_per_beat_at(5760), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(7680), Some(u24::new(600000)));
		assert_eq!(score.get_microseconds_per_beat_at(8640), Some(u24::new(545455)));
		assert_eq!(score.get_microseconds_per_beat_at(9600), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(10560), Some(u24::new(461538)));
		assert_eq!(score.get_microseconds_per_beat_at(11520), Some(u24::new(500000)));
	}

	#[test]
	pub fn get_tempo_at_intermediate() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(score.get_microseconds_per_beat_at(0 + 1), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(3840 + 1), Some(u24::new(428571)));
		assert_eq!(score.get_microseconds_per_beat_at(5760 + 1), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(7680 + 1), Some(u24::new(600000)));
		assert_eq!(score.get_microseconds_per_beat_at(8640 + 1), Some(u24::new(545455)));
		assert_eq!(score.get_microseconds_per_beat_at(9600 + 1), Some(u24::new(500000)));
		assert_eq!(score.get_microseconds_per_beat_at(10560 + 1), Some(u24::new(461538)));
		assert_eq!(score.get_microseconds_per_beat_at(11520 + 1), Some(u24::new(500000)));
	}

	#[test]
	pub fn calculate_duration() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_duration(1.0),
			Duration::from_secs_f64(15.919760883)
		);
	}

	#[test]
	pub fn calculate_duration_until() {
		let standard_midi_file = midly::Smf::parse(MID_FILE_DATA).unwrap();
		let score = Score::new(&standard_midi_file).ok().unwrap();

		assert_eq!(
			score.calculate_duration_until(1.0, 3841),
			Duration::from_secs_f64(3.998331861)
		);
	}
}