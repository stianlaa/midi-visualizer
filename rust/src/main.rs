extern crate portmidi as pm;

use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use pm::{DeviceInfo, MidiEvent, PortMidi};
use std::net::TcpListener;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message};

// Todo:
// - Read up on basic svelte setup, and adding websocket svelte
// - Read up on exposing rust websocket with midi data

const HOST: &str = "127.0.0.1:9001";

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
#[allow(dead_code)]
struct ChannelMessage {
    events: Vec<MidiEvent>,
    device: DeviceInfo,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Note {
    octave: u8,
    key: Key,
}

fn map_event_to_note(event: MidiEvent) -> Note {
    let octave: u8 = event.message.data1 / 12;
    let key = Key::from_u8(event.message.data1 % 12);
    Note { octave, key }
}

fn start_listener(context: PortMidi, tx_channel: mpsc::Sender<ChannelMessage>) {
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
                    tx_channel.send(ChannelMessage { device: port.device(), events }).unwrap();
                }
            }
            thread::sleep(TIMEOUT);
        }
    });
}

// TODO wrong mapping, already closed is incorrect
fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => Error::AlreadyClosed,
        HandshakeError::Failure(f) => f,
    }
}

fn start_websocket_server(rx_channel: mpsc::Receiver<ChannelMessage>) {
    let rx_arc = Arc::new(Mutex::new(rx_channel));
    thread::spawn(move || {
        let server = TcpListener::bind(HOST).unwrap();
        loop {
            for stream_result in server.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let mut socket = accept(stream).map_err(must_not_block).expect("Couldn't expect websocket");
                        println!("Waiting for client connection");
                        loop {
                            let channel_message = rx_arc.lock().expect("Couldn't receive lock").recv().unwrap();
                            for midi_event in channel_message.events {
                                let test_message = String::from(format!("Played note: {:?}", map_event_to_note(midi_event)));
                                socket.write_message(Message::Text(test_message)).expect("Write message failed");
                            }
                            thread::sleep(Duration::from_millis(50));
                        }
                    }
                    Err(err) => println!("Stream error: {err:?}"),
                }
            }
        }
    });
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let (tx_channel, rx_channel) = mpsc::channel::<ChannelMessage>();

    // Create listener thread, listening to all connected midi devices
    start_listener(context, tx_channel);

    // Create websocket server thread, forwarding midi messages
    start_websocket_server(rx_channel);

    loop {
        println!("Running!");
        thread::sleep(Duration::from_secs(2));
    }
}