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
    let _window = WebviewWindow::builder(
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
    let _window = WebviewWindow::builder(
        &app,
        "chat",
        WebviewUrl::App("chat.html".into()),
    )
    .title("Mimir Chat")
    .inner_size(800.0, 700.0)
    .min_inner_size(600.0, 500.0)
    .position(100.0, 100.0)
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create chat window: {}", e))?;
    
    info!("Chat window created successfully");
    Ok(())
}

#[tauri::command]
pub async fn open_log_viewer_window(app: AppHandle, file_name: String) -> Result<(), String> {
    info!("Opening log viewer window for file: {}", file_name);
    
    // Create unique window label for each log file (sanitize filename for window label)
    let sanitized_name = file_name.replace(".", "-").replace(" ", "_");
    let window_label = format!("log-viewer-{}", sanitized_name);
    
    // Check if window for this file already exists
    if let Some(window) = app.get_webview_window(&window_label) {
        // Focus existing window
        window.set_focus()
            .map_err(|e| format!("Failed to focus log viewer window: {}", e))?;
        return Ok(());
    }
    
    // Create new window with filename as query parameter (URL encode)
    let encoded_filename = file_name.replace(" ", "%20").replace(".", "%2E");
    let url = format!("log-viewer.html?file={}", encoded_filename);
    let _window = WebviewWindow::builder(
        &app,
        &window_label,
        WebviewUrl::App(url.into()),
    )
    .title(&format!("Log Viewer - {}", file_name))
    .inner_size(1000.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .position(150.0, 150.0)
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create log viewer window: {}", e))?;
    
    info!("Log viewer window created successfully for file: {}", file_name);
    Ok(())
}