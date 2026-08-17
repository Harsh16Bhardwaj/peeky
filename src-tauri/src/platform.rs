#[cfg(windows)]
pub fn system_idle_seconds() -> u64 {
    use std::mem::size_of;
    use windows_sys::Win32::{
        System::SystemInformation::GetTickCount64,
        UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
    };

    let mut info = LASTINPUTINFO {
        cbSize: size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    let success = unsafe { GetLastInputInfo(&mut info) };
    if success == 0 {
        return 0;
    }
    let ticks = unsafe { GetTickCount64() };
    let last = info.dwTime as u64;
    let current_low = ticks & 0xFFFF_FFFF;
    let elapsed_ms = current_low.wrapping_sub(last);
    elapsed_ms / 1_000
}

#[cfg(not(windows))]
pub fn system_idle_seconds() -> u64 {
    0
}

#[cfg(windows)]
pub fn cursor_position() -> Option<(i32, i32)> {
    use windows_sys::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};
    let mut point = POINT { x: 0, y: 0 };
    let success = unsafe { GetCursorPos(&mut point) };
    (success != 0).then_some((point.x, point.y))
}

#[cfg(not(windows))]
pub fn cursor_position() -> Option<(i32, i32)> {
    None
}

#[derive(Debug, Clone, Copy)]
pub struct WorkArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[cfg(windows)]
pub fn work_area_for_point(x: i32, y: i32) -> Option<WorkArea> {
    use std::mem::size_of;
    use windows_sys::Win32::{
        Foundation::POINT,
        Graphics::Gdi::{GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST},
    };

    let monitor = unsafe { MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONEAREST) };
    if monitor.is_null() {
        return None;
    }
    let mut info = MONITORINFO {
        cbSize: size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    let success = unsafe { GetMonitorInfoW(monitor, &mut info) };
    (success != 0).then_some(WorkArea {
        left: info.rcWork.left,
        top: info.rcWork.top,
        right: info.rcWork.right,
        bottom: info.rcWork.bottom,
    })
}

#[cfg(not(windows))]
pub fn work_area_for_point(_x: i32, _y: i32) -> Option<WorkArea> {
    None
}
#[derive(Debug, Clone)]
pub struct ForegroundApplication {
    pub executable: String,
    pub display_name: String,
}

/// A top-level window outside the Peeky process that can receive focus again
/// after a break overlay is dismissed. This is intentionally process-local and
/// is never persisted because HWND values are only valid for a running session.
#[derive(Debug, Clone, Copy)]
pub struct FocusRestoreTarget(usize);

#[cfg(windows)]
static FOREGROUND_WINDOW: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(windows)]
static LAST_EXTERNAL_WINDOW: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(windows)]
static LAST_APPLICATION: std::sync::OnceLock<
    std::sync::Mutex<Option<(u32, ForegroundApplication)>>,
> = std::sync::OnceLock::new();

#[cfg(windows)]
fn external_window_target(
    hwnd: windows_sys::Win32::Foundation::HWND,
) -> Option<FocusRestoreTarget> {
    use windows_sys::Win32::{
        System::Threading::GetCurrentProcessId,
        UI::WindowsAndMessaging::{GetWindowThreadProcessId, IsWindow},
    };

    if hwnd.is_null() || unsafe { IsWindow(hwnd) } == 0 {
        return None;
    }
    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut process_id) };
    (process_id != 0 && process_id != unsafe { GetCurrentProcessId() })
        .then_some(FocusRestoreTarget(hwnd as usize))
}

