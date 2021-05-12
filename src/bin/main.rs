//! merula is a command-line client for the flat-file, plain-text
//! database format with the same name and the extension `.mr`.
//!
//! DISCLAIMER: This is work-in-progress and not at all ready for
//! production purposes.
//!

use merula::{
    filter::{NodeFilter, KeyFilter, ValueFilter, MemoFilter},
    parser::read_from_file,
    mql::parse_mql,
    memo::{Memo, NodeType}
};

use regex::{Regex, Captures};
use simplelog::*;
use log::*;
use colored::*;

#[allow(unused_imports)]
use clap::{App, crate_version, Arg, ArgGroup};
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


fn lookup_filter(memos: &Vec<Memo>, filter_name: &str)
                 -> Result<MemoFilter, String>
{
    debug!("looking for pre-defined filter '{}'", filter_name);
    let mut mf = MemoFilter::new();
    let nf = NodeFilter::default()
        .with_key(KeyFilter::Equals("mr:filter".into()))
        .with_value(ValueFilter::Equals(filter_name.into()));
    mf.add(nf);
    
    if let Some(mql_memo) = memos.iter().filter(|&memo| mf.check(memo)).next() {
        debug!("Resulting filter: {:#?}", mql_memo);
        if let Some(node) = mql_memo.nodes().filter(|&node| node.key == "mql").next() {
            debug!("Resulting node: {:#?}", node);
            let mql = node.value.to_string();
            debug!("Resulting mql: {}", mql);
            match parse_mql(mql.as_str()) {
                Ok(filter) => Ok(filter),
                Err(msg) => Err(String::from(msg))
            }
        } else {
            Err(format!("pre-defined filter '{}' found, but it contains no `.mql` node", filter_name))
        }
    } else {
        Err(format!("could not find pre-defined filter '{}'", filter_name))
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
                .arg("--filter=[FILTER] 'load an mql expression from a pre-defined filter'")
                .arg("--mql=[MQL] 'sets a mql expression'")
                .arg("-v --verbose... 'Sets the verbosity level'")
                .arg("--all 'use all memos (default)'")
                .arg("--only-mr 'only internal memos (@mr:xxx)'")
                .arg("--only-user 'only user memos'")
                .group(ArgGroup::new("default-filter")
                       .args(&["all", "only-mr", "only-user"])
                       .multiple(false))
        )
        .subcommand(
            App::new("stats")
                .about("print memo statistics")
                .arg("<input> 'sets an input file'")
                .arg("-v --verbose... 'Sets the verbosity level'")
        )
        .subcommand(
            App::new("export")
                .about("export data using a template")
                .arg("<input> 'sets an input file'")
                .arg("--filter=[FILTER] 'load an mql expression from a pre-defined filter'")
                .arg("--mql=[MQL] 'sets a mql expression'")
                .arg("--template=[TEMPLATE] 'name of the template expression'")
                .arg("--all 'use all memos (default)'")
                .arg("--only-mr 'only internal memos (@mr:xxx)'")
                .arg("--only-user 'only user memos'")
                .group(ArgGroup::new("default-filter")
                       .args(&["all", "only-mr", "only-user"])
                       .multiple(false))
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
            let memos = read_from_file(input).unwrap();
            debug!("read {} memos", memos.len());

            // setup filter
            let mut memo_filter = MemoFilter::new();

            // set default filter
            if matches.is_present("only-mr") {
                memo_filter.add(
                    NodeFilter::default()
                        .with_node_type(NodeType::Header)
                        .with_key(KeyFilter::StartsWith("mr:".into()))
                );
            } else if matches.is_present("only-user") {
                memo_filter.add(
                    NodeFilter::default()
                        .with_node_type(NodeType::Header)
                        .with_key(KeyFilter::Not(
                            Box::new(KeyFilter::StartsWith("mr:".into()))))
                );
            }
            
            // check if a pre-defined filter has been supplied
            if let Some(filter_name) = matches.value_of("filter") {
                match lookup_filter(&memos, filter_name) {
                    Ok(mf) => memo_filter = mf,
                    Err(msg) => {
                        eprintln!("{}", msg);
                        std::process::exit(1);
                    }
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

    // --- SUBCOMMAND `export` ---
    
    if let Some(ref matches) = matches.subcommand_matches("export") {
        // read memos from .mr file into database
        // TODO: matches.values_of("input") -> Vec<_>
        if let Some(input) = matches.value_of("input") {

            debug!("loading input file '{}'", input);
            let memos = read_from_file(input).unwrap();
            debug!("read {} memos", memos.len());

            // check if a pre-defined template has been supplied
            if let Some(template_name) = matches.value_of("template") {
                debug!("looking for pre-defined template '{}'", template_name);
                let mut mf = MemoFilter::new();
                let nf = NodeFilter::default()
                    .with_key(KeyFilter::Equals("mr:template".into()))
                    .with_value(ValueFilter::Equals(template_name.into()));
                mf.add(nf);
                if let Some(tpl_memo) =
                    memos.iter().filter(|&memo| mf.check(memo)).next()
                {
                    //debug!("Resulting template: {:#?}", tpl_memo);
                    // get header if available
                    if let Some(header) = tpl_memo.get("header") {
                        println!("{}", header.value);
                    }
                    // get body if available
                    if let Some(body) = tpl_memo.get("body") {
                        let tpl = &body.value.to_string();
                        debug!("template text = {}", tpl);
                        let re: Regex = Regex::new("\\{(.*?)\\}").unwrap();

                        // setup filter
                        let mut memo_filter = MemoFilter::new();
                        
                        // set default filter
                        if matches.is_present("only-mr") {
                            memo_filter.add(
                                NodeFilter::default()
                                    .with_node_type(NodeType::Header)
                                    .with_key(KeyFilter::StartsWith("mr:".into()))
                            );
                        } else if matches.is_present("only-user") {
                            memo_filter.add(
                                NodeFilter::default()
                                    .with_node_type(NodeType::Header)
                                    .with_key(KeyFilter::Not(
                                        Box::new(KeyFilter::StartsWith("mr:".into()))))
                            );
                        }
            
                        // check if a pre-defined filter has been supplied
                        if let Some(filter_name) = matches.value_of("filter") {
                            match lookup_filter(&memos, filter_name) {
                                Ok(mf) => memo_filter = mf,
                                Err(msg) => {
                                    eprintln!("{}", msg);
                                    std::process::exit(1);
                                }
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
                            let result = re.replace_all(tpl, |caps: &Captures| {
                                if let Some(node) = memo.get(&caps[1]) {
                                    format!("{}", node.value)
                                } else {
                                    String::from(&caps[0])
                                }
                            });
                            println!("{}", result);
                        }
                    }
                } else {
                    error!("template '{}' not found", template_name);
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
            let memos = read_from_file(input).unwrap();
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
