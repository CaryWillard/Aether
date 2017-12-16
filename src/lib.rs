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
use indexers::PercentIndexer;
use envelopes::EnvelopeForm;

pub fn run() {
    play_osc();
}

fn play_osc() {
    let mut out = output::StdOutput::new();

    let mut osc1 = Osc::new(110.0, 44_100, Box::new(waveforms::Sine));
    let mut osc2 = Osc::new(110.0, 44_100, Box::new(waveforms::Square::new(0.7)));

    let env = envelopes::Line::new(1.0, 0.0);
    let mut indexer = indexers::UnpitchedIndexer::new();
    indexer.set_increment(10.0, 44_100);

    loop {
        let sample1 = osc1.get_amplitude();
        let sample2 = osc2.get_amplitude() * 0.5;
        let env_perc = env.get_percent(indexer.get_next());
        out.send((sample1 + sample2) * env_perc, 512.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