#[cfg(windows)]
fn remember_external_window(hwnd: windows_sys::Win32::Foundation::HWND) {
    if let Some(target) = external_window_target(hwnd) {
        LAST_EXTERNAL_WINDOW.store(target.0, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Preserve the current external foreground window before Peeky brings one of
/// its own windows forward.
#[cfg(windows)]
pub fn remember_current_external_window() {
    use windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    remember_external_window(unsafe { GetForegroundWindow() });
}

#[cfg(not(windows))]
pub fn remember_current_external_window() {}

/// Returns the most recently seen foreground window outside the Peeky process.
#[cfg(windows)]
pub fn last_external_foreground_window() -> Option<FocusRestoreTarget> {
    remember_current_external_window();
    let hwnd = LAST_EXTERNAL_WINDOW.load(std::sync::atomic::Ordering::Relaxed)
        as windows_sys::Win32::Foundation::HWND;
    external_window_target(hwnd)
}

#[cfg(not(windows))]
pub fn last_external_foreground_window() -> Option<FocusRestoreTarget> {
    None
}

/// Return focus only to a still-valid, visible window outside the Peeky process.
#[cfg(windows)]
pub fn restore_focus(target: FocusRestoreTarget) -> bool {
    use windows_sys::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            IsIconic, IsWindowVisible, SetForegroundWindow, ShowWindow, SW_RESTORE,
        },
    };

    let hwnd = target.0 as HWND;
    if external_window_target(hwnd).is_none() || unsafe { IsWindowVisible(hwnd) } == 0 {
        return false;
    }
    if unsafe { IsIconic(hwnd) } != 0 {
        unsafe { ShowWindow(hwnd, SW_RESTORE) };
    }
    (unsafe { SetForegroundWindow(hwnd) }) != 0
}

#[cfg(not(windows))]
pub fn restore_focus(_target: FocusRestoreTarget) -> bool {
    false
}

#[cfg(windows)]
pub fn start_foreground_monitor() {
    use windows_sys::Win32::{
        Foundation::HWND,
        UI::{
            Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK},
            WindowsAndMessaging::{
                GetMessageW, EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT,
                WINEVENT_SKIPOWNPROCESS,
            },
        },
    };

    unsafe extern "system" fn on_foreground(
        _hook: HWINEVENTHOOK,
        _event: u32,
        hwnd: HWND,
        _object: i32,
        _child: i32,
        _thread: u32,
        _time: u32,
    ) {
        FOREGROUND_WINDOW.store(hwnd as usize, std::sync::atomic::Ordering::Relaxed);
        remember_external_window(hwnd);
    }

    std::thread::spawn(|| unsafe {
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            std::ptr::null_mut(),
            Some(on_foreground),
            0,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
        );
        if hook.is_null() {
            return;
        }
        let mut message: MSG = std::mem::zeroed();
        while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) > 0 {}
        UnhookWinEvent(hook);
    });
}

#[cfg(not(windows))]
pub fn start_foreground_monitor() {}

#[cfg(windows)]
pub fn foreground_application() -> Option<ForegroundApplication> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::{
        Foundation::{CloseHandle, HWND},
        System::Threading::{
            OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::WindowsAndMessaging::{
            EnumChildWindows, GetForegroundWindow, GetWindowThreadProcessId,
        },
    };

    unsafe extern "system" fn find_uwp_child(hwnd: HWND, value: isize) -> i32 {
        let target = &mut *(value as *mut u32);
        let mut child_pid = 0;
        GetWindowThreadProcessId(hwnd, &mut child_pid);
        if child_pid != 0 && child_pid != *target {
            *target = child_pid;
            return 0;
        }
        1
    }

    let tracked = FOREGROUND_WINDOW.load(std::sync::atomic::Ordering::Relaxed);
    let current = unsafe { GetForegroundWindow() };
    let hwnd = if current.is_null() {
        tracked as HWND
    } else {
        current
    };
    if hwnd.is_null() {
        return None;
    }
    FOREGROUND_WINDOW.store(hwnd as usize, std::sync::atomic::Ordering::Relaxed);
    remember_external_window(hwnd);
    let mut pid = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if pid == 0 {
        return Some(restricted_application());
    }

    if let Some(application) = cached_application(pid) {
        return Some(application);
    }

    let mut path = process_image_path(pid);
    if path
        .as_ref()
        .and_then(|value| value.file_name())
        .is_some_and(|value| {
            value
                .to_string_lossy()
                .eq_ignore_ascii_case("ApplicationFrameHost.exe")
        })
    {
        let frame_pid = pid;
        unsafe { EnumChildWindows(hwnd, Some(find_uwp_child), &mut pid as *mut u32 as isize) };
        if pid != frame_pid {
            if let Some(application) = cached_application(pid) {
                return Some(application);
            }
            path = process_image_path(pid);
        }
    }
    let Some(path) = path else {
        return Some(restricted_application());
    };
    let executable = path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "restricted.exe".into());
    let display_name = product_name(&path).unwrap_or_else(|| friendly_process_name(&executable));
    let application = ForegroundApplication {
        executable,
        display_name,
    };
    if let Ok(mut cache) = LAST_APPLICATION.get_or_init(Default::default).lock() {
        *cache = Some((pid, application.clone()));
    }
    return Some(application);

    fn process_image_path(pid: u32) -> Option<std::path::PathBuf> {
        let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
        if process.is_null() {
            return None;
        }
        let mut buffer = vec![0u16; 32_768];
        let mut size = buffer.len() as u32;
        let ok = unsafe { QueryFullProcessImageNameW(process, 0, buffer.as_mut_ptr(), &mut size) };
        unsafe { CloseHandle(process) };
        if ok == 0 || size == 0 {
            return None;
        }
        Some(std::path::PathBuf::from(OsString::from_wide(
            &buffer[..size as usize],
        )))
    }
}

