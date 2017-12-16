// Indexers

use super::{Phase, Frequency, Wavelength, SampleRate, Percent, Seconds};

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

pub trait PercentIndexer {
    fn get_next(&mut self) -> Percent;
}

pub trait DynamicPercentIndexer {
    fn get_next_dynamically(&mut self,
                            duration_seconds: Seconds,
                            sample_rate: SampleRate)
                            -> Percent;
}

pub trait OptionalDynamicPercentIndexer {
    fn get_next_optional_dynamically(&mut self,
                                     duration_seconds: Seconds,
                                     sample_rate: SampleRate)
                                     -> Option<Percent>;
}

pub struct UnpitchedIndexer {
    increment: Percent,
    progress: Percent,
}

impl UnpitchedIndexer {
    pub fn new() -> UnpitchedIndexer {
        UnpitchedIndexer {
            increment: 0.0,
            progress: 0.0,
        }
    }

    pub fn set_increment(&mut self, duration_seconds: Seconds, sample_rate: SampleRate) {
        self.increment = UnpitchedIndexer::get_increment(duration_seconds, sample_rate)
    }

    fn get_increment(duration_seconds: Seconds, sample_rate: SampleRate) -> Percent {
        1.0 / ((sample_rate as Seconds) * duration_seconds)
    }
}

impl PercentIndexer for UnpitchedIndexer {
    fn get_next(&mut self) -> Percent {
        let result = self.progress;
        self.progress += self.increment;
        result
    }
}

impl DynamicPercentIndexer for UnpitchedIndexer {
    fn get_next_dynamically(&mut self,
                            duration_seconds: Seconds,
                            sample_rate: SampleRate)
                            -> Percent {
        let result = self.progress;
        self.increment = UnpitchedIndexer::get_increment(duration_seconds, sample_rate);
        self.progress += self.increment;
        result
    }
}

impl OptionalDynamicPercentIndexer for UnpitchedIndexer {
    fn get_next_optional_dynamically(&mut self,
                                     duration_seconds: Seconds,
                                     sample_rate: SampleRate)
                                     -> Option<Percent> {
        let result = self.progress;
        if result > 1.0 {
            return None;
        } else {
            self.increment = UnpitchedIndexer::get_increment(duration_seconds, sample_rate);
            self.progress += self.increment;
            Some(result)
        }
    }
}
