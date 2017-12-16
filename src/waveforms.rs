// Waveforms

use rand;
use rand::Rng;

use super::{Amplitude, Phase, Percent, Wavelength};

pub trait Waveform {
    fn get_amplitude(&self, phase: Phase) -> Amplitude;
}

pub struct Sine;

impl Waveform for Sine {
    fn get_amplitude(&self, phase: Phase) -> Amplitude {
        phase.sin()
    }
}

pub struct Saw {
    wavelength: Wavelength,
}

impl Saw {
    pub fn new(wavelength: Wavelength) -> Saw {
        Saw { wavelength: wavelength }
    }
}

impl Waveform for Saw {
    fn get_amplitude(&self, phase: Phase) -> Amplitude {
        ((phase % self.wavelength) / self.wavelength) * 2.0 - 1.0
    }
}

pub struct Square {
    wavelength: Wavelength,
    pulse_width: Percent,
}

impl Square {
    pub fn new(wavelength: Wavelength, pulse_width: Percent) -> Square {
        Square {
            wavelength: wavelength,
            pulse_width: pulse_width,
        }
    }
}

impl Waveform for Square {
    fn get_amplitude(&self, phase: Phase) -> Amplitude {
        if (phase % self.wavelength) / self.wavelength < self.pulse_width {
            return -1.0;
        } else {
            return 1.0;
        }
    }
}

pub struct WhiteNoise {
    buffer: Vec<Amplitude>,
}

impl WhiteNoise {
    pub fn new(num_samples: usize) -> WhiteNoise {
        // Store a buffer of three seconds of noise.
        let mut buffer: Vec<f64> = Vec::with_capacity(num_samples);
        let mut rng = rand::thread_rng();
        for _ in 0..buffer.capacity() {
            buffer.push(rng.gen::<f64>());
        }
        WhiteNoise { buffer: buffer }
    }
}

impl Waveform for WhiteNoise {
    fn get_amplitude(&self, index: Phase) -> Amplitude {
        self.buffer[(index as usize) % self.buffer.len()]
    }
}
