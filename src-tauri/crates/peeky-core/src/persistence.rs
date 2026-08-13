use std::{
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::{Local, Utc};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

use crate::domain::{PersistedState, Settings};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Atomic file replacement failed: {0}")]
    Replace(String),
}

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub root: PathBuf,
    pub settings: PathBuf,
    pub settings_prev: PathBuf,
    pub state: PathBuf,
    pub state_prev: PathBuf,
    pub logs: PathBuf,
    pub activity_db: PathBuf,
    pub activity_exports: PathBuf,
}

impl AppPaths {
    pub fn discover() -> Result<Self, StorageError> {
        let local = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir()?);
        let root = local.join("Peeky");
        let logs = root.join("logs");
        let activity_exports = root.join("exports");
        fs::create_dir_all(&logs)?;
        fs::create_dir_all(&activity_exports)?;
        Ok(Self {
            settings: root.join("settings.json"),
            settings_prev: root.join("settings.prev.json"),
            state: root.join("state.json"),
            state_prev: root.join("state.prev.json"),
            root,
            logs,
            activity_db: local.join("Peeky").join("activity.db"),
            activity_exports,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    pub paths: AppPaths,
}

impl Storage {
    pub fn initialize() -> Result<(Self, Settings, PersistedState), StorageError> {
        let storage = Self {
            paths: AppPaths::discover()?,
        };

        let settings = storage.load_settings()?;
        let mut state = storage.load_state()?;
        state.normalize(&settings);
        storage.save_settings(&settings)?;
        storage.save_state(&state)?;
        storage.app_log("INFO", "Storage initialized")?;
        Ok((storage, settings, state))
    }

    pub fn load_settings(&self) -> Result<Settings, StorageError> {
        for (path, preserve) in [
            (&self.paths.settings, true),
            (&self.paths.settings_prev, false),
        ] {
            if !path.exists() {
                continue;
            }
            match read_json::<Settings>(path)
                .and_then(|settings| settings.migrate().map_err(StorageError::Replace))
            {
                Ok(settings) => return Ok(settings),
                Err(_) if preserve => self.preserve_corrupt(path, "settings-invalid")?,
                Err(_) => {}
            }
        }
        Ok(Settings::default())
    }

    pub fn load_state(&self) -> Result<PersistedState, StorageError> {
        Ok(self
            .load_with_fallback::<PersistedState>(
                &self.paths.state,
                &self.paths.state_prev,
                "state",
            )?
            .unwrap_or_default())
    }

    pub fn save_settings(&self, settings: &Settings) -> Result<(), StorageError> {
        settings.validate().map_err(StorageError::Replace)?;
        self.write_json_atomic(&self.paths.settings, &self.paths.settings_prev, settings)
    }

    pub fn save_state(&self, state: &PersistedState) -> Result<(), StorageError> {
        self.write_json_atomic(&self.paths.state, &self.paths.state_prev, state)
    }

    pub fn event(&self, event_type: &str, payload: Value) -> Result<(), StorageError> {
        let path = self
            .paths
            .logs
            .join(format!("events-{}.jsonl", Local::now().format("%Y-%m-%d")));
        let event = json!({
            "ts": Utc::now().to_rfc3339(),
            "type": event_type,
            "payload": payload,
        });
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        serde_json::to_writer(&mut file, &event)?;
        file.write_all(b"\n")?;
        file.flush()?;
        Ok(())
    }

    pub fn app_log(&self, level: &str, message: &str) -> Result<(), StorageError> {
        let path = self
            .paths
            .logs
            .join(format!("peeky-{}.log", Local::now().format("%Y-%m-%d")));
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(
            file,
            "{} [{level}] {message}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        )?;
        file.flush()?;
        Ok(())
    }

    pub fn diagnostics_text(&self, settings: &Settings, state: &PersistedState) -> String {
        let settings_json = serde_json::to_string_pretty(settings).unwrap_or_default();
        let state_json = serde_json::to_string_pretty(state).unwrap_or_default();
        format!(
            "Peeky diagnostics\nVersion: {}\nOS: {}\nData: {}\nGenerated: {}\n\nSettings:\n{}\n\nState:\n{}",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            self.paths.root.display(),
            Utc::now().to_rfc3339(),
            settings_json,
            state_json
        )
    }

    fn load_with_fallback<T: DeserializeOwned>(
        &self,
        current: &Path,
        previous: &Path,
        kind: &str,
    ) -> Result<Option<T>, StorageError> {
        if current.exists() {
            match read_json(current) {
                Ok(value) => return Ok(Some(value)),
                Err(_) => self.preserve_corrupt(current, kind)?,
            }
        }
        if previous.exists() {
            if let Ok(value) = read_json(previous) {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn preserve_corrupt(&self, path: &Path, kind: &str) -> Result<(), StorageError> {
        if path.exists() {
            let preserved = self.paths.root.join(format!(
                "{}.corrupt-{}.json",
                kind,
                Local::now().format("%Y%m%d-%H%M%S")
            ));
            fs::copy(path, preserved)?;
        }
        Ok(())
    }

    fn write_json_atomic<T: Serialize>(
        &self,
        target: &Path,
        previous: &Path,
        value: &T,
    ) -> Result<(), StorageError> {
        let bytes = serde_json::to_vec_pretty(value)?;
        let temp = target.with_extension("tmp.json");
        {
            let mut file = File::create(&temp)?;
            file.write_all(&bytes)?;
            file.write_all(b"\n")?;
            file.flush()?;
            file.sync_all()?;
        }

        if target.exists() {
            fs::copy(target, previous)?;
        }
        replace_file(&temp, target)?;
        let _: Value = read_json(target)?;
        Ok(())
    }
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError> {
    let mut contents = Vec::new();
    File::open(path)?.read_to_end(&mut contents)?;
    Ok(serde_json::from_slice(&contents)?)
}

#[cfg(windows)]
fn replace_file(temp: &Path, target: &Path) -> Result<(), StorageError> {
    use std::{iter::once, os::windows::ffi::OsStrExt, ptr};
    use windows_sys::Win32::Storage::FileSystem::{
        MoveFileExW, ReplaceFileW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
        REPLACEFILE_WRITE_THROUGH,
    };

    fn wide(value: &OsStr) -> Vec<u16> {
        value.encode_wide().chain(once(0)).collect()
    }

    let temp_wide = wide(temp.as_os_str());
    let target_wide = wide(target.as_os_str());
    let mut ok = unsafe {
        if target.exists() {
            ReplaceFileW(
                target_wide.as_ptr(),
                temp_wide.as_ptr(),
                ptr::null(),
                REPLACEFILE_WRITE_THROUGH,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        } else {
            MoveFileExW(
                temp_wide.as_ptr(),
                target_wide.as_ptr(),
                MOVEFILE_WRITE_THROUGH,
            )
        }
    };
    if ok == 0 && temp.exists() {
        ok = unsafe {
            MoveFileExW(
                temp_wide.as_ptr(),
                target_wide.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
    }
    if ok == 0 {
        return Err(StorageError::Replace(
            std::io::Error::last_os_error().to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(windows))]
fn replace_file(temp: &Path, target: &Path) -> Result<(), StorageError> {
    if target.exists() {
        fs::remove_file(target)?;
    }
    fs::rename(temp, target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip_as_json() {
        let settings = Settings::default();
        let encoded = serde_json::to_string(&settings).unwrap();
        let decoded: Settings = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, settings);
    }

    #[test]
    fn version_one_settings_migrate_to_the_five_second_warning() {
        let mut settings = Settings::default();
        settings.schema_version = 1;
        settings.experience.warning_secs = 10;

        let migrated = settings.migrate().unwrap();

        assert_eq!(
            migrated.schema_version,
            crate::domain::SETTINGS_SCHEMA_VERSION
        );
        assert_eq!(migrated.experience.warning_secs, 5);
    }

    #[test]
    fn version_one_custom_warning_is_preserved() {
        let mut settings = Settings::default();
        settings.schema_version = 1;
        settings.experience.warning_secs = 15;

        let migrated = settings.migrate().unwrap();

        assert_eq!(migrated.experience.warning_secs, 15);
    }

    #[test]
    fn version_two_settings_add_opt_in_activity_defaults() {
        let mut settings = Settings::default();
        settings.schema_version = 2;
        settings.activity.consented = true;
        settings.activity.enabled = true;

        let migrated = settings.migrate().unwrap();

        assert_eq!(
            migrated.schema_version,
            crate::domain::SETTINGS_SCHEMA_VERSION
        );
        assert!(!migrated.activity.consented);
        assert!(!migrated.activity.enabled);
        assert_eq!(migrated.activity.retention_days, 90);
    }
}
