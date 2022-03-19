pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!("-");
            }
            print!("\n");
        }
    }
}

