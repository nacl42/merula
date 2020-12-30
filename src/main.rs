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
        println!("testing: add some memos");

        let mut memos: Vec<Memo> = vec!();
        
        let mut memo = Memo::new("book", "The Lord of the Rings");
        memo.push(Node::new("author", "J.R.R. Tolkien"));
        memo.push(Node::new("character", "Bilbo Baggins"));
        memo.push(Node::new("character", "Samweis Gamdschie"));
        
        println!("This is the first memo:");
        println!("{}", memo);
        memos.push(memo);
        
        let mut memo = Memo::new("book", "The Hitchhiker's Guide to the Galaxy");
        memo.push(Node::new("author", "Douglas Adams"));
        memo.push(Node::new("author", "Arthur Dent"));
        memo.push(Node::new("character", "Ford Prefect"));

        println!("\nThis is the second memo:");
        println!("{}", memo);
        memos.push(memo);

        for i in 0..20 {
            memos.push(Memo::new("foo", i));
        }

        // print out list of memos with one line for each memo
        let digits = (memos.len() as f32).log10().trunc() as usize + 1;
        println!("\nThis is a short list of the memos:");
        for (n, memo) in memos.iter().enumerate() {
            println!("[{:width$}] @{} {} ({})",
                     n, memo.group(), memo.title(), memo.data_count(),
                     width=digits);
        }
    }
}
