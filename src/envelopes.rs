// Envelopes

use super::{Percent, Wavelength, Phase, Seconds, SampleRate};

pub trait EnvelopeForm {
    fn get_percent(&self, phase: Phase) -> Percent;
    fn get_wavelength(&self) -> Wavelength;
}

pub struct Line {
    pub start: Percent,
    pub end: Percent,
    pub duration_samples: Wavelength,
}

impl Line {
    pub fn new(start: Percent,
               end: Percent,
               duration_seconds: Seconds,
               sample_rate: SampleRate)
               -> Line {
        Line {
            start: start.abs(),
            end: end.abs(),
            duration_samples: duration_seconds * (sample_rate as Wavelength),
        }
    }

    fn get_wavelength(&self) -> Wavelength {
        self.duration_samples
    }
}

impl EnvelopeForm for Line {
    fn get_percent(&self, phase: Phase) -> Percent {
        if phase > self.duration_samples || phase < 0.0 {
            // If out of range of the envelope, pass.
            return 1.0;
        }
        self.end / phase
    }

    fn get_wavelength(&self) -> Wavelength {
        self.duration_samples
    }
}
