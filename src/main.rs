//! merula is a command-line client for the flat-file, plain-text
//! database format with the same name and the extension `.mr`.
//!
//! DISCLAIMER: This is work-in-progress and not at all ready for
//! production purposes.
//!

#[macro_use] extern crate pest_derive;

#[allow(unused_imports)]
use clap::{App, crate_version, Arg};
#[allow(unused_imports)]
use clap_generate::{generate, generators::Bash};

use std::collections::hash_map::{HashMap};

pub mod memo;
pub mod node;
pub mod value;
pub mod sample;
pub mod parser;
pub mod filter;
pub mod mql;

use memo::{Memo, MemoId};
use node::Node;
use value::{Value, Key};
use filter::{NodeFilter, KeyFilter, ValueFilter, MemoFilter};
use mql::parse_mql;
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
            App::new("test")
                .about("preliminary test")
        )
        .subcommand(
            App::new("test-mql")
                .about("testing mql parser")
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
            let memos = parser::read_from_file(input, true).unwrap();
            debug!("read {} memos", memos.len());

            // check if a mql filter clause has been supplied
            let mut memo_filter = MemoFilter::new();
            if let Some(mql) = matches.value_of("mql") {
                debug!("mql filter expression is: '{}'", mql);
                if let Ok(filter) = parse_mql(mql) {
                    debug!("resulting node filter = {:#?}", filter);
                    memo_filter = filter;
                } else {
                    println!("couldn't parse filter expression!");
                }
            }

            // alternatively, check if a pre-defined filter has been supplied
            if let Some(filter_name) = matches.value_of("filter") {
                debug!("looking for pre-defined filter '{}'", filter_name);
                let mut mf = MemoFilter::new();
                let nf = NodeFilter::new()
                    .with_key(KeyFilter::Equals("mr:filter".into()))
                    .with_value(ValueFilter::Equals(filter_name.into()));
                mf.add_filter(nf);
                if let Some(mql_memo) =
                    memos.iter().filter(|&memo| mf.check_memo(memo)).next()
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
            for memo in memos.iter().filter(|&memo| memo_filter.check_memo(memo)) {
                println!("{}{} {}",
                         "@".red().bold(),
                         memo.collection().red().bold(),
                         memo.title().white().bold()
                );
                if verbosity >= 1 {
                    for node in memo.data() {
                        println!("{}{} {}",
                                 ".".red(),
                                 node.key.red(),
                                 node.value.to_string().white());
                    }
                    println!("");
                }
            }   
        }        
    }

    // --- SUBCOMMAND `test` ---

    if let Some(ref _matches) = matches.subcommand_matches("test") {
        fn section(title: &'static str) {
            println!("\n# {}\n", title);
        }

        section("add some memos");

        let memos = sample::setup_memos();        
                    
        // print out list of memos with one line for each memo
        let digits = (memos.len() as f32).log10().trunc() as usize + 1;
        section("this is a short list of the memos");
        for (n, memo) in memos.iter().enumerate() {
            println!("[{:width$}] @{} {} ({})",
                     n, memo.collection(), memo.title(), memo.data_count(),
                     width=digits);
        }

        // filter out all memos that contain at least one author node
        section("filter all memos with at least an author node");
        for memo in memos.iter().filter(|&m| m.contains_key("author")) {
            println!("@{} {}", memo.collection(), memo.title());
        }

        // filter out all memos from the collection 'character'
        section("filter all memos from 'character' collection");
        for memo in memos.iter().filter(|&m| m.collection() == "character") {
            println!("@{} {}", memo.collection(), memo.title());
        }

        // filter out all memos with a title containing a number
        section("filter all memos with a title containing a number");
        for memo in memos.iter().filter(|&m| m.title().parse::<f32>().is_ok())  {
            println!("@{} {}", memo.collection(), memo.title());
        }

        // filter out all memos with a node value containing 'Bilbo'
        section("filter all memos with a node value containing 'Bilbo'");
        let node_filter = |node: &&Node| node.value.to_string().contains("Bilbo");
        for memo in memos.iter().filter(|&m| m.data().find(node_filter).is_some()) {
            println!("@{} {}", memo.collection(), memo.title());
        }

        // filter out all memos with a node with a boolean value
        section("filter all memos with a node value being a boolean value");
        let node_filter = |node: &&Node| node.value.is_bool();
        for memo in memos.iter().filter(|&m| m.data().find(node_filter).is_some()) {
            println!("@{} {}", memo.collection(), memo.title());
        }
        
        // create index
        section("Create index of memos");

        // move memos into index map (=primary index)
        section("create primary index");
        let mut index: HashMap<MemoId, Memo> = HashMap::new();

        for memo in memos {
            index.insert(memo.id(), memo);
        }
        println!("{:#?}", index);

        // we can now create a secondary index, e.g. by collection
        section("creating secondary index");
        let mut index2: HashMap<Key, Vec<MemoId>>
            = HashMap::new();
        
        for id in index.keys() {
            let collection = index[id].collection();
            index2.entry(collection).or_default().push(id.clone());
        }
        println!("{:#?}", index2);

        // listing all characters
        section("listing all characters by looking up secondary index");
        for id in index2["character"].iter() {
            let memo = &index[&id];
            println!("@{} {}", memo.collection(), memo.title());
        }

        // we could create a third index, e.g. by (collection, title)
        section("creating ternary index");
        let mut index3: HashMap<(Key, String), MemoId>
            = HashMap::new();

        for id in index.keys() {
            let memo = &index[&id];
            index3.insert((memo.collection(), memo.title()), id.clone());
        }
        println!("{:#?}", index3);

    }

    if let Some(ref _matches) = matches.subcommand_matches("test-mql") {
        println!("RESULT:\n{:#?}", mql::parse_mql("hallo~"));
    }
}
