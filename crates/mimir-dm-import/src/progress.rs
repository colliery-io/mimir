//! Progress reporting for import operations

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Progress reporter for import operations
#[derive(Debug)]
pub struct ImportProgress {
    pb: ProgressBar,
    current_step: String,
}

impl ImportProgress {
    /// Create a new progress reporter with the given total entity count
    pub fn new(total_entities: usize) -> Self {
        let pb = ProgressBar::new(total_entities as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(120));

        Self {
            pb,
            current_step: String::new(),
        }
    }

    /// Update the current step being performed
    pub fn set_message<S: Into<String>>(&mut self, message: S) {
        self.current_step = message.into();
        self.pb.set_message(self.current_step.clone());
    }

    /// Increment progress by the given amount
    pub fn inc(&mut self, delta: u64) {
        self.pb.inc(delta);
    }

    /// Set the current position
    pub fn set_position(&mut self, pos: u64) {
        self.pb.set_position(pos);
    }

    /// Mark the import as completed successfully
    pub fn finish_with_message<S: Into<String>>(&mut self, message: S) {
        self.pb.finish_with_message(message.into());
    }

    /// Mark the import as failed
    pub fn abandon_with_message<S: Into<String>>(&mut self, message: S) {
        self.pb.abandon_with_message(message.into());
    }

    /// Create a simple spinner for validation operations
    pub fn spinner(message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Invalid spinner template"),
        );
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message(message.to_string());
        pb
    }
}

impl Drop for ImportProgress {
    fn drop(&mut self) {
        if !self.pb.is_finished() {
            self.pb.abandon();
        }
    }
}