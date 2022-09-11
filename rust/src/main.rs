use std::time::Duration;
use std::thread;
use portmidi::PortMidi;
use crate::midi_listener::start_midi_listener;
use crate::websocket::start_websocket_server;

mod websocket;
mod midi_listener;
mod dto;

fn main() {
    let context = PortMidi::new().unwrap();
    let (tx_channel, rx_channel) = crossbeam_channel::bounded(30);

    // Create listener thread, listening to all connected midi devices
    println!("Starting midi listener");
    start_midi_listener(context, tx_channel);

    // Create websocket server thread, forwarding midi messages
    println!("Starting websocket server");
    start_websocket_server(rx_channel);

    println!("-= Midi broker started =-");
    loop {
        thread::sleep(Duration::from_secs(2));
    }
}