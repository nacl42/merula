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
                let mut text = inner_rules.next().unwrap().as_str().to_string();
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
    let _eof = inner.next().unwrap().as_str();
    let value_eof = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value_eof))
}

pub fn rule_header_node(pair: Pair<Rule>) -> Result<Node, ()> {
    // header_node = @{ header_node_eof | header_node_ml }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::header_node_eof => rule_header_node_eof(inner),
        Rule::header_node_ml => rule_header_node_ml(inner),
        _ => Err(())
    }
}

pub fn rule_data_multinode_ml(pair: Pair<Rule>) -> Result<Vec<Node>, ()> {
    // data_multinode_ml = { "." ~ key ~ sep ~ value_ml }
    let mut nodes = Vec::new();
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let sep = match inner.next().unwrap().as_str() {
        "|" => "\n",
        x => x
    };
    let values = inner.next().unwrap().as_str();
    // split value by given separator, each value is trimmed
    for value in values.split(sep) {
        let value = value.trim();
        if value.len() > 0 {
            nodes.push(Node::new(key, value.trim()));
        }
    }
    Ok(nodes)
}

pub fn rule_data_multinode_eof(pair: Pair<Rule>) -> Result<Vec<Node>, ()> {
    // data_multinode_eof = { "." ~ key ~ sep ~ "<<" ~ PUSH(eof) ~ NEWLINE ~ value_eof ~ POP }
    let mut nodes = Vec::new();
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let sep = match inner.next().unwrap().as_str() {
        "|" => "\n",
        x => x
    };
    let _eof = inner.next().unwrap().as_str();
    let values = inner.next().unwrap().as_str().trim();
    // split value by given separator, each value is trimmed
    for value in values.split(sep) {
        let value = value.trim();
        if value.len() > 0 {
            nodes.push(Node::new(key, value.trim()));
        }
    }
    Ok(nodes)
}



pub fn rule_data_node_ml(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node_ml = { "." ~ key ~ value_ml }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let value = inner.next().unwrap().as_str().trim();
    let mut node = Node::new(key, value);
    for attr in inner {
        let mut attr_inner = attr.into_inner();
        let attr_key = attr_inner.next().unwrap().as_str();
        let attr_value = attr_inner.next().unwrap().as_str();
        node.attrs.insert(attr_key.into(), attr_value.into());
    }
    Ok(node)
}

pub fn rule_data_node_eof(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node_eof = { "." ~ key ~ "<<" ~ PUSH(eof) ~ NEWLINE ~ value_eof ~ POP }
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str();
    let _eof = inner.next().unwrap().as_str();
    let value_eof = inner.next().unwrap().as_str().trim();
    Ok(Node::new(key, value_eof))
}

pub fn rule_data_node(pair: Pair<Rule>) -> Result<Node, ()> {
    // data_node = @{ data_node_eof | data_node_ml }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::data_node_eof => rule_data_node_eof(inner),
        Rule::data_node_ml => rule_data_node_ml(inner),
        _ => Err(())
    }
}

pub fn rule_data_multinode(pair: Pair<Rule>) -> Result<Vec<Node>, ()> {
    // data_multinode = { data_multinode_eof | data_multinode_ml }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::data_multinode_eof => rule_data_multinode_eof(inner),
        Rule::data_multinode_ml => rule_data_multinode_ml(inner),
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
            match data_pair.as_rule() {
                Rule::data_node => {
                    memo.push(rule_data_node(data_pair).unwrap());
                }
                Rule::data_multinode => {
                    for node in rule_data_multinode(data_pair).unwrap() {
                        memo.push(node)
                    }
                },
                _ => {}
            }
        }
        Ok(memo)
    } else {
        Err(())
    }
}

