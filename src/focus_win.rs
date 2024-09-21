use win_screenshot::prelude::*;
use regex::Regex;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

pub fn focus_dnf_window() -> Result<(), &'static str> {
    let window_name = "地下城与勇士";
    focus_window(window_name)
}

pub fn focus_window(window_name: &str) -> Result<(), &'static str> {
    let re = Regex::new(window_name).unwrap();
    let vec: Vec<HwndName> = window_list()
        .unwrap()
        .into_iter()
        .filter(|item| re.is_match(&item.window_name))
        .into_iter()
        .collect();
    if vec.len() != 1 {
        return Err("请确保只有一个游戏进程");
    }
    let hwnd = vec[0].hwnd;
    unsafe {
        let _ = SetForegroundWindow(HWND(hwnd));
    }
    Ok(())
}

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
}