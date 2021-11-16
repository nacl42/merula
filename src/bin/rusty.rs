//! Experimental CLI frontend

use std::borrow::Cow::{self, Borrowed, Owned};

use merula::{
    memo::{Memo, NodeType}
};

use rustyline::{
    highlight::Highlighter,
    error::ReadlineError,
    {Cmd, Editor, KeyEvent, EventHandler, Event, EventContext, RepeatCount, ConditionalEventHandler},
    hint::{Hinter, HistoryHinter},
    Context
};

use rustyline_derive::{Completer, Helper, Validator};

use simplelog::*;
use log::*;
use colored::*;


fn init_logger(log_level: u8) {
    let simple_log = SimpleLogger::new(LevelFilter::Info, Config::default());
    
    match log_level {
        0 => {
            // no verbose flag given
            // => no logging
        },
        1 => {
            // one verbose flag given
            // => just print out the info statements on stdout
            CombinedLogger::init(vec![simple_log]).unwrap();
        },
        _ => {
            // at least two verbose flag given
            // => additionally, write all debug output to stdout
            let debug_log = TermLogger::new(
                LevelFilter::Debug, Config::default(), TerminalMode::Stderr
            );
            CombinedLogger::init(vec![simple_log, debug_log]).unwrap();
        }
    }
}



#[derive(Completer, Helper, Validator)]
struct MyHelper(HistoryHinter);

impl Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.0.hint(line, pos, ctx)
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Owned(format!("\x1b[1;32m{}\x1b[m", prompt))
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[1m{}\x1b[m", hint))
    }
}

struct TabEventHandler;

impl ConditionalEventHandler for TabEventHandler {
    fn handle(&self, evt: &Event, n: RepeatCount, _: bool, ctx: &EventContext) -> Option<Cmd> {
        debug_assert_eq!(*evt, Event::from(KeyEvent::from('\t')));
        if ctx.line()[..ctx.pos()]
            .chars()
            .rev()
            .next()
            .filter(|c| c.is_whitespace())
            .is_some()
        {
            Some(Cmd::SelfInsert(n, '\t'))
        } else {
            None // default complete
        }
    }
}

fn main() {
    init_logger(1);

    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .edit_mode(rustyline::EditMode::Emacs)
        .auto_add_history(true)
        .build();
    
    let mut rl = Editor::<()>::with_config(config);
    
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    rl.bind_sequence(KeyEvent::from('\t'), EventHandler::Conditional(Box::new(TabEventHandler)));
    
    if rl.load_history("history.txt").is_err() {
        println!("no previous history");
    }
    
    loop {
        let readline = rl.readline("» ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {:?}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Quit");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Eof");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.append_history("hystory.txt");
}
