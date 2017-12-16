// Osscillators

use super::{Frequency, Wavelength, Amplitude, SampleRate, Phase};
use super::waveforms::Waveform;
use super::indexers::DynamicIndexer;

pub trait Oscillator {
    fn get_amplitude(&mut self) -> Amplitude;
}

pub struct Osc {
    waveform: Box<Waveform>,
    indexer: Box<DynamicIndexer>,
    freq: Frequency,
    wavelength: Wavelength,
    sample_rate: SampleRate,
}

impl Osc {
    pub fn new(freq: Frequency,
               wavelength: Wavelength,
               sample_rate: SampleRate,
               waveform: Box<Waveform>,
               indexer: Box<DynamicIndexer>)
               -> Osc {

        Osc {
            waveform: waveform,
            indexer: indexer,
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
}
