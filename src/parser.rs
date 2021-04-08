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


pub fn rule_data_node_ml(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node_ml = { "." ~ key ~ value_ml }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let value = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value))
}

pub fn rule_data_node_eof(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node_eof = { "." ~ key ~ "<<" ~ PUSH(eof) ~ NEWLINE ~ value_eof ~ POP }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let eof = inner.next().unwrap().as_str();
    let value_eof = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value_eof))
}

pub fn rule_header_node_ml(pair: Pair<Rule>) -> Result<Node, ()> {
    // header_node_ml = { "@" ~ key ~ value_ml }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let value = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value))
}

pub fn rule_header_node_eof(pair: Pair<Rule>) -> Result<Node, ()> {
    // header_node_eof = { "@" ~ key ~ "<<" ~ PUSH(eof) ~ NEWLINE ~ value_eof ~ POP }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let eof = inner.next().unwrap().as_str();
    let value_eof = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value_eof))
}

pub fn rule_header_node(pair: Pair<Rule>) -> Result<Node, ()> {
    // header_node = @{ header_node_eof | header_node_ml }
    let mut inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::header_node_eof => rule_header_node_eof(inner),
        Rule::header_node_ml => rule_header_node_ml(inner),
        _ => Err(())
    }
}

pub fn rule_data_node(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node = @{ data_node_eof | data_node_ml }
    match pair.as_rule() {
        Rule::data_node_eof => rule_data_node_eof(pair),
        Rule::data_node_ml => rule_data_node_ml(pair),
        _ => Err(())
    }
}

pub fn rule_memo(pair: Pair<Rule>) -> Result<Memo, ()> {
    // memo = { header_node ~ (NEWLINE ~ data_node)* }
    let mut inner = pair.clone().into_inner();
    let p = inner.next().unwrap();
    
    if let Ok(header) = rule_header_node(p) {
        let mut memo = Memo::new(header.key, header.value);
        for data_pair in inner {
            if let Ok(data) = rule_data_node(data_pair) {
                memo.push(data);
            }
        }
        Ok(memo)
    } else {
        Err(())
    }
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
    fn test_fn_rule_data_node_eof() {
        let input = ".color<<EOF\nblue\nEOF";
        let result = MemoParser::parse(Rule::data_node_eof, &input);
        let node = rule_data_node_eof(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));        
    }

    #[test]
    fn test_fn_rule_header_node_eof() {
        let input = "@color<<EOF\nblue\nEOF";
        let result = MemoParser::parse(Rule::header_node_eof, &input);
        let node = rule_header_node_eof(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));        
    }

    #[test]
    fn test_fn_rule_data_node_ml() {
        let input = r#".color blue"#;
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        let node = rule_data_node_ml(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));

        let input = ".colors blue\nred";
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        let node = rule_data_node_ml(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("colors", "blue\nred")));

    }

    #[test]
    fn test_fn_rule_header_node_ml() {
        let input = r#"@color blue"#;
        let result = MemoParser::parse(Rule::header_node_ml, &input);
        let node = rule_header_node_ml(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));

        let input = "@colors blue\nred";
        let result = MemoParser::parse(Rule::header_node_ml, &input);
        let node = rule_header_node_ml(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("colors", "blue\nred")));
    }


    #[test]
    fn test_fn_rule_memo() {
        let input = "@book The Lord of the Rings";
        let result = MemoParser::parse(Rule::memo, &input);
        let memo = rule_memo(result.unwrap().next().unwrap());
        assert_eq!(
            memo,
            Ok(
                Memo::new("book", "The Lord of the Rings")
                    .with(("author", "Tolkien"))
            )
        );

        // TODO: Memo does not compare data nodes properly!!!

        let input = "@book The Lord of the Rings\n.author Tolkien";
        let result = MemoParser::parse(Rule::memo, &input);
        let memo = rule_memo(result.unwrap().next().unwrap());
        assert_eq!(
            memo,
            Ok(
                Memo::new("book", "The Lord of the Rings")
                    .with(("author", "Tolkien"))
            )
        );
        
    }
}
