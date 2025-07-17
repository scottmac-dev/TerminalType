# TerminalType ⌨️🔥

**TerminalType** is a fast and lightweight terminal-based touch typing application written in **Rust**, powered by [`ratatui`](https://github.com/ratatui-org/ratatui) and [`crossterm`](https://github.com/crossterm-rs/crossterm).

It’s designed as a fun way to help you improve your typing speed and accuracy directly from the terminal.

## 💾 Installation
### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- A terminal that supports ANSI escape sequences (most modern terminals do)
- Tested on Arch Linux and Windows ✅ Unsure for Mac 🤷‍♂️

### 🛠️ Build from Source

```bash
# Clone the repository
git clone https://github.com/scottmac-dev/TerminalType.git
cd terminaltype

# Build the app
cargo build --release

# Run in release mode
cargo run --release

# Or if built
./target/release/TerminalType
```
## 🚀 Features

- 📊 **Typing Stats**  
  Get feedback on your typing speed (WPM), accuracy, error rate, and more

- 🏆 **Local Leaderboard**  
  Track your top 10 scores to help track your progress over time

- 🛠️ **Custom Configuration**  
  Adjust round duration and choose from a variety of text themes to match your typing goals or aesthetic preferences

- 🖥️ **TUI Interface**  
  Built with `ratatui` for a responsive, keyboard-driven interface that works seamlessly across platforms

## ⚙️ User Options
Round Time:
- Default 30s
- 1 min
- 2 min

Text Theme Options:
- Default: random words with no correlation
- Lorem: lorem ipsum placeholder text
- Tech: technology themed words
- Food: food themed words

## 📦 Built With

- **[Rust](https://www.rust-lang.org/)** – safe, fast, and efficient systems programming language
- **[ratatui](https://github.com/ratatui-org/ratatui)** – rich TUI library for building modern terminal UIs
- **[crossterm](https://github.com/crossterm-rs/crossterm)** – cross-platform terminal manipulation

## ℹ️ Background
I wanted to improve my typing speed... instead I built a typing speed TUI application

I dont think my typing speed is any better but it was certainly a cool project for learning Rust

Always down for feedback, relatively new to Rust so probably alot that could be improved

If you like the app give it a ⭐

## 🔮 Future
A few things I would be keen to implement in an ideal world.
- Colour theme selection in user config options
- A smoother transition for extending lines when adding more words to keep the cursor in the same position
- Testing 🥲
