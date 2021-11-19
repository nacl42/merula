//! Experimental readline frontend

//! TODOs

//! In a first step, implement read-only actions, i.e. duplicate the
//! functionality of the command line client. We should then be able
//! to move code common to the readline and command line interface
//! into the merula library itself.
//!

//! - for now: load sample data by default (later: command line arg)

//! - mql ... = set mql expression

//! - filter ... = use pre-defined filter

//! - prompt: filter, number of memos, current memo
//!   [density > 5] (20/100)

//! - proper completion for commands and arguments

//! - ls | ls all

//! - stats collection
//!   094 book
//!   005 note
//!   001 mr:filter

//! TODO: allow command line arguments, such as loading a file

//! Manipulation of items is a much more difficult step and goes
//! beyond the functionality provided by the current merula
//! executable.

//! TODO: take a look at the ropey library to perform inline
//! manipulation of text files (in this case, the database .mr file)

//!----------------------------------------------------------------------

use std::borrow::Cow::{self, Borrowed, Owned};

use merula::prelude::*;

use merula::{
    parser::read_from_file,
    mql::parse_mql,
    display,
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


#[derive(Default)]
struct AppState {
    memos: Vec<Memo>,
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

    let mut state = AppState::default();
    
    println!("Use 'q' to quit and 'h' for help.");
    loop {
        let prompt = format!(
            "{len} {prompt} ",
            len = state.memos.len(),
            prompt = "»".bold()
        );
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let line = line.trim();
                let mut args = line.split_whitespace();
                let cmd = args.next();
                match cmd {
                    Some("q") | Some("quit") => {
                        println!("quit!!!");
                        return;
                    },
                    Some("h") | Some("help") => {
                        println!("commands:");
                        println!("h | help         print this help");
                        println!("q | quit         quit");
                        println!("load file        load .mr data file");
                        println!("ls               list all loaded memos");
                        println!("clear            clear memo database");
                        println!("v | view <n>     view memo with given id")
                    },
                    Some("load") => {
                        println!("load file");
                        for arg in args {
                            print!("reading from file '{}'...", arg);
                            if let Ok(new_memos) = read_from_file(&arg) {
                                println!("{} memos", new_memos.len());
                                state.memos.extend(new_memos);
                            } else {
                                println!("failed!");
                            }
                           
                        };
                    },
                    Some("ls") => {
                        for (n, memo) in state.memos.iter().enumerate() {
                            //.filter(|&memo| memo_filter.check(memo)) {
                            print!(
                                "{}",
                                format!("{:<5} ", n).white()
                            );
                            display::print_header(&memo);
                        };
                    },
                    Some("v") | Some("view")=> {
                        if let Some(n) = args.next() {
                            if let Ok(n) = n.parse::<usize>() {
                                //println!("view #{}", n);
                                if let Some(memo) = state.memos.get(n) {
                                    display::print_header(&memo);
                                    display::print_data_nodes(&memo);
                                } else {
                                    println!("invalid memo #{}, not found", n);
                                }
                            } else {
                                println!("please specify an integer for the memo id");
                            }
                        }
                    },
                    Some("clear") => {
                        state.memos.clear();
                        println!("all memos removed");
                    }
                    None => {}, // ignore empty input
                    Some(_) => {
                        println!("unrecognized command '{:?}'", cmd);
                    }
                };
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
    let _ = rl.append_history("history.txt");
}
