extern crate rand;
extern crate byteorder;
extern crate jack;

pub mod waveforms;
pub mod indexers;
pub mod oscillators;
pub mod envelope_forms;
pub mod envelopes;
pub mod output;
pub mod jack_main;

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
use envelopes::OptionEnvelope;
use jack_main::run_client;

pub fn run() {
    run_client();
    // play_osc();
}

fn play_osc() {
    let mut out = output::StdOutput::new();

    let mut osc2 = Osc::new(110.0, 44_100, Box::new(waveforms::Square::new(0.7)));

    let env1 = envelopes::Env::new(0.5, 44_100, Box::new(envelope_forms::Line::new(0.0, 1.0)));
    let env2 = envelopes::Env::new(0.5, 44_100, Box::new(envelope_forms::Line::new(1.0, 0.75)));
    let env3 = envelopes::Env::new(0.7, 44_100, Box::new(envelope_forms::Line::new(0.75, 0.75)));
    let env4 = envelopes::Env::new(0.5, 44_100, Box::new(envelope_forms::Line::new(0.75, 0.0)));
    let mut env_seq = envelopes::EnvSeq::new(vec![env1, env2, env3, env4]);

    loop {
        let sample2 = osc2.get_amplitude();
        let env_op = env_seq.get_percent_option();
        match env_op {
            Some(p) => out.send(sample2 * p, 1024.0),
            None => break,
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
