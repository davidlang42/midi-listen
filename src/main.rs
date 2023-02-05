use std::io::Read;
use std::{env, fs};
use wmidi::{MidiMessage, FromBytesError};

fn main() {
    let mut args = env::args().skip(1);
    let midi_in: String = args.next().expect("The first argument should be the MIDI input device.");
    let mut f = fs::File::options().write(true).open(&midi_in).expect(&format!("Cannot open MIDI IN '{}'", midi_in));
    let mut buf: [u8; 1] = [0; 1];
    let mut bytes = Vec::new();
    while f.read_exact(&mut buf).is_ok() {
        bytes.push(buf[0]);
        match MidiMessage::try_from(bytes.as_slice()) {
            Ok(message) => {
                // message complete
                println!("[{}]: {:?}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"), message);
                bytes.clear();
            },
            Err(FromBytesError::NoBytes) | Err(FromBytesError::NoSysExEndByte) | Err(FromBytesError::NotEnoughBytes) => {
                // wait for more bytes
            }, 
            _ => {
                // invalid message, clear and wait for next message
                bytes.clear();
            }
        }
    }
}
