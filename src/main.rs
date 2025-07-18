use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::prelude::IndexedRandom;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{self, Write},
    time::{Duration, Instant},
    vec,
};
use unicode_width::UnicodeWidthStr;

/// Determines which screen to render
#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    EndRound,
    ShowOptions,
}
/// Used for user round time config
#[derive(Debug, Default)]
pub enum RoundTime {
    #[default]
    Default,
    Min,
    TwoMin,
}
/// Used for user text theme config
#[derive(Debug, Default)]
pub enum TextTheme {
    #[default]
    Default,
    Lorem,
    Tech,
    Food,
}
impl TextTheme {
    /// Returns custom word lists based on selection
    pub fn word_list(&self) -> Vec<&'static str> {
        match self {
            // Default = random no correlation
            TextTheme::Default => vec![
                "river",
                "flag",
                "grit",
                "yellow",
                "bounce",
                "flight",
                "shallow",
                "habit",
                "flame",
                "wander",
                "pocket",
                "scrap",
                "blink",
                "canvas",
                "grind",
                "foggy",
                "stream",
                "patrol",
                "branch",
                "tunnel",
                "window",
                "brief",
                "orbit",
                "sand",
                "melt",
                "parade",
                "cliff",
                "border",
                "charge",
                "wild",
                "pepper",
                "crack",
                "shelter",
                "gentle",
                "prize",
                "canyon",
                "loop",
                "motion",
                "splash",
                "note",
                "tiger",
                "shade",
                "glimpse",
                "cradle",
                "velvet",
                "bucket",
                "slide",
                "curve",
                "dizzy",
                "ladder",
                "brick",
                "shadow",
                "humble",
                "filter",
                "stride",
                "clamp",
                "rugged",
                "narrow",
                "float",
                "puzzle",
                "string",
                "burst",
                "echo",
                "gleam",
                "rust",
                "maze",
                "spark",
                "anchor",
                "gravel",
                "tremble",
                "whirl",
                "scrape",
                "dwell",
                "crisp",
                "shiver",
                "badge",
                "frame",
                "cloak",
                "drift",
                "sketch",
                "and",
                "to",
                "by",
                "when",
                "see",
                "went",
                "why",
                "going",
                "because",
                "from",
                "did",
                "he",
                "she",
                "them",
                "pass",
                "type",
                "of",
                "style",
                "run",
                "walk",
                "gym",
                "try",
                "people",
                "alien",
                "horse",
                "marble",
                "cactus",
                "blanket",
                "owl",
                "plunge",
                "jigsaw",
                "mirror",
                "sneeze",
                "lantern",
                "drizzle",
                "planet",
                "hazard",
                "bucket",
                "napkin",
                "monkey",
                "trick",
                "castle",
                "windowpane",
                "cloudy",
                "teapot",
                "marsh",
                "spoon",
                "ticket",
                "plasma",
                "garage",
                "acorn",
                "swing",
                "flicker",
                "giant",
                "nibble",
                "timber",
                "compass",
                "snore",
                "zipper",
                "yawn",
                "pebble",
                "harbor",
                "waffle",
                "candle",
                "basement",
                "knock",
                "murmur",
                "doodle",
                "stairs",
                "plank",
                "groove",
                "tinsel",
                "shrug",
                "breeze",
                "helmet",
            ],
            // Lorem Ipsum placeholder text
            TextTheme::Lorem => vec![
                "lorem",
                "ipsum",
                "dolor",
                "sit",
                "amet",
                "consectetur",
                "adipiscing",
                "elit",
                "sed",
                "do",
                "eiusmod",
                "tempor",
                "incididunt",
                "ut",
                "labore",
                "et",
                "dolore",
                "magna",
                "aliqua",
                "enim",
                "ad",
                "minim",
                "veniam",
                "quis",
                "nostrud",
                "exercitation",
                "ullamco",
                "laboris",
                "nisi",
                "aliquip",
                "ex",
                "ea",
                "commodo",
                "consequat",
                "duis",
                "aute",
                "irure",
                "in",
                "reprehenderit",
                "voluptate",
                "velit",
                "esse",
                "cillum",
                "eu",
                "fugiat",
                "nulla",
                "pariatur",
                "excepteur",
                "sint",
                "occaecat",
                "cupidatat",
                "non",
                "proident",
                "sunt",
                "in",
                "culpa",
                "qui",
                "officia",
                "deserunt",
                "mollit",
                "anim",
                "id",
                "est",
                "laborum",
                "pellentesque",
                "habitant",
                "morbi",
                "tristique",
                "senectus",
                "netus",
                "fames",
                "egestas",
                "vestibulum",
                "turpis",
                "porta",
                "ac",
                "rutrum",
                "ultricies",
                "tellus",
                "interdum",
                "feugiat",
            ],
            // Technology industry theme
            TextTheme::Tech => vec![
                "protocol",
                "server",
                "network",
                "buffer",
                "compile",
                "binary",
                "virtual",
                "hardware",
                "syntax",
                "bytecode",
                "encryption",
                "router",
                "packet",
                "socket",
                "script",
                "kernel",
                "command",
                "thread",
                "function",
                "object",
                "method",
                "memory",
                "cache",
                "firewall",
                "stack",
                "array",
                "boolean",
                "debugger",
                "monitor",
                "driver",
                "firmware",
                "algorithm",
                "bitrate",
                "latency",
                "backend",
                "frontend",
                "database",
                "cluster",
                "token",
                "gateway",
                "docker",
                "branch",
                "commit",
                "push",
                "pull",
                "fork",
                "instance",
                "queue",
                "runtime",
                "lambda",
                "process",
                "render",
                "shader",
                "sandbox",
                "version",
                "editor",
                "module",
                "update",
                "login",
                "crypto",
                "threading",
                "cloud",
                "endpoint",
                "input",
                "output",
                "integer",
                "float",
                "pixel",
                "vector",
                "api",
                "node",
                "scripted",
                "pipeline",
                "session",
                "wrapper",
                "class",
                "static",
                "mutex",
                "ping",
                "bit",
                "send",
                "bug",
                "code",
                "java",
                "rust",
                "script",
                "hack",
                "future",
                "neuron",
                "optimize",
                "discover",
                "laptop",
                "linux",
                "distributed",
                "terminal",
                "protocols",
                "boolean",
                "overflow",
                "index",
                "tokenize",
                "pointer",
                "emulator",
                "container",
                "firmware",
                "registry",
                "framework",
                "decompile",
                "syntax",
                "debug",
                "bitwise",
                "microchip",
                "kernelspace",
                "iteration",
                "recursion",
                "heap",
                "stacktrace",
                "opcode",
                "bootloader",
                "repository",
                "hypervisor",
                "threadsafe",
                "permissions",
                "clipboard",
                "clipboard",
                "filesystem",
                "hotfix",
                "interface",
                "interrupt",
                "macro",
                "hashmap",
                "tokenizer",
                "graphql",
                "typescript",
                "npm",
                "socketio",
                "firestore",
                "environ",
                "hostname",
                "whitespace",
                "compression",
                "checksum",
                "uptime",
                "localhost",
                "rollback",
                "benchmark",
            ],
            // Food adjacent theme, yum!
            TextTheme::Food => vec![
                "banana",
                "broccoli",
                "carrot",
                "pasta",
                "basil",
                "sausage",
                "chili",
                "turmeric",
                "cinnamon",
                "almond",
                "avocado",
                "sushi",
                "taco",
                "burrito",
                "noodle",
                "curry",
                "cocoa",
                "flour",
                "muffin",
                "bagel",
                "popcorn",
                "ginger",
                "apple",
                "spinach",
                "cheddar",
                "gravy",
                "honey",
                "pudding",
                "crumble",
                "risotto",
                "asparagus",
                "pancake",
                "hazelnut",
                "pomegranate",
                "licorice",
                "ravioli",
                "beetroot",
                "peanut",
                "walnut",
                "toffee",
                "casserole",
                "scone",
                "omelette",
                "truffle",
                "pesto",
                "butter",
                "ketchup",
                "pickle",
                "barbecue",
                "meatball",
                "zucchini",
                "anchovy",
                "custard",
                "steak",
                "salmon",
                "parmesan",
                "tomato",
                "grapefruit",
                "lemon",
                "lime",
                "syrup",
                "croissant",
                "crepe",
                "waffle",
                "espresso",
                "latte",
                "mocha",
                "cupcake",
                "cherry",
                "blueberry",
                "mango",
                "plum",
                "fig",
                "kiwi",
                "cabbage",
                "fennel",
                "turnip",
                "radish",
                "eat",
                "yum",
                "tasty",
                "cook",
                "chef",
                "season",
                "spice",
                "salt",
                "sauce",
                "juice",
                "dine",
                "brisket",
                "coconut",
                "mustard",
                "granola",
                "clove",
                "noodles",
                "peach",
                "bruschetta",
                "fondue",
                "sorbet",
                "jerky",
                "calamari",
                "gouda",
                "yogurt",
                "paprika",
                "lentil",
                "okra",
                "shallot",
                "tamarind",
                "caper",
                "durian",
                "gnocchi",
                "kombucha",
                "kale",
                "brandy",
                "cider",
                "miso",
                "kimchi",
                "edamame",
                "quinoa",
                "arugula",
                "crouton",
                "tart",
                "tofu",
                "aioli",
                "hummus",
                "paella",
                "meringue",
                "shrimp",
                "bacon",
                "ramen",
                "chowder",
                "schnitzel",
                "coleslaw",
                "baguette",
                "prawn",
                "margarine",
                "lollipop",
                "veal",
                "churro",
            ],
        }
    }
}
/// Used for I/O of top scores to local store
#[derive(Debug, Clone)]
pub struct TopScore {
    pub date: String,
    pub wpm_score: usize,
}
/// Data class for single round stats result
#[derive(Debug)]
pub struct RoundResult {
    pub correct_words: usize,
    pub total_chars: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub percentage_words: f64,
    pub percentage_chars: f64,
}
/// For interactions in user options theme and saving choices
#[derive(Debug, Default, Clone)]
pub struct ConfigIndex {
    pub round_time_index: usize,
    pub text_theme_index: usize,
    pub choice_index: usize,
}
/// Main application structure
#[derive(Debug, Default)]
pub struct App {
    pub char_index: usize,                 // current char
    pub word_index: usize,                 // current word
    pub typed_words: Vec<String>,          // words typed tracking
    pub target_words: Vec<String>,         // random word gen list
    pub start_time: Option<Instant>,       // round timing
    pub time_remaining: u64,               // count down
    pub exit: bool,                        // exit app loop
    pub current_screen: CurrentScreen,     // screen state
    pub round_time: RoundTime,             // round time theme choice
    pub text_theme: TextTheme,             // text theme choice
    pub top_scores: Option<Vec<TopScore>>, // top scores from file I/O
    pub config: ConfigIndex,               // config state
    pub cooldown_start: Option<Instant>,   // disable key press post round
}
/// Main app functionality
impl App {
    /// Init new app
    pub fn new() -> Self {
        let text_theme = TextTheme::Default;
        let words_list = text_theme.word_list();
        let target_words = generate_words(&words_list, 60);
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
            text_theme: TextTheme::Default,
            top_scores: None,
            config: ConfigIndex {
                round_time_index: 0,
                text_theme_index: 0,
                choice_index: 0,
            },
            cooldown_start: None,
        }
    }
    /// Init app with custom config
    pub fn new_with_config(config: ConfigIndex) -> Self {
        let text_theme = match config.text_theme_index {
            0 => TextTheme::Default,
            1 => TextTheme::Lorem,
            2 => TextTheme::Tech,
            3 => TextTheme::Food,
            _ => panic!("Invalid text_theme_index {}", config.text_theme_index),
        };
        let words_list = text_theme.word_list();
        let target_words = generate_words(&words_list, 60);
        let time_remaining = match config.round_time_index {
            0 => 30,
            1 => 60,
            2 => 120,
            _ => {
                panic!("Invalid round_time_index {}", config.round_time_index);
            }
        };
        let round_time = match config.round_time_index {
            0 => RoundTime::Default,
            1 => RoundTime::Min,
            2 => RoundTime::TwoMin,
            _ => {
                panic!("Invalid round_time_index {}", config.round_time_index);
            }
        };
        let text_theme = match config.text_theme_index {
            0 => TextTheme::Default,
            1 => TextTheme::Lorem,
            2 => TextTheme::Tech,
            3 => TextTheme::Food,
            _ => {
                panic!("Invalid text_theme_index {}", config.text_theme_index);
            }
        };
        Self {
            char_index: 0,
            word_index: 0,
            typed_words: vec![String::new()],
            target_words,
            start_time: None,
            time_remaining: time_remaining,
            exit: false,
            current_screen: CurrentScreen::Main,
            round_time: round_time,
            text_theme: text_theme,
            top_scores: None,
            config: config,
            cooldown_start: None,
        }
    }
    /// Run main app
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Run until exit triggered
        while !self.exit {
            let top_scores = App::get_leaderboard_file_contents();
            if top_scores.is_some() {
                self.top_scores = top_scores;
            }
            terminal.draw(|f| self.draw(f))?; // render TUI
            self.handle_events()?; // handle screen events
        }
        Ok(())
    }
    /// Render TUI
    fn draw(&self, f: &mut Frame) {
        f.render_widget(self, f.area());
    }
    /// Handle screen events
    fn handle_events(&mut self) -> io::Result<()> {
        // Check for new events every 0.05s
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event); // Key press logic
                }
            }
        }
        // Main screen specific logic
        if let CurrentScreen::Main = self.current_screen {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().as_secs();
                // When round finished
                if elapsed >= self.get_round_time() {
                    let round_results = self.get_accuracy();
                    // Calculate stats
                    let wpm = match self.round_time {
                        RoundTime::Default => {
                            (self.word_index - (self.word_index - round_results.correct_words))
                                as f64
                                / 0.5
                        }
                        RoundTime::Min => {
                            (self.word_index - (self.word_index - round_results.correct_words))
                                as f64
                        }
                        RoundTime::TwoMin => {
                            (self.word_index - (self.word_index - round_results.correct_words))
                                as f64
                                / 2.0
                        }
                    };
                    let mut should_update = false; // true if top 10 score
                    if let Some(scores) = &self.top_scores {
                        if scores.len() < 10 || scores.iter().any(|s| wpm as usize > s.wpm_score) {
                            should_update = true;
                        }
                    } else {
                        should_update = true;
                    }
                    // If top 10
                    if should_update {
                        // Update local file
                        let now = Local::now();
                        let date = now.format("%d-%m-%Y").to_string();
                        self.update_leaderboard_file_contents(TopScore {
                            date: date,
                            wpm_score: wpm as usize,
                        });
                    }
                    self.current_screen = CurrentScreen::EndRound; // switch screen
                    self.start_time = None;
                } else {
                    // Countdown logic
                    self.time_remaining = self.get_round_time() - elapsed;
                }
            }
        }
        Ok(())
    }
    /// Handles key press logic
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        use crossterm::event::KeyModifiers;
        // Screen state specific logic
        match self.current_screen {
            CurrentScreen::Main => {
                // Start countdown
                if self.start_time.is_none() {
                    self.start_time = Some(Instant::now());
                }
                match key_event.code {
                    // Ctrl + c = exit
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
                // Start cooldown for user input
                if self.cooldown_start.is_none() {
                    self.cooldown_start = Some(Instant::now());
                }
                // Disable key press for 0.05 sec post game end
                if let Some(start) = self.cooldown_start {
                    if start.elapsed() >= Duration::from_millis(50) {
                        match key_event.code {
                            // q to exit
                            KeyCode::Char('q') => {
                                self.exit = true;
                            }
                            // r to restart game
                            KeyCode::Char('r') => {
                                *self = App::new_with_config(self.config.clone()) // new app with custom config
                            }
                            // e to edit config
                            KeyCode::Char('e') => {
                                self.cooldown_start = None;
                                self.current_screen = CurrentScreen::ShowOptions;
                            }
                            _ => {}
                        }
                    } else {
                        // Discard key press
                        if event::poll(Duration::from_millis(10)).unwrap() {
                            let _ = event::read();
                        }
                    }
                }
            }
            CurrentScreen::ShowOptions => match key_event.code {
                // Switch between options with right arrow or l
                KeyCode::Right | KeyCode::Char('l') => match self.config.choice_index {
                    0 => match self.config.round_time_index {
                        2 => {
                            self.config.round_time_index = 0;
                        }
                        _ => {
                            self.config.round_time_index += 1;
                        }
                    },
                    1 => match self.config.text_theme_index {
                        3 => {
                            self.config.text_theme_index = 0;
                        }
                        _ => {
                            self.config.text_theme_index += 1;
                        }
                    },
                    _ => {}
                },
                // Switch between options with left arrow or h
                KeyCode::Left | KeyCode::Char('h') => match self.config.choice_index {
                    0 => match self.config.round_time_index {
                        0 => {
                            self.config.round_time_index = 2;
                        }
                        _ => {
                            self.config.round_time_index -= 1;
                        }
                    },
                    1 => match self.config.text_theme_index {
                        0 => {
                            self.config.text_theme_index = 3;
                        }
                        _ => {
                            self.config.text_theme_index -= 1;
                        }
                    },
                    _ => {}
                },
                // Move down with arrow or j
                KeyCode::Down | KeyCode::Char('j') => match self.config.choice_index {
                    0 => {
                        self.config.choice_index = 1;
                    }
                    1 => {
                        self.config.choice_index = 2;
                    }
                    2 => {
                        self.config.choice_index = 0;
                    }
                    _ => {}
                },
                // Move up with arrow or k
                KeyCode::Up | KeyCode::Char('k') => match self.config.choice_index {
                    0 => {
                        self.config.choice_index = 2;
                    }
                    1 => {
                        self.config.choice_index = 0;
                    }
                    2 => {
                        self.config.choice_index = 1;
                    }
                    _ => {}
                },
                // Enter to save and exit options screen
                KeyCode::Enter => match self.config.choice_index {
                    2 => self.current_screen = CurrentScreen::EndRound,
                    _ => self.config.choice_index = 2,
                },
                _ => {}
            },
        }
    }
    /// Used to go to next word on screen during game
    fn next_word(&mut self) {
        self.word_index += 1;
        self.char_index = 0;
        if self.typed_words.len() <= self.word_index {
            self.typed_words.push(String::new());
        }
        if self.typed_words.len() > self.target_words.len() - 20 {
            self.extend_lines();
        }
    }
    /// Used to go to prev word on screen during game
    fn prev_word(&mut self) {
        if self.word_index > 0 {
            self.typed_words.pop();
            self.word_index -= 1;
            self.char_index = self.typed_words[self.word_index].len();
        }
    }
    /// Used to add new lines to screen dynamically
    fn extend_lines(&mut self) {
        let word_list = self.text_theme.word_list();
        let extension_words = generate_words(&word_list, 30);
        self.target_words.extend(extension_words);
    }
    /// Render main app
    fn render_main(&self, area: Rect, buf: &mut Buffer) {
        // App title
        let title: &str = r#"
▗▄▄▄▖▗▄▄▄▖▗▄▄▖ ▗▖  ▗▖▗▄▄▄▖▗▖  ▗▖ ▗▄▖ ▗▖       ▗▄▄▄▖▗▖  ▗▖▗▄▄▖ ▗▄▄▄▖
  █  ▐▌   ▐▌ ▐▌▐▛▚▞▜▌  █  ▐▛▚▖▐▌▐▌ ▐▌▐▌         █   ▝▚▞▘ ▐▌ ▐▌▐▌   
  █  ▐▛▀▀▘▐▛▀▚▖▐▌  ▐▌  █  ▐▌ ▝▜▌▐▛▀▜▌▐▌         █    ▐▌  ▐▛▀▘ ▐▛▀▀▘
  █  ▐▙▄▄▖▐▌ ▐▌▐▌  ▐▌▗▄█▄▖▐▌  ▐▌▐▌ ▐▌▐▙▄▄▖      █    ▐▌  ▐▌   ▐▙▄▄▖
"#;
        let title_style = Style::default().fg(Color::LightBlue).bg(Color::default());
        let title_lines: Vec<Line> = title
            .lines()
            .map(|line| Line::from(Span::styled(line.to_string(), title_style)))
            .collect();
        let title_paragraph = Paragraph::new(Text::from(title_lines))
            .block(Block::default())
            .alignment(Alignment::Center);
        // Define grid layout
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(60),
                Constraint::Percentage(15),
            ])
            .split(area);
        let padding_width = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(outer_layout[1]);
        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(padding_width[1]);
        let main_content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(inner_layout[1]);
        // Outer layer content
        let title = Line::from(vec![Span::styled(
            "  Type To Begin  ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]);
        let instructions = Line::from(vec![
            Span::raw(" Time Remaining: "),
            Span::styled(
                self.time_remaining.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  Words Typed: "),
            Span::styled(
                self.word_index.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  Quit: "),
            Span::styled(
                " <Ctrl + C> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        let outer_block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        // Main content for game words
        let main_content_block = Block::default();
        let mut lines: Vec<Line> = vec![]; // stores each output line
        let mut current_line = Vec::new(); // current line state
        let mut current_width = 0;
        let max_width = main_content_layout[1].width as usize; // determine line width

        // Iterare over words and output to main screen buffer
        for (i, word) in self.target_words.iter().enumerate() {
            let mut word_spans = vec![];
            let mut word_width = 0;

            for (j, c) in word.chars().enumerate() {
                let style = if i == self.word_index && j == self.char_index {
                    // Current index highlighted
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    let typed_char = self.typed_words.get(i).and_then(|w| w.chars().nth(j));
                    match typed_char {
                        Some(tc) if tc == c => Style::default().fg(Color::White), // correct
                        Some(_) => Style::default().fg(Color::Red),               // incorrect
                        None => Style::default().fg(Color::DarkGray),             // not typed
                    }
                };
                let span = Span::styled(c.to_string(), style);
                word_width += span.content.width();
                word_spans.push(span);
            }
            word_spans.push(Span::raw(" ")); // space after each word
            word_width += 1;

            if current_width + word_width > max_width {
                lines.push(Line::from(current_line));
                current_line = vec![];
                current_width = 0;
                // Start clearing top line after 8 lines to avoid overflow
                if lines.len() >= 8 {
                    lines.remove(0);
                }
            }

            current_line.extend(word_spans);
            current_width += word_width;
        }
        // Blank outer template
        let outer_paragraph = Paragraph::new(Text::from(""))
            .block(outer_block)
            .alignment(Alignment::Center);
        outer_paragraph.render(padding_width[1], buf);
        // Title paragraph top of app
        title_paragraph.render(outer_layout[0], buf);
        // Main content for game
        let main_paragraph = Paragraph::new(Text::from(lines))
            .block(main_content_block)
            .wrap(ratatui::widgets::Wrap { trim: false })
            .alignment(Alignment::Left);
        main_paragraph.render(main_content_layout[1], buf);
    }
    /// Renders end of round stats and leaderboard
    fn render_end_screen(&self, area: Rect, buf: &mut Buffer) {
        // Define area grid layout
        let padding_height = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);
        let padding_width = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(padding_height[1]);
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(padding_width[1]);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[0]);
        // Get statistics for output
        let round_type = match self.round_time {
            RoundTime::Default => "30s round".to_string(),
            RoundTime::Min => "1 min round".to_string(),
            RoundTime::TwoMin => "2 min round".to_string(),
        };
        let round_results = self.get_accuracy();
        let actual_wpm = match self.round_time {
            RoundTime::Default => {
                (self.word_index - (self.word_index - round_results.correct_words)) as f64 / 0.5
            }
            RoundTime::Min => {
                (self.word_index - (self.word_index - round_results.correct_words)) as f64
            }
            RoundTime::TwoMin => {
                (self.word_index - (self.word_index - round_results.correct_words)) as f64 / 2.0
            }
        };
        let raw_wpm = match self.round_time {
            RoundTime::Default => self.word_index as f64 / 0.5,
            RoundTime::Min => self.word_index as f64,
            RoundTime::TwoMin => self.word_index as f64 / 2.0,
        };
        // Top left block for round stats
        let top_left_title = Line::from(vec![Span::styled(
            " Round Summary ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]);
        let top_left_block = Block::default()
            .title(top_left_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let user_stats = Text::from(vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::styled(
                    format!("WPM: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", actual_wpm),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("RAW WPM: "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", raw_wpm),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
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
                    format!("{:.1} %", round_results.percentage_words),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
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
                    format!("{:.1} %", round_results.percentage_chars),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
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
                ),
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
                ),
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
                ),
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
                ),
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
                ),
            ])
            .centered(),
        ]);
        let stats_paragraph = Paragraph::new(user_stats)
            .block(top_left_block)
            .alignment(Alignment::Center);
        stats_paragraph.render(inner_layout[0], buf);
        // Top right block for leaderboard
        let top_right_title = Line::from(vec![Span::styled(
            " Leaderboard ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]);
        let top_right_block = Block::default()
            .title(top_right_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let mut leaderboard_lines: Vec<Line> = Vec::<Line>::new();
        leaderboard_lines.push(Line::from(vec![Span::raw("")]));
        if let Some(scores) = &self.top_scores {
            for (i, score) in scores.iter().enumerate() {
                let line = Line::from(vec![
                    Span::styled(
                        format!("{}: ", (i + 1)),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{}  ", score.date),
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{} WPM", score.wpm_score),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
                .centered();
                leaderboard_lines.push(line);
            }
        } else {
            let line = Line::from("No scores recorded").centered();
            leaderboard_lines.push(line);
        };
        let leaderboard_paragraph = Paragraph::new(Text::from(leaderboard_lines))
            .block(top_right_block)
            .alignment(Alignment::Center);
        leaderboard_paragraph.render(inner_layout[1], buf);
        // Bottom block for user options
        let bottom_title = Line::from(vec![Span::styled(
            " User Options ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]);
        let bottom_block = Block::default()
            .title(bottom_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let user_options = Text::from(vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::styled(
                    format!("Press "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("'r'"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" to play again"),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("Press "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("'e'"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" to edit user config"),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .centered(),
            Line::from(vec![
                Span::styled(
                    format!("Press "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("'q'"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" to quit terminal"),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
            .centered(),
        ]);
        let bottom_paragraph = Paragraph::new(user_options)
            .block(bottom_block)
            .alignment(Alignment::Center);
        bottom_paragraph.render(outer_layout[1], buf);
    }
    /// Renders user options screen
    fn render_options(&self, area: Rect, buf: &mut Buffer) {
        // Grid layout
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area);
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(outer_layout[1]);
        let title = Line::from(vec![Span::styled(
            format!(" User Config "),
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]);
        let options_block = Block::default()
            .title(title.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);
        let round_time_options = vec!["30 Seconds", "1 Minute", "2 Minute"];
        let text_theme_options = vec!["Default", "Lorem Ipsum", "Technology", "Food"];
        let options_text = Text::from(vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                format!("Round Time"),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::raw("< "),
                Span::styled(
                    format!("{}", round_time_options[self.config.round_time_index]),
                    Style::default()
                        .fg(if self.config.choice_index == 0 {
                            Color::Black
                        } else {
                            Color::White
                        })
                        .bg(if self.config.choice_index == 0 {
                            Color::LightBlue
                        } else {
                            Color::Reset
                        })
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" >"),
            ]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                format!("Word Theme"),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![
                Span::raw("< "),
                Span::styled(
                    format!("{}", text_theme_options[self.config.text_theme_index]),
                    Style::default()
                        .fg(if self.config.choice_index == 1 {
                            Color::Black
                        } else {
                            Color::White
                        })
                        .bg(if self.config.choice_index == 1 {
                            Color::LightBlue
                        } else {
                            Color::Reset
                        })
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" >"),
            ]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                format!("Save"),
                Style::default()
                    .fg(if self.config.choice_index == 2 {
                        Color::Black
                    } else {
                        Color::White
                    })
                    .bg(if self.config.choice_index == 2 {
                        Color::Green
                    } else {
                        Color::Reset
                    })
                    .add_modifier(Modifier::BOLD),
            )]),
        ]);
        let options_paragraph = Paragraph::new(options_text)
            .block(options_block)
            .alignment(Alignment::Center);
        options_paragraph.render(inner_layout[1], buf);
    }
    // Reurns round time for countdown
    fn get_round_time(&self) -> u64 {
        match self.round_time {
            RoundTime::Default => return 30,
            RoundTime::Min => return 60,
            RoundTime::TwoMin => return 120,
        }
    }
    // Returns end of round statistics
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
    /// File input to get local leaderboard
    fn get_leaderboard_file_contents() -> Option<Vec<TopScore>> {
        // Get file path
        let home_dir = dirs::home_dir()?;
        let leaderboard_file_path = home_dir.join(".local/share/TerminalType/leaderboard.txt");
        let contents = fs::read_to_string(leaderboard_file_path).ok()?;
        if contents.trim().is_empty() {
            return None;
        }
        // Extract scores
        let mut top_scores = Vec::<TopScore>::new();
        for line in contents.lines() {
            let mut parts = line.trim().split_whitespace();
            let date = parts.next()?;
            let wpm_str = parts.next()?;
            let wpm_value = wpm_str.parse::<usize>().ok()?;
            top_scores.push(TopScore {
                date: date.to_string(),
                wpm_score: wpm_value,
            });
        }
        // Return result
        if top_scores.is_empty() {
            None
        } else {
            Some(top_scores)
        }
    }
    /// File outpur for round results in top 10
    fn update_leaderboard_file_contents(&self, new_top_score: TopScore) {
        // Get file path
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return,
        };
        let leaderboard_dir = home_dir.join(".local/share/TerminalType");
        let leaderboard_file_path = leaderboard_dir.join("leaderboard.txt");
        // Validate exists
        if let Err(e) = fs::create_dir_all(&leaderboard_dir) {
            eprintln!("Failed to create directory for output file: {}", e);
            return;
        }
        // Load existing top_scores
        let mut scores = self.top_scores.clone().unwrap_or_else(Vec::new);
        scores.push(new_top_score);
        // Sort by WPM
        scores.sort_by(|a, b| b.wpm_score.cmp(&a.wpm_score));
        // Keep top 10
        scores.truncate(10);
        // Format lines
        let lines: Vec<String> = scores
            .iter()
            .map(|s| format!("{} {}", s.date, s.wpm_score))
            .collect();
        // Write to file
        match File::create(&leaderboard_file_path) {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", lines.join("\n")) {
                    eprintln!("Error writing to leaderboard.txt: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to open leaderboard.txt: {}", e),
        }
    }
}
/// Custom render logic for App which inherits Widget
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Main => self.render_main(area, buf),
            CurrentScreen::EndRound => self.render_end_screen(area, buf),
            CurrentScreen::ShowOptions => self.render_options(area, buf),
        }
    }
}
// Helper function to extract n random words from a list
fn generate_words(words: &[&str], count: usize) -> Vec<String> {
    let mut rng = rand::rng();
    let mut past_ten_words = VecDeque::new();
    let mut random_words = Vec::new();
    while random_words.len() < count {
        let word = words.choose(&mut rng).unwrap().to_string();
        if past_ten_words.len() < 10 {
            if past_ten_words.contains(&word) {
                continue;
            } else {
                past_ten_words.push_back(word.clone());
                random_words.push(word);
            }
        } else {
            if past_ten_words.contains(&word) {
                continue;
            } else {
                past_ten_words.pop_front();
                past_ten_words.push_back(word.clone());
                random_words.push(word);
            }
        }
    }
    return random_words;
}
/// Main function to run app
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
