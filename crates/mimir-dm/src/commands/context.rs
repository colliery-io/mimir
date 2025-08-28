use crate::services::context_service::ContextState;
use tauri::State;
use tracing::debug;

#[tauri::command]
pub async fn update_context(
    window_id: String,
    context_type: String,
    data: String,
    state: State<'_, ContextState>,
) -> Result<(), String> {
    debug!("Updating context for window {}: type={}", window_id, context_type);
    state.0.update_context(&context_type, &data)
}

#[tauri::command]
pub async fn get_full_context(
    state: State<'_, ContextState>,
) -> Result<String, String> {
    debug!("Getting full context");
    state.0.get_full_context()
}

#[tauri::command]
pub async fn register_window(
    window_id: String,
    window_type: String,
    title: String,
    state: State<'_, ContextState>,
) -> Result<(), String> {
    debug!("Registering window: {} ({})", window_id, window_type);
    state.0.register_window(&window_id, &window_type, &title)
}

#[tauri::command]
pub async fn unregister_window(
    window_id: String,
    state: State<'_, ContextState>,
) -> Result<(), String> {
    debug!("Unregistering window: {}", window_id);
    state.0.unregister_window(&window_id)
}

#[tauri::command]
pub async fn clear_shared_context(
    state: State<'_, ContextState>,
) -> Result<(), String> {
    debug!("Clearing shared context");
    state.0.clear_context()
}

#[tauri::command]
pub async fn get_context_for_llm(
    state: State<'_, ContextState>,
) -> Result<String, String> {
    debug!("Getting context for LLM");
    state.0.get_context_for_llm()
}

#[tauri::command]
pub async fn update_context_usage(
    usage: usize,
    state: State<'_, ContextState>,
) -> Result<(), String> {
    debug!("Updating context usage: {} tokens", usage);
    state.0.update_context_usage(usage)
}