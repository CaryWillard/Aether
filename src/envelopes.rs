// Envelopes

use super::{Percent, Seconds, SampleRate};
use super::envelope_forms::EnvelopeForm;
use super::indexers::{DynamicPercentIndexer, UnpitchedIndexer, OptionDynamicPercentIndexer};

pub trait Envelope {
    fn get_percent(&mut self) -> Percent;
}

pub trait OptionEnvelope {
    fn get_percent_option(&mut self) -> Option<Percent>;
}

pub struct Env {
    envelope_form: Box<EnvelopeForm>,
    indexer: Box<UnpitchedIndexer>,
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

impl OptionEnvelope for Env {
    fn get_percent_option(&mut self) -> Option<Percent> {
        let perc = self.indexer
            .get_next_option_dynamically(self.duration_seconds, self.sample_rate);

        match perc {
            Some(p) => Some(self.envelope_form.get_percent(p)),
            None => None,
        }
    }
}


pub struct EnvSeq {
    seq: Vec<Env>,
    i: usize,
}

impl EnvSeq {
    pub fn new(seq: Vec<Env>) -> EnvSeq {
        EnvSeq { seq: seq, i: 0 }
    }
}

impl OptionEnvelope for EnvSeq {
    fn get_percent_option(&mut self) -> Option<Percent> {
        loop {
            if self.i >= self.seq.len() {
                return None;
            }

            let result = self.seq[self.i].get_percent_option();
            match result {
                Some(_) => return result,
                None => {
                    self.i += 1;
                    continue;
                }
            };
        }
    }
}
