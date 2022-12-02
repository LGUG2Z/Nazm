use serde::Deserialize;
use serde::Serialize;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::types::FromRegValue;
use winreg::types::ToRegValue;
use winreg::RegValue;
use winreg::HKEY;

use crate::registry::Setting;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct TextSize(pub u32);

impl FromRegValue for TextSize {
    fn from_reg_value(reg_value: &RegValue) -> std::io::Result<Self> {
        Ok(Self(u32::from_reg_value(reg_value)?))
    }
}

impl ToRegValue for TextSize {
    fn to_reg_value(&self) -> RegValue {
        self.0.to_reg_value()
    }
}

impl Setting for TextSize {
    fn hkey() -> HKEY {
        HKEY_CURRENT_USER
    }

    fn subkey() -> &'static str {
        r"sOFTWARE\Microsoft\Accessibility"
    }

    fn value() -> &'static str {
        r"TextScaleFactor"
    }

    fn logout_required() -> bool {
        true
    }
}
