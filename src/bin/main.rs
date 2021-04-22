//! merula is a command-line client for the flat-file, plain-text
//! database format with the same name and the extension `.mr`.
//!
//! DISCLAIMER: This is work-in-progress and not at all ready for
//! production purposes.
//!

use merula::{
    memo::{Memo},
    node::Node,
    value::{Value, Key},
    filter::{NodeFilter, KeyFilter, ValueFilter, MemoFilter},
    parser::read_from_file,
    mql::parse_mql
};

use simplelog::*;
use log::*;
use colored::*;

#[allow(unused_imports)]
use clap::{App, crate_version, Arg};
#[allow(unused_imports)]
use clap_generate::{generate, generators::Bash};

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


fn main() {
    let app = App::new("merula")
        .version(crate_version!())
        .author("nacl42 <code@sreblov.de>")
        .about("simple cli frontend to access merula files (.mr)")
        .arg("-d --debug... 'Sets the debug level'")
        .subcommand(
            App::new("list")
                .about("list memos")
                .arg("<input> 'sets an input file'")
                .arg("--mql=[MQL] 'sets a mql expression'")
                .arg("--filter=[FILTER] 'load an mql expression from a pre-defined filter'")
                .arg("-v --verbose... 'Sets the verbosity level'")
        )
        .subcommand(
            App::new("stats")
                .about("print memo statistics")
                .arg("<input> 'sets an input file'")
                .arg("-v --verbose... 'Sets the verbosity level'")
        );

    let matches = app.get_matches();

    init_logger(matches.occurrences_of("debug") as u8);

    // --- SUBCOMMAND `list` ---
    
    if let Some(ref matches) = matches.subcommand_matches("list") {
        // read memos from .mr file into database
        // TODO: matches.values_of("input") -> Vec<_>
        if let Some(input) = matches.value_of("input") {
            let verbosity = matches.occurrences_of("verbose") as u8;

            debug!("loading input file '{}'", input);
            let memos = read_from_file(input, true).unwrap();
            debug!("read {} memos", memos.len());

            let mut memo_filter = MemoFilter::new();
            
            // check if a pre-defined filter has been supplied
            if let Some(filter_name) = matches.value_of("filter") {
                debug!("looking for pre-defined filter '{}'", filter_name);
                let mut mf = MemoFilter::new();
                let nf = NodeFilter::default()
                    .with_key(KeyFilter::Equals("mr:filter".into()))
                    .with_value(ValueFilter::Equals(filter_name.into()));
                mf.add(nf);
                if let Some(mql_memo) =
                    memos.iter().filter(|&memo| mf.check(memo)).next()
                {
                    debug!("Resulting filter: {:#?}", mql_memo);
                    if let Some(node) = mql_memo.nodes().filter(|&node| node.key == "mql").next() {
                        debug!("Resulting node: {:#?}", node);
                        let mql = node.value.to_string();
                        debug!("Resulting mql: {}", mql);
                        if let Ok(filter) = parse_mql(mql.as_str()) {
                            debug!("resulting node filter = {:#?}", filter);
                            memo_filter = filter;
                        } else {
                            eprintln!("couldn't parse filter expression!");
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("could not find pre-defined filter '{}'", filter_name);
                    std::process::exit(1);
                }
            }

            // check if a mql filter clause has been supplied
            // any mql condition will be appended to the existing filter
            // it is therefore possible to define both --filter (as base filter)
            // and --mql (as refinement)
            if let Some(mql) = matches.value_of("mql") {
                debug!("mql filter expression is: '{}'", mql);
                if let Ok(mql_filter) = parse_mql(mql) {
                    debug!("resulting mql filter = {:#?}", mql_filter);
                    memo_filter.extend(mql_filter)
                } else {
                    eprintln!("couldn't parse mql filter expression '{}'!", mql);
                }
            }
            
            for memo in memos.iter().filter(|&memo| memo_filter.check(memo)) {
                // always print header
                println!("{}{} {}",
                         "@".red().bold(),
                         memo.collection().red().bold(),
                         memo.title().white().bold()
                );

                match verbosity {
                    1 => {
                        // print only matching nodes
                        // currently, this includes the header node as well
                        
                        for idx in memo_filter.select_indices(&memo) {
                            // skip header node, as it is already printed above
                            // actually, this code relies on an implementation detail, i.e.
                            // that the header node has index 0
                            if idx > 0 {
                                let node = memo.get_by_index(idx).unwrap();
                                println!("{}{} {}",
                                         ".".red(),
                                         node.key.red(),
                                         node.value.to_string().white());
                                for (key, value) in node.attrs() {
                                    println!("{}{} {}",
                                             "+".yellow(),
                                             key.yellow(),
                                             value.to_string().white());
                                }
                            }
                        }
                        println!("");
                    },
                    2 => {
                        // print all nodes
                        for node in memo.data() {
                            println!("{}{} {}",
                                     ".".red(),
                                     node.key.red(),
                                     node.value.to_string().white());

                            for (key, value) in node.attrs() {
                                println!("{}{} {}",
                                         "+".yellow(),
                                         key.yellow(),
                                         value.to_string().white());
                            }
                        }
                        println!("");
                    },
                    _ => {}
                }
            }   
        }        
    }

    // --- SUBCOMMAND `stats` ---
    
    if let Some(ref matches) = matches.subcommand_matches("stats") {
        // read memos from .mr file into database
        if let Some(input) = matches.value_of("input") {
            //let verbosity = matches.occurrences_of("verbose") as u8;

            debug!("loading input file '{}'", input);
            let memos = read_from_file(input, true).unwrap();
            debug!("read {} memos", memos.len());

            let memo_count = memos.len();
            // TODO: implement Memo.len()
            let node_count = memos.iter().fold(0, |acc, m| acc + m.data_count() + 1);

            println!("Statistics for '{}':", input);
            println!("#Memos = {}", memo_count);
            println!("#Nodes = {}", node_count);

            // TODO: we could allow --filter and --mql options
            // and yield a statistic on the filtered nodes
        }
    }
}
