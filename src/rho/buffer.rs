use std::default::Default;

#[derive(Default)]
pub struct Cursor {
    x: u32,
    y: u32,
}

#[derive(Default)]
pub struct Buffer {
    x: u32,
    y: u32,
    contents: Vec<String>,
}


impl Buffer {
    pub fn new() -> Buffer {Buffer{x: 0, y: 0, contents: vec![String::new()]}}

    pub fn insert(&mut self, character: char) {
        self.contents[self.y as usize].insert(self.x as usize, character);
        self.move_right();
    }

    pub fn back_space(&mut self) {
        self.contents[self.y as usize].remove(self.x as usize);
        self.move_left();
    }

    pub fn get_x(&self) -> i32 {self.x as i32}

    pub fn get_y(&self) -> i32 {self.y as i32}

    pub fn get_contents(&self) -> String {self.contents.connect("\n")}

    pub fn insert_line(&mut self) {
        self.contents.insert(self.y as usize, String::new());
        self.move_down();
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let length = self.contents.len().to_string();
        self.contents[0].push_str(&length);
        if self.y + 1 < self.contents.len() as u32 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {self.x -= 1}
    }

    pub fn move_right(&mut self) {
        if self.x + 1 < self.contents[self.y as usize].len() as u32 {
            self.x += 1;
        }
    }
}
