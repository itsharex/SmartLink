#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "linux")]
pub mod linux;

pub trait WindowInfo : Send + Sync{
    fn get_active_window(&self) -> Option<String>;
}

pub fn create_window_monitor() -> Box<dyn WindowInfo> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsMonitor::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOSMonitor::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxMonitor::new())
    }
}

pub trait AutoStart {
    fn set_auto_start(&self, enable: bool) -> Result<(), String>;
    fn is_auto_start_enabled(&self) -> Result<bool, String>;
}