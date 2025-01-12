use midly::{
	num::{u24, u4, u7},
	MetaMessage,
	SmpteTime
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MetaEvent {
	/// For `Format::Sequential` MIDI file types, `TrackNumber` can be empty, and defaults to
	/// the track index.
	TrackNumber(Option<u16>),
	/// Arbitrary text associated to an instant.
	Text(Vec<u8>),
	/// A copyright notice.
	Copyright(Vec<u8>),
	/// Information about the name of the track.
	TrackName(Vec<u8>),
	/// Information about the name of the current instrument.
	InstrumentName(Vec<u8>),
	/// Arbitrary lyric information associated to an instant.
	Lyric(Vec<u8>),
	/// Arbitrary marker text associated to an instant.
	Marker(Vec<u8>),
	/// Arbitrary cue point text associated to an instant.
	CuePoint(Vec<u8>),
	/// Information about the name of the current program.
	ProgramName(Vec<u8>),
	/// Name of the device that this file was intended to be played with.
	DeviceName(Vec<u8>),
	/// Number of the MIDI channel that this file was intended to be played with.
	MidiChannel(u4),
	/// Number of the MIDI port that this file was intended to be played with.
	MidiPort(u7),
	/// Obligatory at track end.
	EndOfTrack,
	/// Amount of microseconds per beat (quarter note).
	///
	/// Usually appears at the beggining of a track, before any midi events are sent, but there
	/// are no guarantees.
	Tempo(u24),
	/// The MIDI SMPTE offset meta message specifies an offset for the starting point of a MIDI
	/// track from the start of a sequence in terms of SMPTE time (hours:minutes:seconds:frames:subframes).
	///
	/// [Reference](https://www.recordingblogs.com/wiki/midi-smpte-offset-meta-message)
	SmpteOffset(SmpteTime),
	/// In order of the MIDI specification, numerator, denominator, MIDI clocks per click, 32nd
	/// notes per quarter
	TimeSignature(u8, u8, u8, u8),
	/// As in the MIDI specification, negative numbers indicate number of flats and positive
	/// numbers indicate number of sharps.
	/// `false` indicates a major scale, `true` indicates a minor scale.
	KeySignature(i8, bool),
	/// Arbitrary data intended for the sequencer.
	/// This data is never sent to a device.
	SequencerSpecific(Vec<u8>),
	/// An unknown or malformed meta-message.
	///
	/// The first `u8` is the raw meta-message identifier byte.
	/// The slice is the actual payload of the meta-message.
	Unknown(u8, Vec<u8>),
}

impl From<MetaMessage<'_>> for MetaEvent {
	fn from(meta_message:MetaMessage<'_>) -> MetaEvent {
		match meta_message {
			MetaMessage::TrackNumber(number) => MetaEvent::TrackNumber(number),
			MetaMessage::Text(data) => MetaEvent::Text(data.to_vec()),
			MetaMessage::Copyright(data) => MetaEvent::Copyright(data.to_vec()),
			MetaMessage::TrackName(data) => MetaEvent::TrackName(data.to_vec()),
			MetaMessage::InstrumentName(data) => MetaEvent::InstrumentName(data.to_vec()),
			MetaMessage::Lyric(data) => MetaEvent::Lyric(data.to_vec()),
			MetaMessage::Marker(data) => MetaEvent::Marker(data.to_vec()),
			MetaMessage::CuePoint(data) => MetaEvent::CuePoint(data.to_vec()),
			MetaMessage::ProgramName(data) => MetaEvent::ProgramName(data.to_vec()),
			MetaMessage::DeviceName(data) => MetaEvent::DeviceName(data.to_vec()),
			MetaMessage::MidiChannel(channel) => MetaEvent::MidiChannel(channel),
			MetaMessage::MidiPort(port) => MetaEvent::MidiPort(port),
			MetaMessage::EndOfTrack => MetaEvent::EndOfTrack,
			MetaMessage::Tempo(tempo) => MetaEvent::Tempo(tempo),
			MetaMessage::SmpteOffset(offset) => MetaEvent::SmpteOffset(offset),
			MetaMessage::TimeSignature(numerator, denominator, midi_clocks_per_click, thirty_second_nd_notes_per_quarter) => MetaEvent::TimeSignature(numerator, denominator, midi_clocks_per_click, thirty_second_nd_notes_per_quarter),
			MetaMessage::KeySignature(flats_or_sharps, is_minor_scale) => MetaEvent::KeySignature(flats_or_sharps, is_minor_scale),
			MetaMessage::SequencerSpecific(data) => MetaEvent::SequencerSpecific(data.to_vec()),
			MetaMessage::Unknown(identifier, data) => MetaEvent::Unknown(identifier, data.to_vec()),
		}
	}
}