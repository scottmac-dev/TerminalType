use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
    execute,
};
use rand::prelude::IndexedRandom;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Paragraph, Block, Borders},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    text::{Line, Text, Span},
};
use std::{io::{stdout}, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    enable_raw_mode()?;
    
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let words_list = vec!["hello", "world", "type", "rust", "juice", "the", "lazy", "dog", "jumped", "over", "sleeping", "fox", "disgrace", "snap", "crop", "pot", "sound", "amber", "code", "intelligence", "chicken", "soup", "tower", "dough", "normal", "speed", "better", "minute", "best", "ever", "to", "and", "when", "by", "learn", "code", "gain", "buffer", "money", "start", "stop", "write", "food", "gym", "vector", "monkey", "through", "threw", "undo"];
    let mut rng = rand::rng();
    let target_words: Vec<&str> = (0..50).map(|_| *words_list.choose(&mut rng).unwrap()).collect();

    let mut typed = String::new();
    let mut word_index = 0;
    let mut char_index = 0;
    let mut start_time = None;

    loop {
        let mut lines = vec![];

        for(i, word) in target_words.iter().enumerate() {
            let mut word_spans = vec![];
            for(j,c) in word.chars().enumerate() {
                let style = if i < word_index || (i == word_index && j < char_index){
                    let typed_char = typed.chars().nth(i * 100 + j);
                    match typed_char {
                        Some(tc) if tc == c => Style::default().fg(Color::White),
                        Some(_) => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Gray),
                    }
                } else if i == word_index && j == char_index {
                    Style::default().add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                word_spans.push(Span::styled(c.to_string(), style));
            }
            word_spans.push(Span::raw(" "));
            lines.push(Line::from(word_spans));
        }

        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default().title("Terminal Type TUI").borders(Borders::ALL);
            let paragraph = Paragraph::new(lines).block(block);
            f.render_widget(paragraph, size);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if start_time.is_none(){
                    start_time = Some(Instant::now());
                }

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => break,
                    KeyCode::Char(c) => {
                        typed.push(c);
                        char_index += 1;
                        if char_index >= target_words[word_index].len() {
                            // validate incorrect length logic
                        }
                    },
                    KeyCode::Backspace => {
                        if char_index > 0 {
                            char_index -= 1;
                            typed.pop();
                        }
                    },
                    KeyCode::Tab | KeyCode::Enter | KeyCode::Char(' ') => {
                        word_index += 1;
                        char_index = 0;
                    },
                    _ => {}
                }

                if word_index >= target_words.len() {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Some(start) = start_time {
        let elapsed = start.elapsed().as_secs_f64();
        let wpm = (word_index as f64 / elapsed) * 60.0;
        println!("\nWPM: {:.2}", wpm);
    }

    Ok(())
}
