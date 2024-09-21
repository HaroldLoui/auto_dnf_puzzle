use enigo::{Enigo, Mouse, Settings};

pub struct EnigoUtils {
    enigo: Enigo,
}

impl EnigoUtils {
    pub fn new() -> Self {
        EnigoUtils { enigo: Enigo::new(&Settings::default()).unwrap() }
    }

    pub fn move_mouse(&mut self, pos: (u32, u32)) {
        self.enigo.move_mouse(pos.0 as i32, pos.1 as i32, enigo::Coordinate::Abs).unwrap();
    }

    pub fn left_click(&mut self) {
        self.enigo.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
    }
}

