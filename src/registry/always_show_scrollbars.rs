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
pub enum AlwaysShowScrollbars {
    Disabled,
    Enabled,
}

impl FromRegValue for AlwaysShowScrollbars {
    fn from_reg_value(reg_value: &RegValue) -> std::io::Result<Self> {
        let value = u32::from_reg_value(reg_value)?;
        Ok(match value {
            1 => Self::Disabled,
            0 => Self::Enabled,
            _ => unreachable!(),
        })
    }
}

impl ToRegValue for AlwaysShowScrollbars {
    fn to_reg_value(&self) -> RegValue {
        match self {
            Self::Disabled => 1_u32.to_reg_value(),
            Self::Enabled => 0_u32.to_reg_value(),
        }
    }
}

impl Setting for AlwaysShowScrollbars {
    fn hkey() -> HKEY {
        HKEY_CURRENT_USER
    }

    fn subkey() -> &'static str {
        r"Control Panel\Accessibility"
    }

    fn value() -> &'static str {
        r"DynamicScrollbars"
    }
}