#[cfg(windows)]
fn cached_application(pid: u32) -> Option<ForegroundApplication> {
    LAST_APPLICATION
        .get_or_init(Default::default)
        .lock()
        .ok()
        .and_then(|cache| {
            cache
                .as_ref()
                .filter(|(cached_pid, _)| *cached_pid == pid)
                .map(|(_, application)| application.clone())
        })
}

#[cfg(not(windows))]
pub fn foreground_application() -> Option<ForegroundApplication> {
    None
}

#[cfg(windows)]
fn product_name(path: &std::path::Path) -> Option<String> {
    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    };

    fn wide(value: &OsStr) -> Vec<u16> {
        value.encode_wide().chain(once(0)).collect()
    }
    let filename = wide(path.as_os_str());
    let mut ignored = 0;
    let size = unsafe { GetFileVersionInfoSizeW(filename.as_ptr(), &mut ignored) };
    if size == 0 {
        return None;
    }
    let mut data = vec![0u8; size as usize];
    if unsafe { GetFileVersionInfoW(filename.as_ptr(), 0, size, data.as_mut_ptr().cast()) } == 0 {
        return None;
    }
    let translations = wide(OsStr::new("\\VarFileInfo\\Translation"));
    let mut translation_ptr = std::ptr::null_mut();
    let mut translation_len = 0;
    let mut language = 0x0409u16;
    let mut codepage = 0x04b0u16;
    if unsafe {
        VerQueryValueW(
            data.as_ptr().cast(),
            translations.as_ptr(),
            &mut translation_ptr,
            &mut translation_len,
        )
    } != 0
        && translation_len >= 4
    {
        let values = translation_ptr as *const u16;
        language = unsafe { *values };
        codepage = unsafe { *values.add(1) };
    }
    let query = format!("\\StringFileInfo\\{language:04x}{codepage:04x}\\ProductName");
    let query = wide(OsStr::new(&query));
    let mut value_ptr = std::ptr::null_mut();
    let mut value_len = 0;
    if unsafe {
        VerQueryValueW(
            data.as_ptr().cast(),
            query.as_ptr(),
            &mut value_ptr,
            &mut value_len,
        )
    } == 0
        || value_len <= 1
    {
        return None;
    }
    let value =
        unsafe { std::slice::from_raw_parts(value_ptr as *const u16, value_len as usize - 1) };
    let name = String::from_utf16_lossy(value).trim().to_string();
    (!name.is_empty()).then_some(name)
}

fn friendly_process_name(executable: &str) -> String {
    match executable.to_ascii_lowercase().as_str() {
        "chrome.exe" => "Google Chrome".into(),
        "msedge.exe" => "Microsoft Edge".into(),
        "firefox.exe" => "Mozilla Firefox".into(),
        "code.exe" => "Visual Studio Code".into(),
        "explorer.exe" => "File Explorer".into(),
        "peeky.exe" => "Peeky".into(),
        _ => executable
            .trim_end_matches(".exe")
            .split(['-', '_'])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut characters = part.chars();
                characters
                    .next()
                    .map(|first| first.to_uppercase().collect::<String>() + characters.as_str())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn restricted_application() -> ForegroundApplication {
    ForegroundApplication {
        executable: "restricted.exe".into(),
        display_name: "Restricted app".into(),
    }
}

#[cfg(windows)]
pub fn session_is_locked() -> bool {
    use windows_sys::Win32::System::StationsAndDesktops::{
        CloseDesktop, OpenInputDesktop, SwitchDesktop, DESKTOP_SWITCHDESKTOP,
    };
    let desktop = unsafe { OpenInputDesktop(0, 0, DESKTOP_SWITCHDESKTOP) };
    if desktop.is_null() {
        return true;
    }
    let available = unsafe { SwitchDesktop(desktop) } != 0;
    unsafe { CloseDesktop(desktop) };
    !available
}

#[cfg(not(windows))]
pub fn session_is_locked() -> bool {
    false
}
