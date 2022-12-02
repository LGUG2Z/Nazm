mod always_show_scrollbars;
mod apps_color_mode;
mod notifications;
mod show_accent_color_on_start_and_taskbar;
mod system_color_mode;
mod taskbar_items_chat;
mod taskbar_items_task_view;
mod taskbar_items_widgets;
mod text_size;
mod transparency_effects;

pub use always_show_scrollbars::AlwaysShowScrollbars;
pub use apps_color_mode::AppsColorMode;
pub use notifications::Notifications;
pub use show_accent_color_on_start_and_taskbar::ShowAccentColorOnStartAndTaskbar;
pub use system_color_mode::SystemColorMode;
pub use taskbar_items_chat::TaskbarItemsChat;
pub use taskbar_items_task_view::TaskbarItemsTaskView;
pub use taskbar_items_widgets::TaskbarItemsWidgets;
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
