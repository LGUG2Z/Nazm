use winreg::enums::KEY_WRITE;
use winreg::RegKey;

use crate::registry::Setting as RegistrySetting;

pub struct Setting;
impl Setting {
    pub fn get<T: RegistrySetting>() -> std::io::Result<T> {
        RegKey::predef(T::hkey())
            .open_subkey(T::subkey())?
            .get_value(T::value())
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn set<T: RegistrySetting>(new_value: T) -> std::io::Result<()> {
        RegKey::predef(T::hkey())
            .open_subkey_with_flags(T::subkey(), KEY_WRITE)?
            .set_value(T::value(), &new_value)
    }
}
