use win_screenshot::prelude::*;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

pub fn focus_dnf_window() -> Result<(), &'static str> {
    let window_name = "地下城与勇士：创新世纪";
    focus_window(window_name)
}

pub fn focus_window(window_name: &str) -> Result<(), &'static str> {
    let hwnd_name_option = window_list()
        .unwrap()
        .into_iter()
        .find(|item| item.window_name.eq(window_name));
    if let Some(hwnd_name) = &hwnd_name_option {
        unsafe {
            let _ = SetForegroundWindow(HWND(hwnd_name.hwnd));
        }
        Ok(())
    } else {
        Err("请确保游戏进程存在")
    }
}

#[allow(dead_code)]
pub fn print_window_names() {
    window_list()
        .unwrap()
        .iter()
        .for_each(|w| println!("{:?}", w.window_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_windows() {
        // let re = Regex::new(r"设置").unwrap();
        let str = "v2rayN";
        assert_eq!(Ok(()), focus_window(str));
    }

    #[test]
    fn windows_list_names() {
        print_window_names();
    }
}