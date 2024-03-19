pub struct TextScroller {
    pub start_line: usize,
    pub end_line: usize,
    pub max_lines: usize,
}

impl TextScroller {
    pub fn new(max_lines: usize) -> Self {
        Self {
            start_line: 0,
            end_line: max_lines,
            max_lines,
        }
    }

    pub fn scroll_up(&mut self) {
        if self.start_line > 0 {
            self.start_line -= 1;
            self.end_line -= 1;
        }
    }

    pub fn scroll_down(&mut self, text: &String) {
        if self.end_line < text.lines().count().min(self.max_lines) {
            self.start_line += 1;
            self.end_line += 1;
        }
    }
}
