// Indexers

use super::{Phase, Frequency, Wavelength, SampleRate};

pub trait Indexer {
    fn get_next(&mut self) -> Phase;
}

pub trait DynamicIndexer {
    fn get_next_dynamically(&mut self,
                            freq: Frequency,
                            wavelength: Wavelength,
                            sample_rate: SampleRate)
                            -> Phase;
}

pub struct PitchedIndexer {
    increment: Phase,
    index: Phase,
}

impl PitchedIndexer {
    pub fn new() -> PitchedIndexer {
        PitchedIndexer {
            increment: 0.0,
            index: 0.0,
        }
    }

    pub fn set_increment(&mut self,
                         freq: Frequency,
                         wavelength: Wavelength,
                         sample_rate: SampleRate) {
        self.increment = PitchedIndexer::get_increment(freq, wavelength, sample_rate);
    }

    fn get_increment(freq: Frequency, wavelength: Wavelength, sample_rate: SampleRate) -> Phase {
        (freq * wavelength) / (sample_rate as Frequency)
    }
}

impl Indexer for PitchedIndexer {
    fn get_next(&mut self) -> Phase {
        let result = self.index;
        self.index += self.increment;
        result
    }
}

impl DynamicIndexer for PitchedIndexer {
    fn get_next_dynamically(&mut self,
                            freq: Frequency,
                            wavelength: Wavelength,
                            sample_rate: SampleRate)
                            -> Phase {
        let result = self.index;
        self.increment = PitchedIndexer::get_increment(freq, wavelength, sample_rate);
        self.index += self.increment;
        result
    }
}
