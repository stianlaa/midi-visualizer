use std::thread;
use std::time::Duration;
use portmidi::{PortMidi};
use crate::dto::ChannelMessage;

const TIMEOUT: Duration = Duration::from_millis(10);
const BUF_LEN: usize = 1024;

pub fn start_midi_listener(context: PortMidi, tx_channel: crossbeam_channel::Sender<ChannelMessage>) {
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
