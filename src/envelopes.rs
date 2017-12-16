// Envelopes

use super::Percent;

pub trait EnvelopeForm {
    fn get_percent(&self, progress: Percent) -> Percent;
}

pub struct Line {
    pub amplitude_offset: Percent,
    pub slope: f64,
}

impl Line {
    pub fn new(start: Percent, end: Percent) -> Line {
        Line {
            amplitude_offset: start,
            slope: end - start,
        }
    }
}

impl EnvelopeForm for Line {
    fn get_percent(&self, progress: Percent) -> Percent {
        progress * self.slope + self.amplitude_offset
    }
}
