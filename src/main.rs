use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::IndexedRandom;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize, Color, Modifier},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Paragraph, Block, Widget},
    DefaultTerminal, Frame,
};
use std::{io, time::{Instant, Duration}};
use unicode_width::UnicodeWidthStr;
const ROUND_TIME_SECS: u64 = 30;

#[derive(Debug, Default)]
pub struct App {
    pub word_count: usize,
    pub char_index: usize,
    pub word_index: usize,
    pub typed: String,
    pub target_words: Vec<String>,
    pub start_time: Option<Instant>,
    pub time_remaining: u64,
    pub exit: bool,
}
impl App {
    pub fn new() -> Self {
        let words_list = vec!["hello", "world", "type", "rust", "juice", "the", "lazy", "dog", "jumped", "over", "sleeping", "fox", "disgrace", "snap", "crop", "pot", "sound", "amber", "code", "intelligence", "chicken", "soup", "tower", "dough", "normal", "speed", "better", "minute", "best", "ever", "to", "and", "when", "by", "learn", "code", "gain", "buffer", "money", "start", "stop", "write", "food", "gym", "vector", "monkey", "through", "threw", "undo"];
        let mut rng = rand::rng();
        let target_words = (0..50)
            .map(|_| words_list.choose(&mut rng).unwrap().to_string()).collect();

        Self {
            word_count: 0,
            char_index: 0,
            word_index: 0,
            typed: String::new(),
            target_words,
            start_time: None,
            time_remaining: ROUND_TIME_SECS,
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        };
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            let wpm = (self.word_index as f64 / elapsed) * 60.0;
            println!("\nWPM: {:.2}", wpm);
        }
        Ok(())
    }
    fn draw(&self, f: &mut Frame){
        f.render_widget(self, f.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if self.start_time.is_none() && event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.start_time = Some(Instant::now());
                    self.handle_key_event(key_event);
                }
            }
        } else {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().as_secs();
                if elapsed >= ROUND_TIME_SECS {
                    self.exit = true;
                    return Ok(());
                } else {
                    self.time_remaining = ROUND_TIME_SECS - elapsed;
                }
            }
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event);
                    }
                }
            }
        }
       Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        use crossterm::event::KeyModifiers;

        if self.start_time.is_none(){
            self.start_time = Some(Instant::now());
        }

        match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit = true;
                    }
                    KeyCode::Char(c) => {
                        self.typed.push(c);
                        self.char_index += 1;
                        if let Some(word) = self.target_words.get(self.word_index) {
                            if self.char_index >= word.len() {
                            // validate incorrect length logic
                            }
                        } else {
                            self.exit = true;
                        }
                        
                    },
                    KeyCode::Backspace => {
                        if self.char_index > 0 {
                            self.char_index -= 1;
                            self.typed.pop();
                        }
                    },
                    KeyCode::Tab | KeyCode::Enter | KeyCode::Char(' ') => {
                        self.word_index += 1;
                        self.char_index = 0;

                        if self.word_index >= self.target_words.len() {
                            self.exit = true;
                        }
                    },
                    _ => {}
        }
    }
    fn typed_index(&self, word_idx: usize, char_idx: usize) -> usize {
        let mut index = 0;
        for i in 0..word_idx {
            index += self.target_words[i].len() + 1; // accounts for space
        }
        index + char_idx
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" TerminalType: Type To Begin ").bold();
        let instructions = Line::from(vec![
            Span::raw(" Time Remaining:"),
            Span::styled(self.time_remaining.to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  |  Words Typed:"),
            Span::styled(self.word_index.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  |  Quit:"),
            Span::styled("<Ctrl + C> ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let mut spans = vec![];

        for(i, word) in self.target_words.iter().enumerate() {
            for(j,c) in word.chars().enumerate() {
                let style = if i == self.word_index && j == self.char_index {
                    // current
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    let typed_index = self.typed_index(i, j);
                    let typed_char = self.typed.chars().nth(typed_index);
                    match typed_char {
                        Some(tc) if tc == c => Style::default().fg(Color::White), // correct
                        Some(_) => Style::default().fg(Color::Red), // incorrect
                        None => Style::default().fg(Color::DarkGray),
                    }
                };
                spans.push(Span::styled(c.to_string(), style));
            }
            spans.push(Span::raw(" "));
        }

        let mut lines: Vec<Line> = vec![];
        let mut current_line = Vec::new();
        let mut current_width = 0;
        let max_width = area.width as usize - 4;

        for span in spans {
            let span_width = span.content.width();
            if current_width + span_width > max_width {
                lines.push(Line::from(current_line));
                current_line = Vec::new();
                current_width = 0;
            }
            current_width += span_width;
            current_line.push(span);
        }

        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        let paragraph = Paragraph::new(Text::from(lines))
            .block(block)
            .wrap(ratatui::widgets::Wrap {trim: false})
            .alignment(ratatui::layout::Alignment::Left);
        
        paragraph.render(area, buf);
    }
}



fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
