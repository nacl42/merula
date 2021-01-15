//! merula is a command-line client for the flat-file, plain-text
//! database format with the same name and the extension `.mr`.
//!
//! DISCLAIMER: This is work-in-progress and not at all ready for
//! production purposes.
//!

// TODO: filter by value ("url~github", "amu>5")
// TODO: transform result set by applying a template to each resulting Memo
// TODO: list all available keys for a result set (--keys)
// TODO: filter by collection ("@app")
// TODO: filter by data node (".url")


#[macro_use] extern crate pest_derive;
#[macro_use] extern crate lazy_static;

#[allow(unused_imports)]
use clap::{App, crate_version, Arg};
#[allow(unused_imports)]
use clap_generate::{generate, generators::Bash};

use std::collections::hash_map::{HashMap, DefaultHasher};
use std::hash::{Hash, Hasher};

pub mod memo;
pub mod node;
pub mod value;
pub mod sample;
pub mod parser;
pub mod filter;

use memo::{Memo, MemoId};
use node::Node;
use value::{Value, Key};
use filter::NodeFilter;

fn main() {
    let app = App::new("merula")
        .version(crate_version!())
        .author("nacl42 <code@sreblov.de>")
        .about("simple cli frontend to access merula files (.mr)")
        .arg("-v --verbose... 'Sets the verbosity level'")
        .subcommand(
            App::new("list")
                .about("list memos")
                .arg("<input> 'sets an input file'")
                .arg("--filter=[FILTER] 'sets a filter condition'")
        )
        .subcommand(
            App::new("test")
                .about("preliminary test")
        );

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("list") {
        // read memos from .mr file into database
        // TODO: let mut db = Database::new()
        if let Some(input) = matches.value_of("input") {
            //init.logger(matches.occurences_of("verbose") as u8);
            println!("loading input file '{}'", input);
            let memos = parser::read_from_file(input, true).unwrap();
            // TODO: db.memos.extend(memos)
            println!("==> {} memos", memos.len());

            // check if a filter clause has been supplied
            if let Some(filter) = matches.value_of("filter") {
                println!("filter nodes with key '{}'", filter);
                // TODO: parse filter expression
                // distinguish between NodeFilter and MemoFilter ... ?
                let f = filter::Comparison::Contains(filter.to_string());
                //let f = filter::KeyFilter::new(filter);
                for memo in memos.iter().filter(
                    |&memo| memo.nodes().find(f.predicate()).is_some()
                ) {
                    println!("@{} {}", memo.collection(), memo.title())
                }
                
            } else {
                for memo in memos {
                    println!("@{} {}", memo.collection(), memo.title())
                }
            }
        }        
    }

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
}
