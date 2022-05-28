extern crate portmidi as pm;

use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use pm::{DeviceInfo, MidiEvent, PortMidi};
use std::net::TcpListener;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message};
use serde::Serialize;

const HOST: &str = "127.0.0.1:9001";
const MOCK_ENABLED: bool = false;

#[derive(Serialize, Debug)]
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

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Note {
    octave: u8,
    key: Key,
    pressed: bool,
    timestamp: u32
}

fn map_event_to_note(event: MidiEvent) -> Note {
    let octave: u8 = event.message.data1 / 12;
    let key = Key::from_u8(event.message.data1 % 12);
    let pressed = event.message.status == 144;
    let timestamp = event.timestamp;
    Note { octave, key, pressed, timestamp }
}

fn start_listener(context: PortMidi, tx_channel: mpsc::Sender<ChannelMessage>) {
    thread::Builder::new()
        .name("midi-listener".to_string())
        .spawn(move || {
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
        })
        .expect("Building thread failed");
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
    // Todo, might want to change to this producer & receiver: https://users.rust-lang.org/t/having-multiple-receivers-listening-to-the-same-sender-in-rust/61317/3
    // and move thread spawning inside incoming server request, that way we would spawn a new handling thread.
    // current problem is that only one instance of Receive can exist
    thread::Builder::new()
        .name("websocket-server".to_string())
        .spawn(move || {
            let server = TcpListener::bind(HOST).expect("Unable to bind tcp listener");
            for stream_result in server.incoming() {
                match stream_result {
                    Ok(stream) => {
                        let mut socket = accept(stream)
                            .map_err(must_not_block)
                            .expect("Couldn't expect websocket");
                        println!("Websocket client connected");
                        loop {
                            if MOCK_ENABLED {
                                socket.write_message(Message::Text(String::from("{\"octave\": 5, \"key\": \"C\"}")))
                                    .expect("Write message failed");
                                thread::sleep(Duration::from_millis(1000));
                            } else {
                                let channel_message = rx_arc.lock()
                                    .expect("Couldn't acquire lock")
                                    .recv()
                                    .expect("Couldn't receive channel message");

                                for midi_event in channel_message.events {
                                    let midi_note = map_event_to_note(midi_event);
                                    let serialized_note = serde_json::to_string(&midi_note)
                                        .expect("Unable to serialize note");
                                    let websocket_message = Message::Text(serialized_note);
                                    socket.write_message(websocket_message)
                                        .expect("Write message failed");
                                }
                                thread::sleep(Duration::from_millis(20));
                            }
                        }
                    }
                    Err(err) => println!("Stream error: {err:?}"),
                }
            }
        })
        .expect("Building thread failed");
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let (tx_channel, rx_channel) = mpsc::channel::<ChannelMessage>();

    // Create listener thread, listening to all connected midi devices
    start_listener(context, tx_channel);

    // Create websocket server thread, forwarding midi messages
    start_websocket_server(rx_channel);

    println!("-= Midi broker started =-");
    loop {
        thread::sleep(Duration::from_secs(2));
    }
}