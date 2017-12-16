extern crate rand;
extern crate byteorder;

pub mod waveforms;
pub mod indexers;
pub mod oscillators;
pub mod output;

type Amplitude = f64;
type Phase = f64;
// On range [0.0, 1.0]
type Percent = f64;
type Wavelength = f64;
type SampleRate = usize;
type Frequency = f64;

use std::f64;

use output::Output;
use oscillators::{Oscillator, Osc};

pub fn run() {
    play_osc();
}

fn play_osc() {
    let mut osc1 = Osc::new(220.0, 44_100, Box::new(waveforms::Sine));
    let mut osc2 = Osc::new(220.0, 44_100, Box::new(waveforms::Saw));

    let mut out = output::StdOutput::new();

    loop {
        let sample1 = osc1.get_amplitude();
        let sample2 = osc2.get_amplitude() * 0.5;
        out.send(sample1 + sample2, 512.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
