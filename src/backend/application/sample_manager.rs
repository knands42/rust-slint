use thiserror::Error;
use uuid::Uuid;

use crate::backend::{
    application::state::{SharedState, StateError},
    domain::sample::Sample,
};

#[derive(Error, Debug)]
pub enum SampleManagerError {
    #[error("state error: {0}")]
    State(#[from] StateError),
}

pub struct SampleManager {
    state: SharedState,
}

impl SampleManager {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    pub fn create_sample(&mut self) -> Result<Sample, SampleManagerError> {
        let sample = Sample::new(Uuid::new_v4());
        {
            let mut guard = self.state.write()?;
            guard.set_sample(sample.clone());
        }
        Ok(sample)
    }

    pub fn state(&self) -> &SharedState {
        &self.state
    }
}
