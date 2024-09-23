use windows::Win32::UI::Input::KeyboardAndMouse::{INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};
use windows::Win32::UI::Input::KeyboardAndMouse::{mouse_event, SendInput, INPUT, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDeviceCaps, HDC, HORZRES, VERTRES, GetDC, ReleaseDC};

pub enum Button {
    Left,
    Right,
}

pub enum Coordinate {
    Absolute,
    Relative,
}

pub struct MouseInput;

impl MouseInput {
    pub fn new() -> Self {
        MouseInput
    }

    pub fn click(&self, button: Button) {
        let (up, down) = button_2_flags(button);
        send_mouse_event(down, 0, 0, 0);
        send_mouse_event(up, 0, 0, 0);
    }

    pub fn pressed(&self, button: Button) {
        let (_, down) = button_2_flags(button);
        send_mouse_event(down, 0, 0, 0);
    }

    pub fn released(&self, button: Button) {
        let (up, _) = button_2_flags(button);
        send_mouse_event(up, 0, 0, 0);
    }

    pub fn mouse_move(&self, x: i32, y: i32, coordinate: Coordinate) {
        let (new_x, new_y, flags) = match coordinate {
            Coordinate::Absolute => {
                let (screen_x, screen_y) = get_screen_resolution();
                // 将屏幕坐标 (x, y) 转换为绝对坐标
                let abs_x = (x * 65535) / screen_x; // 假设屏幕宽度为 1920
                let abs_y = (y * 65535) / screen_y; // 假设屏幕高度为 1080
                (abs_x, abs_y, MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE)
            }
            Coordinate::Relative => {
                (x, y, MOUSEEVENTF_MOVE)
            }
        };
        send_mouse_event(flags, new_x, new_y, 0);
    }
}

fn send_mouse_event(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, mouse_data: u32) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: mouse_data,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
        // mouse_event(flags, dx, dy, 0, 0);
    }
}

fn button_2_flags(button: Button) -> (MOUSE_EVENT_FLAGS, MOUSE_EVENT_FLAGS) {
    match button {
        Button::Left => (MOUSEEVENTF_LEFTUP, MOUSEEVENTF_LEFTDOWN),
        Button::Right => (MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_RIGHTDOWN)
    }
}

fn get_screen_resolution() -> (i32, i32) {
    unsafe {
        let hdc: HDC = GetDC(HWND(0));
        let width = GetDeviceCaps(hdc, HORZRES);
        let height = GetDeviceCaps(hdc, VERTRES);
        ReleaseDC(HWND(0), hdc);
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution() {
        println!("{:?}", get_screen_resolution());
    }
}
