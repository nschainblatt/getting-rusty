#[allow(dead_code)]
#[derive(Debug)]
pub struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    pub fn build(width: u8, height: u8) -> Self {
        Rectangle { width, height }
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }
}
