//! Parser to read mql expressions

use pest::Parser;
use pest::iterators::Pair;

use crate::{NodeFilter, MemoFilter, KindFilter, ValueFilter, KeyFilter};
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
    let mut prefix: Option<&str> = None;
    let mut key: Option<&str> = None;
    let mut operator: Option<&str> = None;
    let mut value: Option<&str> = None;
    
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::prefix => prefix = Some(pair.as_str()),
            Rule::key => key = Some(pair.as_str()),
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
            _ => { println!("rule? {:?}", pair.as_rule())}
        }
    }

    // construct NodeFilter object
    let kind_filter = match prefix {
        Some("@") => Some(KindFilter::Header),
        Some(".") => Some(KindFilter::Data),
        _ => None
    };
    
    let key_filter = match key {
        Some(key) => Some(KeyFilter::Equals(key.into())),
        _ => None
    };

    let value_filter = match (operator, value) {
        (Some("="), Some(s)) => Some(ValueFilter::Equals(s.into())),
        (Some("~"), Some(s)) => Some(ValueFilter::Contains(s.into())),
        _ => None
    };

    debug!("kind-filter = {:?}", kind_filter);
    debug!("key-filter = {:?}", key_filter);
    debug!("value-filter = {:?}", value_filter);

    let mut nf = NodeFilter::new();
    nf.kind = kind_filter;
    nf.key = key_filter;
    nf.value = value_filter;

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
        let ok = ["foo", "bar", "foo123", "mr:filter"];
        let err = ["@foo", ".abc"];
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
        assert_eq!(check_ok_err(rule, &ok, &err), (None, None));                 }
}












// pub fn parse_mql<'a>(input: &'a str) -> Result<MemoFilter, String> {
//     let pairs = MqlParser::parse(Rule::mql, &input)
//         .expect("unsuccessful parse");

//     let mut filter = MemoFilter::new();
    
//     for pair in pairs {
//         match pair.as_rule() {
//             Rule::expr => {
//                 debug!("found filter expression: {}", pair.as_str());
//                 let new_filter = parse_expr(pair).unwrap();
//                 filter.add(new_filter);
//             },
//             Rule::more_expr => {
//                 debug!("found another filter expression: {}", pair.as_str());
//                 let pair = pair.into_inner().next().unwrap();
//                 let new_filter = parse_expr(pair).unwrap();
//                 filter.add(new_filter);
//             }
//             Rule::EOI => {},
//             _ => {
//                 debug!("Could not parse filter expression: {:#?}", pair);
//                 return Err(format!("unknown filter expression: {:#?}", pair))
//             }
//         }
//     }
//     Ok(filter)
// }


// fn parse_expr(pair: Pair<Rule>) -> Result<NodeFilter, String> {
//     let pair = pair.into_inner().next().unwrap();
//     match pair.as_rule() {
//         Rule::key_op_value => parse_key_op_value(pair),
//         Rule::key => parse_key(pair),
//         _ => Err("".into())
//     }
// }


// fn parse_key(pair: Pair<Rule>) -> Result<NodeFilter, String> {
//     let mut nf = NodeFilter::new();
//     nf.key = Some(KeyFilter::Equals(pair.as_str().to_string()));
//     Ok(nf)
// }

// fn parse_key_op_value(pair: Pair<Rule>) -> Result<NodeFilter, String> {
//     let mut pairs = pair.into_inner();
//     let key = pairs.next().unwrap().as_str();
//     let op = pairs.next().unwrap().as_str();

//     // for quoted values, remove quote characters
//     let value_pair = pairs.next().unwrap();
//     let value = match value_pair.as_rule() {
//         Rule::quoted_value => {
//             let value = value_pair.as_str();
//             &value[1..value.len() - 1]
//         },
//         _ => value_pair.as_str()
//     };

//     match op {
//         "~" => {
//             let mut nf = NodeFilter::new();
//             nf.key = Some(KeyFilter::Equals(key.into()));
//             nf.value = Some(ValueFilter::Contains(value.into()));
//             Ok(nf)
//         },
//         "=" => {
//             let mut nf = NodeFilter::new();
//             nf.key = Some(KeyFilter::Equals(key.into()));
//             nf.value = Some(ValueFilter::Equals(value.into()));
//             Ok(nf)
//         },
//         "<" => {
//             match value.parse::<f32>() {
//                 Ok(number) => {
//                     let mut nf = NodeFilter::new();
//                     nf.key = Some(KeyFilter::Equals(key.into()));
//                     nf.value = Some(ValueFilter::LessThan(number));
//                     debug!("Setting up < filter: {:#?}", nf);
//                     Ok(nf)
//                 },
//                 Err(_) => Err("".into()) // TODO: pass on error message
//             }
//         },
//         "<=" => {
//             match value.parse::<f32>() {
//                 Ok(number) => {
//                     let mut nf = NodeFilter::new();
//                     nf.key = Some(KeyFilter::Equals(key.into()));
//                     nf.value = Some(ValueFilter::AtMost(number));
//                     Ok(nf)
//                 },
//                 Err(_) => Err("".into()) // TODO: pass on error message
//             }
//         },
//         ">" => {
//             match value.parse::<f32>() {
//                 Ok(number) => {
//                     let mut nf = NodeFilter::new();
//                     nf.key = Some(KeyFilter::Equals(key.into()));
//                     nf.value = Some(ValueFilter::MoreThan(number));
//                     Ok(nf)
//                 },
//                 Err(_) => Err("".into()) // TODO: pass on error message
//             }
//         },
//         ">=" => {
//             match value.parse::<f32>() {
//                 Ok(number) => {
//                     let mut nf = NodeFilter::new();
//                     nf.key = Some(KeyFilter::Equals(key.into()));
//                     nf.value = Some(ValueFilter::AtLeast(number));
//                     Ok(nf)
//                 },
//                 Err(_) => Err("".into()) // TODO: pass on error message
//             }
//         },
//         &_ => Err("".into())
//     }
// }


