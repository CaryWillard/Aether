extern crate rand;
extern crate byteorder;

pub mod waveforms;
pub mod indexers;
pub mod oscillators;
pub mod envelopes;
pub mod output;

// Seconds
type Seconds = f64;
// Speaker position relative to 0.
type Amplitude = f64;
// Number of samples offset from the beginning of a wave.
type Phase = f64;
// On range [0.0, 1.0]
type Percent = f64;
// Number of samples in a wave.
type Wavelength = f64;
// Samples / second
type SampleRate = usize;
// Cycles / second
type Frequency = f64;

use std::f64;

use output::Output;
use oscillators::{Oscillator, Osc};

pub fn run() {
    play_osc();
}

fn play_osc() {
    let mut osc1 = Osc::new(0.3, 44_100, Box::new(waveforms::Sine));
    let mut osc2 = Osc::new(220.0, 44_100, Box::new(waveforms::Square::new(0.7)));

    let mut out = output::StdOutput::new();

    loop {
        let sample1 = osc1.get_amplitude();
        let sample2 = osc2.get_amplitude() * 0.5;
        out.send(sample1 * sample2, 512.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
