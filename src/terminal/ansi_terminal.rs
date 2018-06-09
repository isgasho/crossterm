//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use {Construct, Terminal};
use shared::functions;
use super::{ClearType, ITerminal};

use std::io;
use std::io::Write;
use std::sync::Mutex;
use std::cell::RefCell;

use ScreenManager;

/// This struct is an ansi implementation for terminal related actions.
pub struct AnsiTerminal;

impl Construct for AnsiTerminal {
    fn new() -> Box<AnsiTerminal> {
        Box::from(AnsiTerminal {})
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType, terminal: &Terminal) {

        let mut screen_manager = terminal.screen_manager.lock().unwrap();
        {
            let mut stdout = screen_manager.stdout();

            match clear_type {
                ClearType::All => {
                    write!(stdout, csi!("2J"));
                },
                ClearType::FromCursorDown => {
                    write!(stdout, csi!("J"));
                },
                ClearType::FromCursorUp => {
                    write!(stdout, csi!("1J"));
                },
                ClearType::CurrentLine => {
                    write!(stdout, csi!("2K"));
                },
                ClearType::UntilNewLine => {
                    write!(stdout, csi!("K"));
                },
            };
        }
    }

    fn terminal_size(&self, terminal: &Terminal) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}S"), count));
        }
    }

    fn scroll_down(&self, count: i16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("{}T"), count));
        }
    }

    fn set_size(&self, width: i16, height: i16, terminal: &Terminal) {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi(format!(csi!("8;{};{}t"), width, height));
        }
    }
}
