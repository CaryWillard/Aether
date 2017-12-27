// Osscillators

use super::{Frequency, Wavelength, Amplitude, SampleRate, Phase};
use super::waveforms::Waveform;
use super::indexers::{DynamicIndexer, PitchedIndexer};

pub trait Oscillator {
    fn get_amplitude(&mut self) -> Amplitude;
    fn set_frequency(&mut self, freq: Frequency);
}

pub struct Osc {
    waveform: Box<Waveform + Send>,
    indexer: Box<DynamicIndexer + Send>,
    freq: Frequency,
    wavelength: Wavelength,
    sample_rate: SampleRate,
}

impl Osc {
    pub fn new(freq: Frequency, sample_rate: SampleRate, waveform: Box<Waveform + Send>) -> Osc {
        let wavelength: Wavelength = waveform.get_wavelength();
        Osc {
            waveform: waveform,
            indexer: Box::new(PitchedIndexer::new()),
            freq: freq,
            wavelength: wavelength,
            sample_rate: sample_rate,
        }
    }
}

impl Oscillator for Osc {
    fn get_amplitude(&mut self) -> Amplitude {
        let i: Phase = self.indexer
            .get_next_dynamically(self.freq, self.wavelength, self.sample_rate);
        self.waveform.get_amplitude(i)
    }

    fn set_frequency(&mut self, freq: Frequency) {
        self.freq = freq;
    }
}
