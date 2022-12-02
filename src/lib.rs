#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::redundant_pub_crate)]

mod cli;
mod nazm;
mod registry;

use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

pub use cli::Opts;
pub use cli::SubCommand;
pub use nazm::Setting as NazmSetting;
pub use registry::AppsColorMode;
pub use registry::Notifications;
pub use registry::ShowAccentColorOnStartAndTaskBar;
pub use registry::SystemColorMode;
pub use registry::TextSize;
pub use registry::TransparencyEffects;
use serde::Deserialize;
use serde::Serialize;

use crate::registry::AlwaysShowScrollbars;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub apps_color_mode: AppsColorMode,
    pub always_show_scrollbars: AlwaysShowScrollbars,
    pub notifications: Notifications,
    pub show_accent_color_on_start_and_task_bar: ShowAccentColorOnStartAndTaskBar,
    pub system_color_mode: SystemColorMode,
    pub text_size: TextSize,
    pub transparency_effects: TransparencyEffects,
}

impl Config {
    pub fn export() -> std::io::Result<Self> {
        Ok(Self {
            apps_color_mode: NazmSetting::get()?,
            always_show_scrollbars: NazmSetting::get()?,
            notifications: NazmSetting::get()?,
            show_accent_color_on_start_and_task_bar: NazmSetting::get()?,
            system_color_mode: NazmSetting::get()?,
            text_size: NazmSetting::get()?,
            transparency_effects: NazmSetting::get()?,
        })
    }

    pub fn apply(self) -> std::io::Result<Self> {
        NazmSetting::set(self.apps_color_mode)?;
        NazmSetting::set(self.always_show_scrollbars)?;
        NazmSetting::set(self.notifications)?;
        NazmSetting::set(self.show_accent_color_on_start_and_task_bar)?;
        NazmSetting::set(self.system_color_mode)?;
        NazmSetting::set(self.text_size)?;
        NazmSetting::set(self.transparency_effects)?;

        Self::export()
    }

    pub fn from_path(path: PathBuf) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            toml::to_string_pretty(self).expect("invalid struct")
        )
    }
}
