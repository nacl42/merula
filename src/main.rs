#[allow(unused_imports)]
use clap::{App, crate_version, Arg};
#[allow(unused_imports)]
use clap_generate::{generate, generators::Bash};

pub mod memo;
pub mod value;

use memo::{Memo, Item};

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

        let mut memo = Memo::new("book", "The Lord of the Rings");
        memo.push(Item::new("author", "J.R.R. Tolkien"));
        memo.push(Item::new("character", "Bilbo Baggins"));
        memo.push(Item::new("character", "Samweis Gamdschie"));
        
        println!("This is the first memo:");
        println!("{}", memo);

        let mut memo = Memo::new("book", "The Hitchhiker's Guide to the Galaxy");
        memo.push(Item::new("author", "Douglas Adams"));
        memo.push(Item::new("author", "Arthur Dent"));
        memo.push(Item::new("character", "Ford Prefect"));

        println!("\nThis is the second memo:");
        println!("{}", memo);
    }
}
