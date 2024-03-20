use raylib::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum ConsoleEntryType {
    Log,
    Warning,
    Error,
    Debug,
}

impl ConsoleEntryType {
    const fn color(&self) -> Color {
        use ConsoleEntryType::*;
        match self {
            Log => Color::BLUE,
            Warning => Color::GOLD,
            Error => Color::RED,
            Debug => Color::MAGENTA,
        }
    }
}

struct ConsoleEntry {
    text: String,
    kind: ConsoleEntryType,
    /// Number of duplicates
    dups: usize,
    /// Whether `dups` changed since last draw
    dups_changed: bool,
}

impl ConsoleEntry {
    const FONT_SIZE: i32 = 8;
    const PADDING: i32 = 4;
    const HALF_PADDING: i32 = Self::PADDING / 2;
    const LINE_HEIGHT: i32 = Self::FONT_SIZE + Self::PADDING;
    const MAX_DUPLICATES: usize = 99; // after this we just show 99+
    const GUTTER_WIDTH: i32 = 24;

    fn new(s: &str, kind: ConsoleEntryType) -> Self {
        Self {
            text: s.to_string(),
            kind,
            dups: 0,
            dups_changed: true, // from null to 0
        }
    }

    fn draw_dupe_count(d: &mut RaylibDrawHandle, x: i32, y: i32, dups: usize, color: Color) {
        if dups > Self::MAX_DUPLICATES {
            assert_eq!(Self::MAX_DUPLICATES, 99, "Check the following line");
            d.draw_text("99+", x, y, Self::FONT_SIZE, color);
        } else {
            let dups_str = dups.to_string();
            d.draw_text(&dups_str, x, y, Self::FONT_SIZE, color);
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        let color: Color = self.kind.color();

        let x_body = x + Self::GUTTER_WIDTH;
        if self.dups_changed {
            d.draw_rectangle(
                x - Self::HALF_PADDING,
                y - Self::HALF_PADDING,
                Self::GUTTER_WIDTH,
                Self::LINE_HEIGHT,
                color.fade(0.25),
            );
            self.dups_changed = false;
        }

        d.draw_text(&self.text, x_body, y, Self::FONT_SIZE, color);

        if self.dups > 0 {
            Self::draw_dupe_count(d, x, y, self.dups, color);
        }
    }

    fn num_lines(&self) -> usize {
        self.text.lines().count()
    }

    fn is_duplicate(&self, prop_text: &str, prop_kind: &ConsoleEntryType) -> bool {
        (self.kind == *prop_kind) && (self.text == prop_text)
    }

    fn incr_dups(&mut self) {
        self.dups_changed = true;
        if self.dups <= Self::MAX_DUPLICATES {
            self.dups += 1;
        }
    }
}

pub struct Console {
    entries: Vec<ConsoleEntry>,
    start_entry: usize,
}

impl Console {
    /// Maximum lines visible in the console at a time (it can hold more)
    const MAX_LINES: usize = 10;

    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            start_entry: 0,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let visible_entries: Vec<&mut ConsoleEntry> = self
            .entries
            .iter_mut()
            .skip(self.start_entry)
            .take(Console::MAX_LINES)
            .collect();

        let x: i32 = 12;
        let mut y: i32 = 12;
        for entry in visible_entries {
            entry.draw(d, x, y);

            let height: i32 = (entry.num_lines() as i32) * ConsoleEntry::LINE_HEIGHT;
            y += height;
        }
    }

    pub fn scroll_up(&mut self) {
        self.start_entry = self.start_entry.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        let max_start = self.entries.len().saturating_sub(Console::MAX_LINES);
        self.start_entry = self.start_entry.saturating_add(1).min(max_start);
    }

    /// Add a log/warning/error/debug to the console.
    fn push_entry(&mut self, text: &str, kind: ConsoleEntryType) {
        if let Some(most_recent) = self.entries.last_mut() {
            if most_recent.is_duplicate(text, &kind) {
                most_recent.incr_dups();
                return;
            }
        }
        self.entries.push(ConsoleEntry::new(text, kind));
        self.scroll_down();
    }

    pub fn log(&mut self, text: &str) {
        self.push_entry(text, ConsoleEntryType::Log);
    }

    #[allow(dead_code)]
    pub fn warn(&mut self, text: &str) {
        self.push_entry(text, ConsoleEntryType::Warning);
    }

    #[allow(dead_code)]
    pub fn err(&mut self, text: &str) {
        self.push_entry(text, ConsoleEntryType::Error);
    }

    #[allow(dead_code)]
    pub fn debug(&mut self, text: &str) {
        self.push_entry(text, ConsoleEntryType::Debug);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_console() {
        let console = Console::new();
        assert_eq!(console.entries.len(), 0);
    }

    #[test]
    fn test_add_log() {
        let mut console = Console::new();

        console.log("Hello world");
        assert_eq!(console.entries.len(), 1);
    }

    #[test]
    fn test_add_duplicate_logs() {
        let mut console = Console::new();

        const TEST_STRING: &str = "Apple";

        console.log(TEST_STRING);
        assert_eq!(console.entries.len(), 1);
        assert_eq!(console.entries.last().unwrap().dups, 0);

        console.log(TEST_STRING);
        assert_eq!(console.entries.len(), 1);
        assert_eq!(console.entries.last().unwrap().dups, 1);

        console.log(TEST_STRING);
        assert_eq!(console.entries.len(), 1);
        assert_eq!(console.entries.last().unwrap().dups, 2);
    }

    #[test]
    fn test_add_unique_logs() {
        let mut console = Console::new();

        const TEST_STRING_A: &str = "Apple";
        const TEST_STRING_B: &str = "Banana";
        assert_ne!(TEST_STRING_A, TEST_STRING_B);

        console.log(TEST_STRING_A);
        assert_eq!(console.entries.len(), 1);
        assert_eq!(console.entries.last().unwrap().dups, 0);

        console.log(TEST_STRING_B);
        assert_eq!(console.entries.len(), 2);
        assert_eq!(console.entries.last().unwrap().dups, 0);
    }
}
