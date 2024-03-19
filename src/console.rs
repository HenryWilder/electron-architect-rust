pub struct Console {
    stream: String,
}

impl Console {
    pub fn new() -> Self {
        Self {
            stream: String::new(),
        }
    }

    pub fn get_lines<'idk>(&'idk self, offset: usize, lines: usize) -> &'idk str {
        let mut lines_iter = self.stream.match_indices('\n');
        let n = lines_iter.clone().count();

        if self.stream.ends_with('\n') {
            lines_iter.next_back(); // Don't count terminating newline
        }

        let start_iter: Option<(usize, &str)> = lines_iter.clone().nth_back(offset + lines);
        let start: usize = match start_iter {
            Some((index, _)) => index + 1,
            None => 0,
        };

        let end_iter: Option<(usize, &str)> = lines_iter.clone().nth_back(offset); // todo: Don't scroll beyond start
        let end: usize = match end_iter {
            Some((index, _)) => index + 1,
            None => self.stream.len(),
        };

        &self.stream[start..end]
    }

    pub fn write(&mut self, text: &str) {
        self.stream.push_str(text);
    }

    pub fn writeln(&mut self, text: &str) {
        self.stream.reserve(text.len() + 1);
        self.stream.push_str(text);
        self.stream.push('\n');
    }
}
