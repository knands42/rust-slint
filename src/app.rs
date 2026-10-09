use crate::{
    backend::application::{sample_manager::SampleManager, state::SharedState},
    ui,
};

pub struct App {
    state: SharedState,
    sample_manager: SampleManager,
}

impl App {
    pub fn new() -> Self {
        let state = SharedState::new();
        let sample_manager = SampleManager::new(state.clone());
        Self {
            state,
            sample_manager,
        }
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        ui::run(self.state.clone())
    }
}
