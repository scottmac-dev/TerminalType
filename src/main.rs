use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::IndexedRandom;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
    path::{PathBuf},
    fs::{self, File},
};
use unicode_width::UnicodeWidthStr;
use chrono::{Local};

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    EndRound,
}
#[derive(Debug, Default)]
pub enum RoundTime {
    #[default]
    Default,
    Min,
    TwoMin,
    FiveMin,
}
#[derive(Debug)]
pub struct TopScore {
   pub date: String,
   pub wpm_score: usize,
}
#[derive(Debug)]
pub struct RoundResult {
    pub correct_words: usize,
    pub total_chars: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub percentage_words: f64,
    pub percentage_chars: f64,
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
    pub round_time: RoundTime,
    pub top_scores: Option<Vec<TopScore>>,
}
impl App {
    pub fn new() -> Self {
        let words_list = vec![
            "hello",
            "world",
            "type",
            "rust",
            "juice",
            "the",
            "lazy",
            "dog",
            "jumped",
            "over",
            "sleeping",
            "fox",
            "disgrace",
            "snap",
            "crop",
            "pot",
            "sound",
            "amber",
            "code",
            "intelligence",
            "chicken",
            "soup",
            "tower",
            "dough",
            "normal",
            "speed",
            "better",
            "minute",
            "best",
            "ever",
            "to",
            "and",
            "when",
            "by",
            "learn",
            "code",
            "gain",
            "buffer",
            "money",
            "start",
            "stop",
            "write",
            "food",
            "gym",
            "vector",
            "monkey",
            "through",
            "threw",
            "undo",
        ];
        let mut rng = rand::rng();
        let target_words = (0..50)
            .map(|_| words_list.choose(&mut rng).unwrap().to_string())
            .collect();
        Self {
            char_index: 0,
            word_index: 0,
            typed_words: vec![String::new()],
            target_words,
            start_time: None,
            time_remaining: 30,
            exit: false,
            current_screen: CurrentScreen::Main,
            round_time: RoundTime::Default,
            top_scores: None,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let top_scores = App::get_leaderboard_file_contents();
            if top_scores.is_some() {
                self.top_scores = top_scores;
            }
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, f: &mut Frame) {
        f.render_widget(self, f.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }

        // Main screen logic
        if let CurrentScreen::Main = self.current_screen {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().as_secs();
                if elapsed >= self.get_round_time() {
                    self.current_screen = CurrentScreen::EndRound;
                    self.start_time = None;
                } else {
                    self.time_remaining = self.get_round_time() - elapsed;
                }
            }
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        use crossterm::event::KeyModifiers;

        match self.current_screen {
            CurrentScreen::Main => {
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }

                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit = true;
                    }
                    KeyCode::Char(' ') => {
                        if self.char_index > 0 {
                            self.next_word();
                        }
                    }
                    KeyCode::Char(c) => {
                        if self.typed_words.len() <= self.word_index {
                            self.typed_words.push(String::new());
                        }
                        self.typed_words[self.word_index].push(c);
                        self.char_index += 1;
                    }
                    KeyCode::Backspace => {
                        if self.char_index > 0 {
                            self.char_index -= 1;
                            self.typed_words[self.word_index].pop();
                        } else {
                            self.prev_word()
                        }
                    }
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
    fn next_word(&mut self) {
        self.word_index += 1;
        self.char_index = 0;

        if self.typed_words.len() <= self.word_index {
            self.typed_words.push(String::new());
        }
    }
    fn prev_word(&mut self) {
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
            Span::styled(
                self.time_remaining.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  Words Typed:"),
            Span::styled(
                self.word_index.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  Quit:"),
            Span::styled(
                " <Ctrl + C> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
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

        for (i, word) in self.target_words.iter().enumerate() {
            let mut word_spans = vec![];
            let mut word_width = 0;

            for (j, c) in word.chars().enumerate() {
                let style = if i == self.word_index && j == self.char_index {
                    // current
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    let typed_char = self.typed_words.get(i).and_then(|w| w.chars().nth(j));
                    match typed_char {
                        Some(tc) if tc == c => Style::default().fg(Color::White), // correct
                        Some(_) => Style::default().fg(Color::Red),               // incorrect
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
            .wrap(ratatui::widgets::Wrap { trim: false })
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
    fn render_end_screen(&self, area: Rect, buf: &mut Buffer) {
        // Define area grid layout
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(outer_layout[0]);
        // Get statistics for output
        let wpm = match self.round_time {
            RoundTime::Default => self.word_index as f64 / 0.5,
            RoundTime::Min => self.word_index as f64,
            RoundTime::TwoMin => self.word_index as f64 / 2.0,
            RoundTime::FiveMin => self.word_index as f64 / 5.0,
        };
        let round_type = match self.round_time {
            RoundTime::Default => "30s round".to_string(),
            RoundTime::Min => "1 min round".to_string(),
            RoundTime::TwoMin => "2 min round".to_string(),
            RoundTime::FiveMin => "5 min round".to_string(),
        };
        let round_results = self.get_accuracy();
        // Top left block for round stats
        let top_left_block = Block::default()
            .title("== Round Summary ==")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let user_stats = Text::from(vec![
            Line::from(vec![
                Span::styled(
                    format!("WPM: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}",wpm),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("WORD ACCURACY: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}%", round_results.percentage_words),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),            
            Line::from(vec![
                Span::styled(
                    format!("CHAR ACCURACY: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:.1}%", round_results.percentage_chars),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("WORDS TYPED: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", self.word_index),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("WORDS CORRECT: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),                
                Span::styled(
                    format!("{}", round_results.correct_words),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("CHARS TYPED: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),       
                Span::styled(
                    format!("{}", round_results.total_chars),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("CORRECT CHARS: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ), 
                Span::styled(
                    format!("{}", round_results.correct_chars),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("TYPE: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", round_type),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            ])
            .centered(),
        ]);
        let stats_paragraph = Paragraph::new(user_stats)
            .block(top_left_block)
            .alignment(Alignment::Center);
        stats_paragraph.render(inner_layout[0], buf);
        // Top right block for leader board
        let top_right_block = Block::default()
            .title("== Leaderboard ==")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let leaderboard_paragraph = Paragraph::new("test")
            .block(top_right_block)
            .alignment(Alignment::Center);
        leaderboard_paragraph.render(inner_layout[1], buf);
        // Bottom block for user options
        let bottom_block = Block::default()
            .title("== User Options ==")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let user_options = Text::from(vec![
            Line::from("Press 'r' to play again or 'q' to quit").centered(),
        ]);
        let bottom_paragraph = Paragraph::new(user_options)
            .block(bottom_block)
            .alignment(Alignment::Center);
        bottom_paragraph.render(outer_layout[1], buf);
    }
    // Functions for reurning game stats
    fn get_round_time(&self) -> u64 {
        match self.round_time {
            RoundTime::Default => return 30,
            RoundTime::Min => return 60,
            RoundTime::TwoMin => return 120,
            RoundTime::FiveMin => return 300,
        }
    }
    fn get_accuracy(&self) -> RoundResult {
        let num_words_typed = self.word_index;
        let typed_words = &self.typed_words[..num_words_typed];
        let comparison_words = &self.target_words[..num_words_typed];
        let mut total_chars = 0;
        let mut correct_chars = 0;
        let mut correct_words = 0;

        for (typed_word, target_word) in typed_words.iter().zip(comparison_words.iter()) {
            let typed_chars: Vec<char> = typed_word.chars().collect();
            let target_chars: Vec<char> = target_word.chars().collect();
            total_chars += typed_chars.len();

            // match
            if typed_word == target_word {
                correct_words += 1;
                correct_chars += typed_chars.len();
            } else {
                for (typed_c, target_c) in typed_chars.iter().zip(target_chars.iter()) {
                    if typed_c == target_c {
                        correct_chars += 1;
                    }
                }
            }
        }
        let incorrect_chars = total_chars - correct_chars;
        let percentage_words = (correct_words as f64 / num_words_typed as f64) * 100 as f64;
        let percentage_chars = (correct_chars as f64 / total_chars as f64) * 100 as f64;
        let res = RoundResult {
            correct_words: correct_words,
            total_chars: total_chars,
            correct_chars: correct_chars,
            incorrect_chars: incorrect_chars,
            percentage_words: percentage_words,
            percentage_chars: percentage_chars,
        };
        return res;
    }
    fn get_leaderboard_file_contents() -> Option<Vec<TopScore>> {
        // For saving top 10 wpm scores to output file for leaderboard
        let HOME_DIR = dirs::home_dir().expect("Error: couldnt locate home directory");
        let mut leaderboard_file_path = PathBuf::from(HOME_DIR);
        leaderboard_file_path.push(".local/share/TerminalType/leaderboard.txt");

        // Expected format of file leaderboard.txt
        // 01-01-2025 44
        // 02-01-2025 38
        // ...
        let contents = fs::read_to_string(leaderboard_file_path).expect("Error reading leaderboard.txt");
        if contents.is_empty(){
            return None;
        }
        let top_scores_from_file: Vec<String> = contents.split("\n").collect();
        let mut top_scores = Vec::<TopScore>::new();
        for line in top_scores_from_file.iter() {
            let values: Vec<String> = line.split(' ').collect();
            let score = TopScore{
                date: values[0].trim(),
                wpm_score: values[1].trim().parse()?,
            };
            top_scores.push(score);
        }
        return Some(top_scores)
    }
    fn update_leaderboard_file_contents(new_top_score: TopScore) {
        // will only be called if score is in top 10 
        // drops lowest date/score and replaces with new top score
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

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result}
