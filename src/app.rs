#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub item_counter: u8,
    pub cursor: u8,
}

impl App {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.item_counter.checked_add(1) {
            self.item_counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.item_counter.checked_sub(1) {
            self.item_counter = res;
        }
    }

    pub fn user_choose_next(&mut self) {
        if self.cursor.checked_add(1).is_some() && self.cursor + 1 < self.item_counter {
            self.cursor += 1;
        }
    }

    pub fn user_choose_previous(&mut self) {
        if self.cursor.checked_sub(1).is_some() && self.cursor >= u8::MIN {
            self.cursor -= 1;
        }
    }

}
