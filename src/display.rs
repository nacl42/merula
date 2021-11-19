
use crate::Memo;

use colored::*;

pub fn print_header(memo: &Memo) {
    println!(
        "{}{} {}",
        "@".red().bold(),
        memo.collection().red().bold(),
        memo.title().white().bold()
    );
}

pub fn print_data_nodes(memo: &Memo) {
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
}
