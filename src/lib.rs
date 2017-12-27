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

use jack_main::run_client;

pub fn run() {
    run_client();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
