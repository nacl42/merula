//! Parser to read a filter expression
//!

use pest::Parser;
use pest::iterators::Pair;

use crate::NodeFilter;

#[derive(Parser)]
#[grammar = "fex.pest"]
pub struct FexParser;

fn rule_key_op_value(pair: Pair<Rule>) -> Result<NodeFilter, ()> {
    let mut pairs = pair.into_inner();
    let key = pairs.next().unwrap().as_str();
    let op = pairs.next().unwrap().as_str();
    let value = pairs.next().unwrap().as_str();

    match op {
        "~" => Ok(
            NodeFilter::HasKey(key.into()) & NodeFilter::ContainsValue(value.into())
        ),
        "=" => Ok(
            NodeFilter::HasKey(key.into()) & NodeFilter::EqualsValue(value.into())
        ),
        &_ => Err(())
    }
}

fn rule_key(pair: Pair<Rule>) -> Result<NodeFilter, ()> {
    Ok(NodeFilter::HasKey(pair.as_str().to_string()))
}

pub fn parse_fex<'a>(input: &'a str) -> Result<NodeFilter, ()> {
    let mut pair = FexParser::parse(Rule::fex, &input)
        .expect("unsuccessful parse")
        .next().unwrap();

    //DEBUG println!("{:#?}", pair);
    
    match pair.as_rule() {
        Rule::fex => {
            //DEBUG println!("FEX RULE");
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
    fn parse_fex() {
        let input = "name~ium";
        //assert_eq(parse_fex(input), Ok(NodeFilter::))
        let fex = FexParser::parse(Rule::fex, &input)
            .expect("unsuccessful parse")
            .next().unwrap();

        let input = "name";
        let fex = FexParser::parse(Rule::fex, &input)
            .expect("unsuccessful parse")
            .next().unwrap();

        // let input = "~";
        // let fex = FexParser::parse(Rule::fex, &input)
        //     .expect("unsuccessful parse")
        //     .next().unwrap();

        // TODO: print output of test function
        
        // for line in fex.into_inner() {
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
