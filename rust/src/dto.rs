use portmidi::{DeviceInfo, MidiEvent};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Key {
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
    pub fn from_u8(value: u8) -> Key {
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChannelMessage {
    pub events: Vec<MidiEvent>,
    pub device: DeviceInfo,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct Note {
    pub octave: u8,
    pub key: Key,
    pub pressed: bool,
    pub timestamp: u32,
}
