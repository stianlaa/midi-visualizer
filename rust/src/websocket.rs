use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::{TcpListener, TcpStream};
use crossbeam_channel::Receiver;
use portmidi::MidiEvent;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, WebSocket};
use crate::dto::{ChannelMessage, Key, Note};

const HOST: &str = "127.0.0.1:9001";

fn map_event_to_note(event: MidiEvent) -> Note {
    let octave: u8 = event.message.data1 / 12;
    let key = Key::from_u8(event.message.data1 % 12);
    let pressed = event.message.status == 144;
    let timestamp = event.timestamp;
    Note { octave, key, pressed, timestamp }
}

pub fn start_websocket_server(rx_channel: Receiver<ChannelMessage>) {
    let rx_arc = Arc::new(Mutex::new(rx_channel));
    thread::Builder::new()
        .name("websocket-server".to_string())
        .spawn(move || {

            loop {
                let server = TcpListener::bind(HOST).expect("Unable to bind tcp listener");
                for stream_result in server.incoming() {
                    match stream_result {
                        Ok(stream) => {
                            let mut socket = accept(stream)
                                .map_err(must_not_block)
                                .expect("Should have accepted websocket");

                            handle_connected_websocket(&mut socket, &rx_arc);
                        }
                        Err(err) => println!("Stream error: {err:?}"),
                    }
                }

                thread::sleep(Duration::from_millis(100));
            }
        })
        .expect("Building thread failed");
}

fn handle_connected_websocket(socket: &mut WebSocket<TcpStream>, rx_arc: &Arc<Mutex<Receiver<ChannelMessage>>>) {
    println!("Websocket client connected");
    loop {
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
                .expect("should be able to write message");
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => Error::AlreadyClosed,
        HandshakeError::Failure(f) => f,
    }
}