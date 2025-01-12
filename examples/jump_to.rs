use std::time::Duration;

use clap::Parser;
use midir::{os::unix::VirtualOutput, MidiOutput};

use midi_file_performer::Performer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Args {
	/// List all connected midi output devices
	#[arg(short, long, default_value_t = false)]
	list: bool,

	/// The index of the connected midi output device to use 
	#[arg(short, long, default_value_t = 0)]
	device: usize,

	/// Ignore any connected midi output devices and set up a virtual output instead
	#[arg(short, long, default_value_t = false)]
	use_virtual: bool
}

fn main() {
	let args = Args::parse();

	//get midi output port
		let midi_output = MidiOutput::new("play_midi").unwrap();
		let midi_output_connections = midi_output.ports();

		if args.list {
			if midi_output_connections.is_empty() {
				println!("there are no MIDI output devices detected");
			} else {
				midi_output_connections.iter().enumerate().for_each(|(index, midi_output_port)| {
					println!("{index} > {:?}", midi_output.port_name(midi_output_port));
				});
			}

			return;
		}

		let mut midi_output_connection = if args.use_virtual {
			midi_output.create_virtual("midi_file_performer").unwrap()
		} else {
			if midi_output_connections.is_empty() {
				println!("no MIDI output device available, will use virtual output instead");
				midi_output.create_virtual("midi_file_performer").unwrap()
			} else {
				let midi_output_port = &midi_output_connections[args.device];
				println!("using MIDI output device {} : {:?}", args.device, midi_output.port_name(midi_output_port));
				midi_output.connect(midi_output_port, "play_midi_connection").unwrap()
			}
		};
		
	//load midi file
		let standard_midi_file = midly::Smf::parse(include_bytes!("../tests/scarborough_fair.mid")).unwrap();
	
	//perform
		let mut performer = Performer::new(standard_midi_file);
		performer.jump_to(performer.get_length()/2).ok();
		performer.play().ok();

	loop {
		if !performer.is_playing() {
			break;
		}

		if let Some(events) = performer.poll().ok().unwrap() {
			events.iter().for_each(|(_track, midi_event)| {
				let mut buf = Vec::new();
				midi_event.into_live_event().write(&mut buf).unwrap();
				midi_output_connection.send(&buf[..]).unwrap();
			});
			std::thread::sleep(Duration::from_micros(1));
		} else {
			break;
		}
	}
}