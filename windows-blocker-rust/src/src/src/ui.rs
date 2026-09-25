use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{MessageBoxW, IDYES, MB_ICONQUESTION, MB_YESNO};

// تحويل النصوص إلى صيغة UTF-16 المقبولة لدى Windows API
fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

// إظهار نافذة تفاعلية تسأل المستخدم
pub fn ask_permission(title: &str, message: &str) -> bool {
    let wide_title = to_wide_string(title);
    let wide_message = to_wide_string(message);

    unsafe {
        let result = MessageBoxW(
            std::ptr::null_mut(),
            wide_message.as_ptr(),
            wide_title.as_ptr(),
            MB_YESNO | MB_ICONQUESTION,
        );
        result == IDYES
    }
}