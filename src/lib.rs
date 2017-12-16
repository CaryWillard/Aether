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

use waveforms::Waveform;
use indexers::Indexer;
use output::Output;
use oscillators::{Oscillator, Osc};

pub fn run() {
    // play_sine();
    // play_saw();
    // play_square();
    // play_white_noise();
    // play_mod_sine();
    play_osc();
}

fn play_sine() {
    let freq: Frequency = 440.0;
    let wavelength: Wavelength = f64::consts::PI * 2.0;
    let sample_rate: SampleRate = 44_100;

    let mut ind = indexers::PhaseIndexer::new();
    ind.set_increment(freq, wavelength, sample_rate);

    let wave = waveforms::Sine;

    let mut out = output::StdOutput::new();

    loop {
        out.send(wave.get_amplitude(ind.get_next()), 512.0);
    }
}

fn play_saw() {
    let freq: Frequency = 440.0;
    let wavelength: Wavelength = f64::consts::PI * 2.0;
    let sample_rate: SampleRate = 44_100;

    let mut ind = indexers::PhaseIndexer::new();
    ind.set_increment(freq, wavelength, sample_rate);

    let wave = waveforms::Saw::new(wavelength);

    let mut out = output::StdOutput::new();

    loop {
        out.send(wave.get_amplitude(ind.get_next()), 512.0);
    }
}

fn play_square() {
    let freq: Frequency = 440.0;
    let wavelength: Wavelength = 1.0;
    let sample_rate: SampleRate = 44_100;

    let mut ind = indexers::PhaseIndexer::new();
    ind.set_increment(freq, wavelength, sample_rate);

    let wave = waveforms::Square::new(wavelength, 0.5);

    let mut out = output::StdOutput::new();

    loop {
        let i = ind.get_next();
        let a = wave.get_amplitude(i);
        // eprintln!("i: {:?}, a: {:?}", i, a);
        out.send(a, 512.0);
    }
}

fn play_white_noise() {
    let freq: Frequency = 1.0;
    let sample_rate: SampleRate = 44_100;
    let num_samples: usize = sample_rate * 5;

    let mut ind = indexers::PhaseIndexer::new();
    ind.set_increment(freq, num_samples as Wavelength, sample_rate);

    let wave = waveforms::WhiteNoise::new(num_samples);

    let mut out = output::StdOutput::new();

    loop {
        out.send(wave.get_amplitude(ind.get_next()), 512.0);
    }
}

fn play_mod_sine() {
    let wavelength: Wavelength = f64::consts::PI * 2.0;
    let sample_rate: SampleRate = 44_100;

    let freq1: Frequency = 440.0;
    let freq2: Frequency = 110.0;

    let mut ind1 = indexers::PhaseIndexer::new();
    ind1.set_increment(freq1, wavelength, sample_rate);

    let mut ind2 = indexers::PhaseIndexer::new();
    ind2.set_increment(freq2, wavelength, sample_rate);

    let wave1 = waveforms::Sine;
    let wave2 = waveforms::Sine;

    let mut out = output::StdOutput::new();

    loop {
        let sample1 = wave1.get_amplitude(ind1.get_next());
        let sample2 = wave2.get_amplitude(ind2.get_next());
        out.send(sample1 * sample2, 512.0);
    }
}

fn play_osc() {
    let mut osc1 = Osc::new(440.0,
                            f64::consts::PI * 2.0,
                            44_100,
                            Box::new(waveforms::Sine),
                            Box::new(indexers::PhaseIndexer::new()));

    let mut out = output::StdOutput::new();

    loop {
        let sample1 = osc1.get_amplitude();
        out.send(sample1, 512.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
