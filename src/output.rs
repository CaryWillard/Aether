// Output

use std::io;
use byteorder::{WriteBytesExt, LittleEndian};

use super::Amplitude;

pub trait Output {
    fn send(&mut self, sample: Amplitude, volume: Amplitude);
}

pub struct StdOutput {
    out: io::Stdout,
}

impl StdOutput {
    pub fn new() -> StdOutput {
        StdOutput { out: io::stdout() }
    }
}

impl Output for StdOutput {
    fn send(&mut self, sample: Amplitude, volume: Amplitude) {
        let sample = ((sample * volume) + (volume / 2.0)) as i16;
        self.out.write_i16::<LittleEndian>(sample).unwrap();
    }
}
