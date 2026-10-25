use std::{env, fs, path::PathBuf};

use iced::{Element, widget::{Button, Checkbox, Column, button}};
use serde::{Deserialize, Serialize};

use crate::AppMessage;

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ToggleShowHiddenFiles(bool),
    ToggleStoreLatest(bool),
    TryPushLatest(String),
    ClearLatest,
    FlushToFile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppSettings {
    show_hidden_files: bool,
    store_latest: bool,
    latest_files: Vec<String>,
    #[serde(skip_serializing)]
    dirty: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            show_hidden_files: true,
            store_latest: true,
            latest_files: Vec::new(),
            dirty: false,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        // failsafe check for two paths: $XDG_CONFIG_HOME, then $HOME/.config;
        // if neither does not exist, fall back to defaults
        // TODO non-linux check
        if let Ok(path) = env::var("XDG_CONFIG_HOME").map_or(env::home_dir().map_or(Err(()), |dir| Ok(dir.join(".config"))), |var| Ok(PathBuf::new().join(var))) {
            if let Ok(contents) = fs::read_to_string(path.join("nbtedit.toml")) {
                toml::from_str(contents.as_str()).unwrap_or_default()
            } else {
                // save settings to file before loading
                let settings: AppSettings = Default::default();
                settings.flush();
                settings
            }
        } else {
            Default::default()
        }
    }

    pub fn flush(&self) -> bool {
        // similar to above, but gives up on saving config
        // if for some reason home directory does not exist
        if let Ok(path) = env::var("XDG_CONFIG_HOME").map_or(env::home_dir().map_or(Err(()), |dir| Ok(dir.join(".config"))), |var| Ok(PathBuf::new().join(var))) {
            if let Ok(contents) = toml::to_string(self) {
                fs::write(path.join("nbtedit.toml"), contents).is_ok()
            } else {
                Default::default()
            }
        } else {
            false
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::ToggleShowHiddenFiles(value) => {
                self.show_hidden_files = value;
            },
            SettingsMessage::ToggleStoreLatest(value) => {
                self.store_latest = value;
            },
            SettingsMessage::TryPushLatest(value) => {
                self.latest_files.push(value);
            },
            SettingsMessage::ClearLatest => {
                self.latest_files.clear();
            },
            SettingsMessage::FlushToFile => {
                if self.dirty {
                    self.flush();
                    self.dirty = false;
                }
            }
        }

        self.dirty = true;
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        Column::new()
            .push(Checkbox::new(self.show_hidden_files)
                .label("Show hidden files")
                .on_toggle(|v| AppMessage::PassToSettings(SettingsMessage::ToggleShowHiddenFiles(v)))
            )
            .push(Checkbox::new(self.store_latest)
                .label("Store recently opened files")
                .on_toggle(|v| AppMessage::PassToSettings(SettingsMessage::ToggleStoreLatest(v)))
            )
            .push(Button::new("Clear recently opened files list")
                .on_press(AppMessage::PassToSettings(SettingsMessage::ClearLatest))
                .style(button::danger)
            )
            .spacing(4)
            .padding(4)
            // .align_x(Alignment::Center)
            .into()
    }
}