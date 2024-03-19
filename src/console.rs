use crate::text_scroller::TextScroller;

pub struct Console {
    stream: String,
    scroller: TextScroller,
}

impl Console {
    pub fn new() -> Self {
        let stream = String::new();
        Self {
            stream,
            scroller: TextScroller::new(10),
        }
    }

    pub fn read(&self) -> &str {
        let start = self
            .stream
            .lines()
            .enumerate()
            .nth(self.scroller.start_line);

        let end = self.stream.lines().enumerate().nth(self.scroller.end_line);

        let start_index: usize = match start {
            Some((i, _)) => i,
            None => 0,
        };

        let end_index: usize = match end {
            Some((i, _)) => i,
            None => self.scroller.max_lines,
        };

        &self.stream[start_index..end_index]
    }

    pub fn scroll_up(&mut self) {
        self.scroller.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroller.scroll_down(&self.stream);
    }

    #[allow(dead_code)]
    pub fn write(&mut self, text: &str) {
        self.stream.push_str(text);
    }

    pub fn writeln(&mut self, text: &str) {
        self.stream.reserve(text.len() + 1);
        self.stream.push_str(text);
        self.stream.push('\n');
        println!("console stream is now: {}", self.stream);
    }
}
