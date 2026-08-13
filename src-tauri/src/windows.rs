use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, WebviewUrl,
    WebviewWindowBuilder, Wry,
};

use crate::platform;

pub fn show_quick_panel(app: &AppHandle<Wry>) -> Result<(), String> {
    let window = app
        .get_webview_window("quick")
        .ok_or_else(|| "Quick panel window is unavailable".to_string())?;
    window.show().map_err(to_string)?;
    place_bottom_right(app, &window, 16)?;
    window.set_focus().map_err(to_string)?;
    Ok(())
}

pub fn toggle_quick_panel(app: &AppHandle<Wry>) -> Result<(), String> {
    let window = app
        .get_webview_window("quick")
        .ok_or_else(|| "Quick panel window is unavailable".to_string())?;
    if window.is_visible().map_err(to_string)? {
        window.hide().map_err(to_string)
    } else {
        show_quick_panel(app)
    }
}

pub fn show_settings(app: &AppHandle<Wry>) -> Result<(), String> {
    let window = app
        .get_webview_window("settings")
        .ok_or_else(|| "Settings window is unavailable".to_string())?;
    window.center().map_err(to_string)?;
    window.show().map_err(to_string)?;
    window.unminimize().map_err(to_string)?;
    window.set_focus().map_err(to_string)?;
    Ok(())
}

pub fn show_dashboard(app: &AppHandle<Wry>) -> Result<(), String> {
    let window = app
        .get_webview_window("dashboard")
        .ok_or_else(|| "Dashboard window is unavailable".to_string())?;
    window.show().map_err(to_string)?;
    window.unminimize().map_err(to_string)?;
    window.set_focus().map_err(to_string)?;
    Ok(())
}

pub fn show_warning(app: &AppHandle<Wry>) -> Result<(), String> {
    let window = match app.get_webview_window("warning") {
        Some(window) => window,
        None => WebviewWindowBuilder::new(
            app,
            "warning",
            WebviewUrl::App("index.html#/warning".into()),
        )
        .title("Peeky break heads-up")
        .inner_size(430.0, 204.0)
        .resizable(false)
        .decorations(false)
        .transparent(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .shadow(false)
        .visible(false)
        .build()
        .map_err(to_string)?,
    };
    place_bottom_right(app, &window, 16)?;
    window.show().map_err(to_string)?;
    Ok(())
}

pub fn close_warning(app: &AppHandle<Wry>) {
    if let Some(window) = app.get_webview_window("warning") {
        let _ = window.hide();
    }
}

pub fn show_overlays(app: &AppHandle<Wry>) -> Result<(), String> {
    close_overlays(app);
    let monitors = app.available_monitors().map_err(to_string)?;
    let cursor = platform::cursor_position();
    let primary_index = cursor
        .and_then(|(x, y)| {
            monitors.iter().position(|monitor| {
                let position = monitor.position();
                let size = monitor.size();
                x >= position.x
                    && y >= position.y
                    && x < position.x + size.width as i32
                    && y < position.y + size.height as i32
            })
        })
        .unwrap_or(0);

    for (index, monitor) in monitors.iter().enumerate() {
        let label = format!("overlay-{index}");
        let url = format!(
            "index.html#/overlay?primary={}",
            if index == primary_index { "1" } else { "0" }
        );
        let position = monitor.position();
        let size = monitor.size();
        let window = match app.get_webview_window(&label) {
            Some(window) => window,
            None => WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
                .title("Peeky break")
                .position(position.x as f64, position.y as f64)
                .inner_size(size.width as f64, size.height as f64)
                .resizable(false)
                .decorations(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .focusable(true)
                .focused(index == primary_index)
                .visible(false)
                .build()
                .map_err(to_string)?,
        };
        window.set_fullscreen(false).map_err(to_string)?;
        window
            .set_position(Position::Physical(PhysicalPosition::new(
                position.x, position.y,
            )))
            .map_err(to_string)?;
        window
            .set_size(Size::Physical(PhysicalSize::new(size.width, size.height)))
            .map_err(to_string)?;
        window.set_focusable(true).map_err(to_string)?;
        window
            .emit("overlay_role_changed", index == primary_index)
            .map_err(to_string)?;
        window.set_fullscreen(true).map_err(to_string)?;
        window.show().map_err(to_string)?;
        if index == primary_index {
            window.set_focus().map_err(to_string)?;
        }
    }
    Ok(())
}

pub fn close_overlays(app: &AppHandle<Wry>) {
    let labels = app
        .webview_windows()
        .keys()
        .filter(|label| label.starts_with("overlay-"))
        .cloned()
        .collect::<Vec<_>>();
    for label in labels {
        if let Some(window) = app.get_webview_window(&label) {
            let _ = window.hide();
            let _ = window.set_focusable(false);
        }
    }
}

pub fn open_logs_folder(path: &std::path::Path) -> Result<(), String> {
    std::process::Command::new("explorer.exe")
        .arg(path)
        .spawn()
        .map_err(to_string)?;
    Ok(())
}

fn place_bottom_right(
    app: &AppHandle<Wry>,
    window: &tauri::WebviewWindow<Wry>,
    margin: i32,
) -> Result<(), String> {
    let Some(monitor) = app.primary_monitor().map_err(to_string)? else {
        return Ok(());
    };
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    let center_x = monitor_position.x + monitor_size.width as i32 / 2;
    let center_y = monitor_position.y + monitor_size.height as i32 / 2;
    let fallback = platform::WorkArea {
        left: monitor_position.x,
        top: monitor_position.y,
        right: monitor_position.x + monitor_size.width as i32,
        bottom: monitor_position.y + monitor_size.height as i32,
    };
    let work = platform::work_area_for_point(center_x, center_y).unwrap_or(fallback);
    let outer = window.outer_size().map_err(to_string)?;
    let x = (work.right - outer.width as i32 - margin).max(work.left + margin);
    let bottom_clearance = margin + 64;
    let y = (work.bottom - outer.height as i32 - bottom_clearance).max(work.top + margin);
    window
        .set_position(Position::Physical(PhysicalPosition::new(x, y)))
        .map_err(to_string)
}

fn to_string(error: impl std::fmt::Display) -> String {
    error.to_string()
}
