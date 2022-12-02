#![allow(clippy::use_self)]

use serde::Deserialize;
use serde::Serialize;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::types::FromRegValue;
use winreg::types::ToRegValue;
use winreg::RegValue;
use winreg::HKEY;

use crate::registry::Setting;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum ShowAccentColorOnStartAndTaskBar {
    Disabled,
    Enabled,
}

impl FromRegValue for ShowAccentColorOnStartAndTaskBar {
    fn from_reg_value(reg_value: &RegValue) -> std::io::Result<Self> {
        let value = u32::from_reg_value(reg_value)?;
        Ok(match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            _ => unreachable!(),
        })
    }
}

impl ToRegValue for ShowAccentColorOnStartAndTaskBar {
    fn to_reg_value(&self) -> RegValue {
        match self {
            Self::Disabled => 0_u32.to_reg_value(),
            Self::Enabled => 1_u32.to_reg_value(),
        }
    }
}

impl Setting for ShowAccentColorOnStartAndTaskBar {
    fn hkey() -> HKEY {
        HKEY_CURRENT_USER
    }

    fn subkey() -> &'static str {
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
    }

    fn value() -> &'static str {
        r"ColorPrevalence"
    }
}
