use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::IndexedRandom;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize, Color, Modifier},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Paragraph, Block, Widget, Borders},
    prelude::{Layout, Direction, Constraint},
    DefaultTerminal, Frame,
};
use std::{io, time::{Instant, Duration}};
use unicode_width::UnicodeWidthStr;

const DEFAULT_ROUND_TIME_SEC: u64 = 30;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default] Main,
    EndRound,
}

#[derive(Debug, Default)]
pub struct App {
    pub char_index: usize,
    pub word_index: usize,
    pub typed_words: Vec<String>,
    pub target_words: Vec<String>,
    pub start_time: Option<Instant>,
    pub time_remaining: u64,
    pub exit: bool,
    pub current_screen: CurrentScreen,
}
impl App {
    pub fn new() -> Self {
        let words_list = vec!["hello", "world", "type", "rust", "juice", "the", "lazy", "dog", "jumped", "over", "sleeping", "fox", "disgrace", "snap", "crop", "pot", "sound", "amber", "code", "intelligence", "chicken", "soup", "tower", "dough", "normal", "speed", "better", "minute", "best", "ever", "to", "and", "when", "by", "learn", "code", "gain", "buffer", "money", "start", "stop", "write", "food", "gym", "vector", "monkey", "through", "threw", "undo"];
        let mut rng = rand::rng();
        let target_words = (0..50)
            .map(|_| words_list.choose(&mut rng).unwrap().to_string()).collect();
        Self {
            char_index: 0,
            word_index: 0,
            typed_words: vec![String::new()],
            target_words,
            start_time: None,
            time_remaining: DEFAULT_ROUND_TIME_SEC,
            exit: false,
            current_screen: CurrentScreen::Main,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        };
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
                if elapsed >= DEFAULT_ROUND_TIME_SEC {
                    self.current_screen = CurrentScreen::EndRound;
                    return Ok(());
                } else {
                    self.time_remaining = DEFAULT_ROUND_TIME_SEC - elapsed;
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

        match self.current_screen {
            CurrentScreen::Main => {
                if self.start_time.is_none(){
                    self.start_time = Some(Instant::now());
                }

                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.exit = true;
                    },
                    KeyCode::Char(' ') => {
                        if self.char_index > 0 {
                            self.next_word();
                        }
                    },
                    KeyCode::Char(c) => {
                        if self.typed_words.len() <= self.word_index {
                            self.typed_words.push(String::new());
                        }
                        self.typed_words[self.word_index].push(c);
                        self.char_index += 1;
                    },
                    KeyCode::Backspace => {
                        if self.char_index > 0 {
                            self.char_index -= 1;
                            self.typed_words[self.word_index].pop();
                        } else {
                            self.prev_word()
                        }
                    },
                    _ => {}
                }
            }
            CurrentScreen::EndRound => {
                match key_event.code {
                    KeyCode::Char('q') => {
                        self.exit = true;
                    }
                    KeyCode::Char('r') => {
                        *self = App::new() // reset to default
                    }
                    _ => {}
                }
            }
        }
       
    }
    fn next_word(&mut self){
        self.word_index += 1;
        self.char_index = 0;

        if self.typed_words.len() <= self.word_index {
            self.typed_words.push(String::new());
        }
    }
    fn prev_word(&mut self){
        if self.word_index > 0 {
            self.typed_words.pop();
            self.word_index -= 1;
            self.char_index = self.typed_words[self.word_index].len();
        }
    }
    fn render_main(&self, area: Rect, buf: &mut Buffer) {
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
        let inner_area = block.inner(area);
        
        let mut lines: Vec<Line> = vec![];
        let mut current_line = Vec::new();
        let mut current_width = 0;
        let max_width = inner_area.width as usize;

        for(i, word) in self.target_words.iter().enumerate() {
            let mut word_spans = vec![];
            let mut word_width = 0;

            for(j,c) in word.chars().enumerate() {
                let style = if i == self.word_index && j == self.char_index {
                    // current
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    let typed_char = self.typed_words
                        .get(i)
                        .and_then(|w| w.chars().nth(j));
                    match typed_char {
                        Some(tc) if tc == c => Style::default().fg(Color::White), // correct
                        Some(_) => Style::default().fg(Color::Red), // incorrect
                        None => Style::default().fg(Color::DarkGray),
                    }
                };
                let span = Span::styled(c.to_string(), style);
                word_width += span.content.width();
                word_spans.push(span);
            }
            word_spans.push(Span::raw(" "));
            word_width += 1;

            if current_width + word_width > max_width {
                lines.push(Line::from(current_line));
                current_line = vec![];
                current_width = 0;
            }

            current_line.extend(word_spans);
            current_width += word_width;
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
    fn render_end_screen(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);
        
        let p1 = Paragraph::new("Top").block(Block::new().borders(Borders::ALL));
        let p2 = Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL));
        p1.render(layout[0], buf);
        p2.render(layout[1], buf);
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Main => self.render_main(area, buf),
            CurrentScreen::EndRound => self.render_end_screen(area, buf),

        }
    }
}

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