pub fn rule_memos(pair: Pair<Rule>) -> Result<Vec<Memo>, ()> {
    // memos = { (comment | memo | NEWLINE)* }
    let mut memos = Vec::<Memo>::new();
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::memo => {
                memos.push(rule_memo(token)?) 
            },
            _ => {} // ignore silently
        }
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

    #[test]
    fn test_fn_rule_data_node_eof() {
        let input = ".color<<EOF\nblue\nEOF";
        let result = MemoParser::parse(Rule::data_node_eof, &input);
        let node = rule_data_node_eof(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));        
    }

    #[test]
    fn test_fn_rule_data_node() {
        let input = ".color blue";
        let result = MemoParser::parse(Rule::data_node, &input);
        let node = rule_data_node(result.unwrap().next().unwrap());
        assert_eq!(node, Ok(Node::new("color", "blue")));
    }

    #[test]
    fn test_fn_rule_data_multinode_ml() {
        let input = ".color, blue, red";
        let result = MemoParser::parse(Rule::data_multinode_ml, &input);
        let nodes = rule_data_multinode_ml(result.unwrap().next().unwrap());
        let expected = vec!(Node::new("color", "blue"), Node::new("color", "red"));
        assert_eq!(nodes, Ok(expected));

        let input = ".color|\nblue\nred";
        let result = MemoParser::parse(Rule::data_multinode_ml, &input);
        let nodes = rule_data_multinode_ml(result.unwrap().next().unwrap());
        let expected = vec!(Node::new("color", "blue"), Node::new("color", "red"));
        assert_eq!(nodes, Ok(expected));
    }

    #[test]
    fn test_fn_rule_data_multinode_eof() {
        let input = ".color,<<EOF\nblue, red\nEOF";
        let result = MemoParser::parse(Rule::data_multinode_eof, &input);
        let nodes = rule_data_multinode_eof(result.unwrap().next().unwrap());
        let expected = vec!(Node::new("color", "blue"), Node::new("color", "red"));
        assert_eq!(nodes, Ok(expected));

        let input = ".color|<<EOF\nblue\nred\nEOF";
        let result = MemoParser::parse(Rule::data_multinode_eof, &input);
        let nodes = rule_data_multinode_eof(result.unwrap().next().unwrap());
        let expected = vec!(Node::new("color", "blue"), Node::new("color", "red"));
        assert_eq!(nodes, Ok(expected));
    }

    #[test]
    fn test_fn_rule_data_multinode() {
        // TODO
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
        
        let input = ".colors blue\nred\n+tag foo";
        let result = MemoParser::parse(Rule::data_node_ml, &input);
        let node = rule_data_node_ml(result.unwrap().next().unwrap());
        let expected = Node::new("colors", "blue\nred").set("tag", "foo");
        assert_eq!(node, Ok(expected));
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
        let expect = Memo::new("book", "The Lord of the Rings");
        assert_eq!(memo, Ok(expect));

        let input = "@book The Lord of the Rings\n.author Tolkien";
        let result = MemoParser::parse(Rule::memo, &input);
        let memo = rule_memo(result.unwrap().next().unwrap());
        let expect = Memo::new("book", "The Lord of the Rings")
            .with(("author", "Tolkien"));
        assert_eq!(memo, Ok(expect));

        let input = "@book The Lord of the Rings\n.author Tolkien\n.character, Frodo,Samweis";
        let result = MemoParser::parse(Rule::memo, &input);
        let memo = rule_memo(result.unwrap().next().unwrap());
        let expect = Memo::new("book", "The Lord of the Rings")
            .with(("author", "Tolkien"))
            .with(("character", "Frodo"))
            .with(("character", "Samweis"));
        assert_eq!(memo, Ok(expect));
    }

    #[test]
    fn test_fn_rule_memos() {
        let input = "@book The Lord of the Rings\n@book The Hobbit";
        let result = MemoParser::parse(Rule::memos, &input);
        let memos = rule_memos(result.unwrap().next().unwrap());
        let expect = vec!(Memo::new("book", "The Lord of the Rings"),
                          Memo::new("book", "The Hobbit"));
        assert_eq!(memos, Ok(expect));

        let input = "@book The Lord of the Rings\n\n\n@book The Hobbit";
        let result = MemoParser::parse(Rule::memos, &input);
        let memos = rule_memos(result.unwrap().next().unwrap());
        let expect = vec!(Memo::new("book", "The Lord of the Rings"),
                          Memo::new("book", "The Hobbit"));
        assert_eq!(memos, Ok(expect));

        let input = "@book The Lord of the Rings\n.author Tolkien\n\n\n@book The Hobbit";
        let result = MemoParser::parse(Rule::memos, &input);
        let memos = rule_memos(result.unwrap().next().unwrap());
        let expect = vec!(Memo::new("book", "The Lord of the Rings")
                          .with(("author", "Tolkien")),
                          Memo::new("book", "The Hobbit"));
        assert_eq!(memos, Ok(expect));

        let input = "# a sample file\n@book The Lord of the Rings\n.author Tolkien\n\n\n@book The Hobbit";
        let result = MemoParser::parse(Rule::memos, &input);
        let memos = rule_memos(result.unwrap().next().unwrap());
        let expect = vec!(Memo::new("book", "The Lord of the Rings")
                          .with(("author", "Tolkien")),
                          Memo::new("book", "The Hobbit"));
        assert_eq!(memos, Ok(expect));
        
        let input = "# a sample file\n@book The Lord of the Rings\n.author Tolkien\n+source Wikipedia\n\n@book The Hobbit";
        let result = MemoParser::parse(Rule::memos, &input);
        let memos = rule_memos(result.unwrap().next().unwrap());
        let expect = vec!(Memo::new("book", "The Lord of the Rings")
                          .with(("author", "Tolkien"))
                          .with_attr("source", "Wikipedia"),
                          Memo::new("book", "The Hobbit"));
        assert_eq!(memos, Ok(expect));

    }
}
