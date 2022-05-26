extern crate portmidi as pm;

use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use pm::{DeviceInfo, MidiEvent, PortMidi};

// Todo:
// - Read up on basic svelte setup, and adding websocket svelte
// - Read up on exposing rust websocket with midi data

#[derive(Debug)]
enum Key {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Key {
    fn from_u8(value: u8) -> Key {
        match value {
            0 => Key::C,
            1 => Key::Cs,
            2 => Key::D,
            3 => Key::Ds,
            4 => Key::E,
            5 => Key::F,
            6 => Key::Fs,
            7 => Key::G,
            8 => Key::Gs,
            9 => Key::A,
            10 => Key::As,
            11 => Key::B,
            _ => panic!("Unknown value, can't convert to key: {}", value),
        }
    }
}

const TIMEOUT: Duration = Duration::from_millis(10);
const BUF_LEN: usize = 1024;

#[derive(Debug)]
struct ChannelMessage {
    events: Vec<MidiEvent>,
    device: DeviceInfo,
}

#[derive(Debug)]
struct Note {
    octave: u8,
    key: Key,
}

fn map_event_to_note(event: MidiEvent) -> Note {
    let octave: u8 = event.message.data1 / 12;
    let key = Key::from_u8(event.message.data1 % 12);
    Note { octave, key }
}

fn start_listener(context: PortMidi, tx: mpsc::Sender<ChannelMessage>) {
    thread::spawn(move || {
        let in_ports = context
            .devices()
            .unwrap()
            .into_iter()
            .filter_map(|dev| context.input_port(dev, BUF_LEN).ok())
            .collect::<Vec<_>>();
        loop {
            for port in &in_ports {
                if let Ok(Some(events)) = port.read_n(BUF_LEN) {
                    tx.send(ChannelMessage { device: port.device(), events }).unwrap();
                }
            }
            thread::sleep(TIMEOUT);
        }
    });
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let (tx, rx) = mpsc::channel::<ChannelMessage>();

    // Create listener thread, listening to all connected midi devices
    start_listener(context, tx);

    loop {
        let message = rx.recv().unwrap();
        for event in message.events {
            // println!("[{}] {:?}", message.device, event);
            println!("{:?}", map_event_to_note(event));
        }
    }
}