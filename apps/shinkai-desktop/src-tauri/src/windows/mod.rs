use tauri::{AppHandle, Listener, Manager, WebviewWindowBuilder};

#[derive(Debug, Clone, Copy)]
pub enum Window {
    Main,
    ShinkaiNodeManager,
    Spotlight,
    Coordinator,
}

impl Window {
    pub fn as_str(&self) -> &'static str {
        match self {
            Window::Main => "main",
            Window::ShinkaiNodeManager => "shinkai-node-manager",
            Window::Spotlight => "spotlight",
            Window::Coordinator => "coordinator",
        }
    }
}

pub fn recreate_window(app_handle: AppHandle, window_name: Window, focus: bool) {
    let label = window_name.as_str();
    if let Some(window) = app_handle.get_webview_window(label) {
        log::info!("window {} found, bringing to front", label);
        if focus {
            window.show().unwrap();
            window.center().unwrap();
            let _ = window.set_focus();
        }
    } else {
        log::info!("window {} not found, recreating...", label);
        let window_config = app_handle
            .config()
            .app
            .windows
            .iter()
            .find(|w| w.label == label)
            .unwrap()
            .clone();
        match WebviewWindowBuilder::from_config(&app_handle, &window_config) {
            Ok(builder) => {
                if let Err(e) = builder.build() {
                    log::error!("failed to recreate window: {}", e);
                }
            }
            Err(e) => {
                log::error!("failed to recreate window from config: {}", e);
            }
        }
    }
}

pub fn hide_spotlight_window(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window(Window::Spotlight.as_str()) {
        window.hide().unwrap();
    }
}
