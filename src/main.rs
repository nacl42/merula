//! merula is a command-line client for the flat-file, plain-text
//! database format with the same name and the extension `.mr`.
//!
//! DISCLAIMER: This is work-in-progress and not at all ready for
//! production purposes.
//!

#[allow(unused_imports)]
use clap::{App, crate_version, Arg};
#[allow(unused_imports)]
use clap_generate::{generate, generators::Bash};

pub mod memo;
pub mod node;
pub mod value;

use memo::Memo;
use node::Node;

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
        )
        .subcommand(
            App::new("test")
                .about("preliminary test")
        );

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("list") {
        // read memos from .mr file into database
        // TODO
        if let Some(input) = matches.value_of("input") {
            //init.logger(matches.occurences_of("verbose") as u8);
            println!("pretending to load input file '{}'", input);
        }        
    }

    if let Some(ref _matches) = matches.subcommand_matches("test") {
        fn section(title: &'static str) {
            println!("\n# {}\n", title);
        }

        section("add some memos");

        let mut memos: Vec<Memo> = vec!();
        
        let mut memo = Memo::new("book", "The Lord of the Rings");
        memo.push(("author", "J.R.R. Tolkien"));
        memo.push(("character", "Bilbo Baggins"));
        memo.push(("character", "Samweis Gamdschie"));

        section("first memo");
        println!("{}", memo);
        memos.push(memo);
        
        let memo = Memo::new("book", "The Hitchhiker's Guide to the Galaxy")
            .with(("author", "Douglas Adams"))
            .with(("author", "Arthur Dent"))
            .with(("character", "Ford Prefect"));
        
        section("second memo");
        println!("{}", memo);
        memos.push(memo);

        let mut memo = Memo::new("character", "Bilbo Baggins");
        memo.push(("class", "hobbit"));
        memo.push(("friend-of", "Samweis Gamdschie"));
        memo.push(("is-hobbit", true));
        memos.push(memo);

        let memo = Memo::new("character", "Samweis Gamdschie")
            .with(("is-hobbit", true));
        memos.push(memo);
        
        for i in 0..8 {
            let mut memo = Memo::new("foo", i);
            if i == 5 {
                memo.push(("author", "foobar"));
            }
            memos.push(memo);
        }


        // print out list of memos with one line for each memo
        let digits = (memos.len() as f32).log10().trunc() as usize + 1;
        section("this is a short list of the memos");
        for (n, memo) in memos.iter().enumerate() {
            println!("[{:width$}] @{} {} ({})",
                     n, memo.group(), memo.title(), memo.data_count(),
                     width=digits);
        }

        // filter out all memos that contain at least one author node
        section("filter all memos with at least an author node");
        for memo in memos.iter().filter(|&m| m.contains_key("author")) {
            println!("@{} {}", memo.group(), memo.title());
        }

        // filter out all memos from the group 'character'
        section("filter all memos from 'character' group");
        for memo in memos.iter().filter(|&m| m.group() == "character") {
            println!("@{} {}", memo.group(), memo.title());
        }

        // filter out all memos with a title containing a number
        section("filter all memos with a title containing a number");
        for memo in memos.iter().filter(|&m| m.title().parse::<f32>().is_ok())  {
            println!("@{} {}", memo.group(), memo.title());
        }

        // filter out all memos with a node value containing 'Bilbo'
        section("filter all memos with a node value containing 'Bilbo'");
        let node_filter = |node: &&Node| node.value.to_string().contains("Bilbo");
        for memo in memos.iter().filter(|&m| m.data().find(node_filter).is_some()) {
            println!("@{} {}", memo.group(), memo.title());
        }

        // filter out all memos with a node with a boolean value
        section("filter all memos with a node value being a boolean value");
        let node_filter = |node: &&Node| node.value.is_bool();
        for memo in memos.iter().filter(|&m| m.data().find(node_filter).is_some()) {
            println!("@{} {}", memo.group(), memo.title());
        }
    }
}
