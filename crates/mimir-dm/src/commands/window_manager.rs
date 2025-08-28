use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow};
use tracing::info;

#[tauri::command]
pub async fn open_context_debug_window(app: AppHandle) -> Result<(), String> {
    info!("Opening context debug window");
    
    // Check if window already exists
    if let Some(window) = app.get_webview_window("context-debug") {
        // Focus existing window
        window.set_focus()
            .map_err(|e| format!("Failed to focus context debug window: {}", e))?;
        return Ok(());
    }
    
    // Create new window
    let window = WebviewWindow::builder(
        &app,
        "context-debug",
        WebviewUrl::App("context-debug.html".into()),
    )
    .title("Context Debug - Mimir")
    .inner_size(800.0, 600.0)
    .min_inner_size(600.0, 400.0)
    .position(100.0, 100.0)
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create context debug window: {}", e))?;
    
    info!("Context debug window created successfully");
    Ok(())
}

#[tauri::command]
pub async fn open_chat_window(app: AppHandle) -> Result<(), String> {
    info!("Opening chat window");
    
    // Check if window already exists
    if let Some(window) = app.get_webview_window("chat") {
        // Focus existing window
        window.set_focus()
            .map_err(|e| format!("Failed to focus chat window: {}", e))?;
        return Ok(());
    }
    
    // Create new window
    let window = WebviewWindow::builder(
        &app,
        "chat",
        WebviewUrl::App("chat.html".into()),
    )
    .title("Mimir Chat")
    .inner_size(400.0, 600.0)
    .min_inner_size(350.0, 500.0)
    .position(100.0, 100.0)
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create chat window: {}", e))?;
    
    info!("Chat window created successfully");
    Ok(())
}