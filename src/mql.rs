//! Parser to read a filter expression
//!

use pest::Parser;
use pest::iterators::Pair;

use crate::NodeFilter;
use log::*;

#[derive(Parser)]
#[grammar = "mql.pest"]
pub struct MqlParser;

fn rule_key_op_value(pair: Pair<Rule>) -> Result<NodeFilter, ()> {
    let mut pairs = pair.into_inner();
    let key = pairs.next().unwrap().as_str();
    let op = pairs.next().unwrap().as_str();

    // for quoted values, remove quote characters
    let value_pair = pairs.next().unwrap();
    let value = match value_pair.as_rule() {
        Rule::quoted_value => {
            let value = value_pair.as_str();
            &value[1..value.len() - 1]
        },
        _ => value_pair.as_str()
    };

    match op {
        "~" => Ok(
            NodeFilter::HasKey(key.into()) & NodeFilter::Contains(value.into())
        ),
        "=" => Ok(
            NodeFilter::HasKey(key.into()) & NodeFilter::Equals(value.into())
        ),
        "<" => {
            match value.parse::<f32>() {
                Ok(number) => Ok(
                    NodeFilter::HasKey(key.into()) &
                        NodeFilter::LessThan(number)
                ),
                Err(_) => Err(()) // TODO: pass on error message
            }
        },
        "<=" => {
            match value.parse::<f32>() {
                Ok(number) => Ok(
                    NodeFilter::HasKey(key.into()) &
                        NodeFilter::LessOrEqual(number)
                ),
                Err(_) => Err(()) // TODO: pass on error message
            }
        },
        ">" => {
            match value.parse::<f32>() {
                Ok(number) => Ok(
                    NodeFilter::HasKey(key.into()) &
                        NodeFilter::GreaterThan(number)
                ),
                Err(_) => Err(()) // TODO: pass on error message
            }
        },
        ">=" => {
            match value.parse::<f32>() {
                Ok(number) => Ok(
                    NodeFilter::HasKey(key.into()) &
                        NodeFilter::GreaterOrEqual(number)
                ),
                Err(_) => Err(()) // TODO: pass on error message
            }
        },

        &_ => Err(())
    }
}

fn rule_key(pair: Pair<Rule>) -> Result<NodeFilter, ()> {
    Ok(NodeFilter::HasKey(pair.as_str().to_string()))
}

pub fn parse_mql<'a>(input: &'a str) -> Result<NodeFilter, ()> {
    let mut pair = MqlParser::parse(Rule::mql, &input)
        .expect("unsuccessful parse")
        .next().unwrap();

    //DEBUG println!("{:#?}", pair);
    
    match pair.as_rule() {
        Rule::mql => {
            //DEBUG println!("MQL RULE");
            pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::key_op_value => rule_key_op_value(pair),
                Rule::key => rule_key(pair),
                _ => Err(())
            }
        },
        _ => Err(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mql() {
        let input = "name~ium";
        //assert_eq(parse_mql(input), Ok(NodeFilter::))
        let mql = MqlParser::parse(Rule::mql, &input)
            .expect("unsuccessful parse")
            .next().unwrap();

        let input = "name";
        let mql = MqlParser::parse(Rule::mql, &input)
            .expect("unsuccessful parse")
            .next().unwrap();

        // let input = "~";
        // let mql = MqlParser::parse(Rule::mql, &input)
        //     .expect("unsuccessful parse")
        //     .next().unwrap();

        // TODO: print output of test function
        
        // for line in mql.into_inner() {
        //     match line.as_rule() {
        //         Rule:: => {
        //             // comments are currently ignored
        //             let mut inner_rules = line.into_inner();
        //             let value = inner_rules.next().unwrap().as_str();
        //             debug!("# {}", value);
        //         }
        //     }
        // }
    }

}
