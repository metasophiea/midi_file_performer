#[derive(Debug)]
pub enum Error {
	/// An incompatible timing value was supplied.
	TimingFormat,
	/// This midi file does not contain any tempo information.
	NoTempo
}