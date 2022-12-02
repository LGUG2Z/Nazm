mod apps_color_mode;
mod notifications;
mod show_accent_color_on_start_and_taskbar;
mod system_color_mode;
mod text_size;
mod transparency_effects;

pub use apps_color_mode::AppsColorMode;
pub use notifications::Notifications;
pub use show_accent_color_on_start_and_taskbar::ShowAccentColorOnStartAndTaskBar;
pub use system_color_mode::SystemColorMode;
pub use text_size::TextSize;
pub use transparency_effects::TransparencyEffects;
use winreg::types::FromRegValue;
use winreg::types::ToRegValue;
use winreg::HKEY;

pub trait Setting: ToRegValue + FromRegValue + PartialEq + Eq + Copy + Clone {
    fn hkey() -> HKEY;
    fn subkey() -> &'static str;
    fn value() -> &'static str;
    fn logout_required() -> bool {
        false
    }
}
