//! Parser to read in a complete file.
//!
//! The parser uses the `pest` crate.
//!

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "memo.pest"]
pub struct MemoParser;

use std::fs;
use log::*;

use crate::{Memo, Node, Value};
use std::path::{Path, PathBuf};

// TODO:
// There are multiple issues with the code below.
// - We are mixing filenames as &str with Path and PathBuf.
// - We could accept anything that can be converted to a Path
// - nested includes are not possible, instead we are simply limiting
//   to one include

fn determine_include_path(current_filename: &'_ str, include_filename: &'_ str) -> PathBuf {
    let include_path = Path::new(current_filename);
    if include_path.is_relative() {
        // if given filename is relative, create complete
        // file path from the current working directory
        let cwd = Path::new(current_filename);
        cwd.with_file_name(include_filename)
    } else {
        // if given filename is absolute, then use it
        include_path.to_path_buf()
    }
}

pub fn read_from_file(filename: &'_ str, drop_first: bool)
                      -> Result<Vec<Memo>, ()>
{
    read_from_file_internal(filename, drop_first, &mut vec!())
}

fn read_from_file_internal(filename: &'_ str, drop_first: bool,
                           include_path_trail: &mut Vec<PathBuf>)
                           -> Result<Vec<Memo>, ()>
{
    debug!("reading file {}", filename);
    let unparsed_file = fs::read_to_string(filename)
        .expect("cannot read mr file");

    include_path_trail.push(Path::new(filename).to_path_buf());
    
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
                if key == "mr:include" {
                    let include_path = determine_include_path(&filename, value);
                    debug!("include path is '{:#?}'", include_path.to_str());
                    // we could have an include path trail and check if the include
                    // is in there. OR even simpler, we could just allow one include :-)
                    debug!("trying to include {}", include_path.to_str().unwrap());
                    if include_path_trail.len() < 2 {
                        let included_memos = read_from_file_internal(include_path.to_str().unwrap(), false, include_path_trail).unwrap();
                        info!("included {} memos from included file '{}'", included_memos.len(), include_path.to_str().unwrap());
                        memos.extend(included_memos);
                    } else {
                        eprintln!("merula currently does not supporting nested includes");
                    }
                    
                };
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

pub fn rule_ml_data_node_nosep(pair: Pair<Rule>) -> Result<Node, ()> {
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let value = inner.next().unwrap().as_str().trim();

    Ok(Node::new(key, value))
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

    #[test]
    #[ignore]
    fn test_rule_data_eof() {
        // example 1
        let input = r#".cities<<EOF
Frankfurt
Berlin
Paris
EOF
.foo"#;
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        assert_eq!(result.is_ok(), true);

        let pair = result.unwrap().next().unwrap();
        let mut inner = pair.into_inner();

        let key = inner.next().unwrap().as_str();
        assert_eq!(key, "cities");

        //let inner_value = inner.next().unwrap().into_inner();
        //let pair = inner_value.unwrap().next().unwrap();

        //let value = inner.next().unwrap().as_str().trim();
        //
        assert_eq!(value, "Hi, you!\nThis is a very interesting book, you know!");        
    }
    
    #[test]
    fn test_rule_data_node_ml() {
        // example 1
        let input = r#".doc Hi, you!
This is a very interesting book, you know!
.foo"#;
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        assert_eq!(result.is_ok(), true);

        let pair = result.unwrap().next().unwrap();
        let mut inner = pair.into_inner();

        let key = inner.next().unwrap().as_str();
        assert_eq!(key, "doc");

        let value = inner.next().unwrap().as_str().trim();
        assert_eq!(value, "Hi, you!\nThis is a very interesting book, you know!");

        // example 2: ignore first newline
        let input = r#".ingredients
Rice
Mushrooms
Soy Sauce
.foo"#;
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        assert_eq!(result.is_ok(), true);

        let pair = result.unwrap().next().unwrap();
        let mut inner = pair.into_inner();

        let key = inner.next().unwrap().as_str();
        assert_eq!(key, "ingredients");

        let value = inner.next().unwrap().as_str().trim();
        assert_eq!(value, "Rice\nMushrooms\nSoy Sauce");

        // example 3: single line
        let input = ".character Sam Gamdschie";

        let result = MemoParser::parse(Rule::data_node_ml, &input);
        assert_eq!(result.is_ok(), true);

        let pair = result.unwrap().next().unwrap();
        let mut inner = pair.into_inner();

        let key = inner.next().unwrap().as_str();
        assert_eq!(key, "character");

        let value = inner.next().unwrap().as_str().trim();
        assert_eq!(value, "Sam Gamdschie");
    }
    
    #[test]
    #[ignore]
    fn parse_memo_1() {
        let input = "@book The Lord of the Rings";
        let output = Memo::new("book", "The Lord of the Rings");
        let result = MemoParser::parse(Rule::header, &input).unwrap();
        //assert_eq!(result, output);
        // TODO: read from string or string buffer        
    }
}
