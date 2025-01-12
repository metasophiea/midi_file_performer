A library for the asynchronous playback of Midi files.

This crate provides a struct that can be used to play a midi file, pause and restart it, adjust playback speed and jump to any point in the file. One can also learn of the length of the Midi file and the playhead position in ticks (`usize`) and time (`Duration`).

The asynchronous nature of this crate is achieved by using a dedicated thread, used to loop through the midi file at the appropriate speed. Messages are sent to and from the "console" struct to this "engine", in order to control the playback and garner midi events.

This crate works alongside the [midly](https://crates.io/crates/midir) crate.

# Examples
- `/examples/basic_playback.rs` - an example of basic playback
- `/examples/jump_to.rs` - an example jumping to different sections of the score
- `/examples/basic_playback.rs` - an example adjusting the playback speed

# Acknowledgements
This library is very based on earlier work by Taylan Gökkaya which can be found at [https://github.com/insomnimus/nodi](https://github.com/insomnimus/nodi])