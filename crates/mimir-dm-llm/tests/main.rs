// Integration tests for the mimir-dm-llm crate
// These tests require a running Ollama instance on localhost:11434

mod ollama;

// Test utilities and setup
pub mod test_utils {
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn setup() {
        INIT.call_once(|| {
            // Tests will handle their own async setup
            println!("Integration test setup initialized");
        });
    }
}