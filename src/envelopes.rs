// Envelopes

use super::{Percent, Seconds, SampleRate};
use super::envelope_forms::EnvelopeForm;
use super::indexers::{DynamicPercentIndexer, UnpitchedIndexer};

pub trait Envelope {
    fn get_percent(&mut self) -> Percent;
}

pub struct Env {
    envelope_form: Box<EnvelopeForm>,
    indexer: Box<DynamicPercentIndexer>,
    duration_seconds: Seconds,
    sample_rate: SampleRate,
}

impl Env {
    pub fn new(duration_seconds: Seconds,
               sample_rate: SampleRate,
               envelope_form: Box<EnvelopeForm>)
               -> Env {
        Env {
            envelope_form: envelope_form,
            indexer: Box::new(UnpitchedIndexer::new()),
            duration_seconds: duration_seconds,
            sample_rate: sample_rate,
        }
    }
}

impl Envelope for Env {
    fn get_percent(&mut self) -> Percent {
        let p: Percent = self.indexer
            .get_next_dynamically(self.duration_seconds, self.sample_rate);

        self.envelope_form.get_percent(p)
    }
}
