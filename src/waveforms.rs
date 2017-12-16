// Waveforms

use std::f64;
use rand;
use rand::Rng;

use super::{Amplitude, Phase, Percent, Wavelength, SampleRate};

pub trait Waveform {
    fn get_amplitude(&self, phase: Phase) -> Amplitude;
    fn get_wavelength(&self) -> Wavelength;
}

pub struct Sine;

impl Waveform for Sine {
    fn get_amplitude(&self, phase: Phase) -> Amplitude {
        phase.sin()
    }

    fn get_wavelength(&self) -> Wavelength {
        f64::consts::PI * 2.0
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

    fn get_wavelength(&self) -> Wavelength {
        1.0
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

    fn get_wavelength(&self) -> Wavelength {
        1.0
    }
}

pub struct WhiteNoise {
    buffer: Vec<Amplitude>,
}

impl WhiteNoise {
    pub fn new(sample_rate: SampleRate) -> WhiteNoise {
        // Stores several seconds of noise.
        let mut buffer: Vec<f64> = Vec::with_capacity((sample_rate * 4) as usize);
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

    fn get_wavelength(&self) -> Wavelength {
        self.buffer.len() as Wavelength
    }
}
