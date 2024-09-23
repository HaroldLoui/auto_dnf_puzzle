use winput::{Button, Mouse};

pub struct MouseUtils;

impl MouseUtils {
    pub fn new() -> Self {
        MouseUtils
    }

    pub fn move_mouse(&self, pos: (u32, u32)) {
        Mouse::set_position(pos.0 as i32, pos.1 as i32).expect("failed to move mouse");
    }

    pub fn left_click(&self) {
        winput::press(Button::Left);
        winput::release(Button::Left);
        // let error = winput::WindowsError::from_last_error();
        // println!("{}", error);
    }

    #[allow(dead_code)]
    pub fn right_click(&self) {
        winput::press(Button::Right);
        winput::release(Button::Right);
    }
}


#[cfg(test)]
mod tests {
    use crate::focus_win::focus_window;
    use super::*;

    #[test]
    fn test1() {
        match focus_window("v2rayN - V6.55 - 2024/08/05 - 以非管理员身份运行") {
            Ok(_) => {
                let mouse_utils = MouseUtils::new();
                mouse_utils.move_mouse((700, 500));
                mouse_utils.left_click();
                mouse_utils.right_click();
            }
            Err(str) => {
                println!("{}", str);
            }
        }
    }
}

