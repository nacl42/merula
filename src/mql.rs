//! Parser to read mql expressions

use pest::Parser;
use pest::iterators::Pair;

use crate::filter::{
    MemoFilter,
    NodeFilter,
    IndexFilter, KeyFilter, ValueFilter
};

use crate::memo::NodeType;

use log::*;

#[derive(Parser)]
#[grammar = "mql.pest"]
pub struct MqlParser;

type ParseResult<T> = Result<T, &'static str>;


pub fn parse_mql(input: &'_ str) -> ParseResult<MemoFilter>
{
    if let Ok(pairs) = MqlParser::parse(Rule::mql, &input) {
        let mut filter = MemoFilter::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::condition => {
                    let condition = parse_condition(pair).unwrap();
                    filter.add(condition);
                },                
                _ => {}
            }
        }
        Ok(filter)
    } else {
        Err("unsuccessful parse")
    }
}

fn parse_condition(pair: Pair<Rule>) -> ParseResult<NodeFilter>
{
    let mut nf = NodeFilter::default();

    let mut operator: Option<&str> = None;
    let mut value: Option<&str> = None;
    
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::prefix => {
                nf.node_type = match pair.as_str() {
                    "@" => NodeType::Header,
                    "." => NodeType::Data,
                    _ => NodeType::Any
                };
            },
            Rule::key => {
                nf.key = KeyFilter::Equals(pair.as_str().into());
            },
            Rule::index_single => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::index => {
                            if let Ok(index) = pair.as_str().parse::<usize>() {
                                nf.index = IndexFilter::Single(index)
                            }
                        },
                        _ => {},
                    }
                }
            },
            Rule::index_range => {
                let mut index_from: Option<usize> = None;
                let mut index_to: Option<usize> = None;
                
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::index_from => {
                            index_from = Some(pair.as_str().parse::<usize>().unwrap());
                            debug!("index from: {:?}", index_from);
                        },
                        Rule::index_to =>
                            index_to = Some(pair.as_str().parse::<usize>().unwrap()),
                        _ => { debug!("unknown rule: {:?}", pair.as_rule()); }
                    }
                }

                if let (Some(from), Some(to)) = (index_from, index_to) {
                    // TODO: we could check that from <= to and
                    // return an Error instead
                    nf.index = IndexFilter::Range(from, to);
                    debug!("setting index filter to {:?}", nf.index);
                } else {
                    debug!("not setting filter ({:?}, {:?})", index_from, index_to);
                }
            },
            // TODO: merge operator/value into one expression
            // and create it directly in the rule
            Rule::operator => operator = Some(pair.as_str()),
            Rule::value => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::inner_value | Rule::unquoted_value =>
                            value = Some(pair.as_str()),
                        _ => {},                           
                    }
                }
            },
            _ => { warn!("unhandled mql rule '{:?}'", pair.as_rule())}
        }
    }

    // construct NodeFilter object

    debug!("operator = {:?}", operator);
    debug!("value = {:?}", value);
    let value_filter = match (operator, value) {
        (Some("="), Some(s)) => ValueFilter::Equals(s.into()),
        (Some("~"), Some(s)) => ValueFilter::Contains(s.into()),
        (Some(">"), Some(s)) => {
            match s.parse::<f32>() {
                Ok(value_f32) => ValueFilter::MoreThan(value_f32),
                _ => ValueFilter::Any
            }
        },
        (Some("<"), Some(s)) => {
            match s.parse::<f32>() {
                Ok(value_f32) => ValueFilter::LessThan(value_f32),
                _ => ValueFilter::Any
            }
        },
        (Some(">="), Some(s)) => {
            match s.parse::<f32>() {
                Ok(value_f32) => ValueFilter::AtLeast(value_f32),
                _ => ValueFilter::Any
            }
        },
        (Some("<="), Some(s)) => {
            match s.parse::<f32>() {
                Ok(value_f32) => ValueFilter::AtMost(value_f32),
                _ => ValueFilter::Any
            }
        },
        _ => ValueFilter::Any
    };
    debug!("value-filter = {:?}", value_filter);

    nf.value = value_filter;
    debug!("filter: \n{:#?}", nf); // TESTING

    Ok(nf)
}



#[cfg(test)]
mod tests {
    use crate::mql::*;

    // Try to match all given input item `ok` with the given `rule`.
    // Return a vector of all input strings, that could not be matched
    // or None if all items matched.
    fn check_ok<'a>(rule: Rule, ok: &[&'a str])
                    -> Option<Vec<String>>
    {
        let output = ok.iter()
            .filter(|&item| MqlParser::parse(rule, &item).is_err())
            .map(|&item| item.to_string())
            .collect::<Vec<String>>();
        
        if output.len() > 0 {
            Some(output)
        } else {
            None
        }
    }

    // Try to match all given input items `err` with the given `rule`.
    // Return a vector of all input strings, that matched (even thoud
    // we assumed they would not) or None if no item matched.
    fn check_err<'a>(rule: Rule, err: &[&'a str])
                     -> Option<Vec<String>>
    {
        let output = err.iter()
            .filter(|&item| MqlParser::parse(rule, &item).is_ok())
            .map(|&item| item.to_string())
            .collect::<Vec<String>>();
        
        if output.len() > 0 {
            Some(output)
        } else {
            None
        }
    }

    // Combine check_ok and check_err into one function.
    fn check_ok_err<'a>(rule: Rule, ok: &[&'a str], err: &[&'a str])
                        -> (Option<Vec<String>>, Option<Vec<String>>)
    {
        (check_ok(rule, &ok), check_err(rule, &err))
    }
    
    #[test]
    fn parse_key() {
        let rule = Rule::key;
        let ok = ["foo", "bar", "foo123", "mr:filter",
                  "Glück", "Überraschung"];
        let err = ["@foo", ".abc"];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_alpha() {
        let rule = Rule::key;
        let ok = ["a", "b", "c", "ü", "ä"];
        let err = ["1", "2", "_"];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_unquoted_value() {
        let rule = Rule::unquoted_value;
        let ok = ["foo", "bar"];
        let err = [];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_quoted_value() {
        let rule = Rule::quoted_value;
        let ok = ["'foo'", "'bar'", "'foo bar'"];
        let err = ["'foo", "foo'"];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_value() {
        let rule = Rule::value;
        let ok = ["foo", "bar", "'foo'", "'bar'", "'foo bar'"];
        let err = [];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));        
    }

    #[test]
    fn parse_mql() {
        let rule = Rule::mql;
        let ok = ["foo", "bar", "foo,bar", " foo,bar "];
        let err = [];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_index_single() {
        let rule = Rule::index_single;
        let ok = ["[1]", "[42]"];
        let err = ["1", "42", "alpha", "[a]", "[1", "2]"];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

    #[test]
    fn parse_index_range() {
        let rule = Rule::index_range;
        let ok = ["[1:42]"];
        // TODO: these ranges do not work yet "[:42]", "[1:]", "[:]"];
        let err = ["1", "42", "alpha", "[a]", "[1", "2]", "[1]", "[42]"];
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));
    }

}
