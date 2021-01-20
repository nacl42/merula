//! Parser to read in a complete file.
//!
//! The parser uses the `pest` crate.
//!

use pest::Parser;

#[derive(Parser)]
#[grammar = "memo.pest"]
pub struct MemoParser;

use std::fs;
use log::*;

use crate::{Memo, Node, Value};


pub fn read_from_file(filename: &'_ str, drop_first: bool)
                      -> Result<Vec<Memo>, ()>
{

    let unparsed_file = fs::read_to_string(filename)
        .expect("cannot read mr file");

    let file = MemoParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();

    let mut memos = Vec::<Memo>::new();
    let mut memo: Option<Memo> = Some(Memo::new("mr:default", ""));
    
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::comment => {
                // comments are currently ignored
                let mut inner_rules = line.into_inner();
                let value = inner_rules.next().unwrap().as_str();
                debug!("# {}", value);
            }
            Rule::header => {
                // a header rule starts a new Memo
                let mut inner_rules = line.into_inner();
                let key = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();
                let new_memo = Memo::new(key, value);
                memo.take().map(|m| memos.push(m));
                memo = Some(new_memo);
                debug!("@{} {}", key, value);
            }
            Rule::multivalue_node => {
                // a multivalue node (requires that there is a current node)
                let mut inner_rules = line.into_inner();
                let key = inner_rules.next().unwrap().as_str();
                let sep = inner_rules.next().unwrap().as_str();
                let values = inner_rules.next().unwrap().as_str();
                debug!(".{}<<{}\n{}\n{}", key, sep, values, sep);
                // split value by given separator
                // each value is trimmed
                for value in values.split(sep) {
                    memo.as_mut().map(|m| m.push(Node::new(key, value.trim())));
                }
            }
            Rule::node => {
                // a simple node with a single value
                let mut inner_rules = line.into_inner();
                let key = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();
                debug!(".{} {}", key, value);
                memo.as_mut().map(|m| m.push(Node::new(key, value)));
            }
            Rule::attr => {
                // an attribute is set for the last node
                let mut inner_rules = line.into_inner();
                let key = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();
                memo.as_mut().map(
                    |m| m.last_mut().attrs.insert(key.into(), value.into())
                );
                debug!("+{} {}", key, value);                
            }
            Rule::multiline_node => {
                // a multiline node must be set as Value::MultiLine
                let mut inner_rules = line.into_inner();
                let key = inner_rules.next().unwrap().as_str();
                let eof = inner_rules.next().unwrap().as_str();
                let mut text = inner_rules.next().unwrap().as_str()
                    .to_string();
                // pop() removes the last newline, as it is included
                // in the current parsing rule from memo.pest
                if text.len() > 0 {
                    text.pop();
                }
                let value = Value::MultiLineText(text.into(), eof.into());
                debug!(".{}<<{}\n{}{}", key, eof, value, eof);
                memo.as_mut().map(|m| m.push(Node::new(key, value)));
            }
            _ => {}
        }
    }

    memo.take().map(|m| memos.push(m));

    if drop_first {
        memos.remove(0);
    }

    Ok(memos)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header() {
        // testing is very limited, just check if it is valid or not
        let result = MemoParser::parse(Rule::header, &"@foo bar");
        assert!(result.is_ok());

        let result = MemoParser::parse(Rule::header, &"@foo");
        assert!(result.is_ok());

        let result = MemoParser::parse(Rule::header, &".foo");
        assert!(result.is_err());
    }
}
