//! Parser to read mql expressions

use pest::Parser;
use pest::iterators::Pair;

use crate::{NodeFilter, MemoFilter, ValueFilter, KeyFilter};
use log::*;

#[derive(Parser)]
#[grammar = "mql.pest"]
pub struct MqlParser;


pub fn parse_mql<'a>(input: &'a str) -> Result<MemoFilter, String> {
    let pairs = MqlParser::parse(Rule::mql, &input)
        .expect("unsuccessful parse");

    let mut filter = MemoFilter::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                debug!("found filter expression: {}", pair.as_str());
                let new_filter = parse_expr(pair).unwrap();
                filter.add_filter(new_filter);
            },
            Rule::more_expr => {
                debug!("found another filter expression: {}", pair.as_str());
                let pair = pair.into_inner().next().unwrap();
                let new_filter = parse_expr(pair).unwrap();
                filter.add_filter(new_filter);
            }
            Rule::EOI => {},
            _ => {
                debug!("Could not parse filter expression: {:#?}", pair);
                return Err(format!("unknown filter expression: {:#?}", pair))
            }
        }
    }
    Ok(filter)
}


fn parse_expr(pair: Pair<Rule>) -> Result<NodeFilter, String> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::key_op_value => parse_key_op_value(pair),
        Rule::key => parse_key(pair),
        _ => Err("".into())
    }
}


fn parse_key(pair: Pair<Rule>) -> Result<NodeFilter, String> {
    let mut nf = NodeFilter::new();
    nf.key = Some(KeyFilter::Equals(pair.as_str().to_string()));
    Ok(nf)
}

fn parse_key_op_value(pair: Pair<Rule>) -> Result<NodeFilter, String> {
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
        "~" => {
            let mut nf = NodeFilter::new();
            nf.key = Some(KeyFilter::Equals(key.into()));
            nf.value = Some(ValueFilter::Contains(value.into()));
            Ok(nf)
        },
        "=" => {
            let mut nf = NodeFilter::new();
            nf.key = Some(KeyFilter::Equals(key.into()));
            nf.value = Some(ValueFilter::Equals(value.into()));
            Ok(nf)
        },
        "<" => {
            match value.parse::<f32>() {
                Ok(number) => {
                    let mut nf = NodeFilter::new();
                    nf.key = Some(KeyFilter::Equals(key.into()));
                    nf.value = Some(ValueFilter::LessThan(number));
                    debug!("Setting up < filter: {:#?}", nf);
                    Ok(nf)
                },
                Err(_) => Err("".into()) // TODO: pass on error message
            }
        },
        "<=" => {
            match value.parse::<f32>() {
                Ok(number) => {
                    let mut nf = NodeFilter::new();
                    nf.key = Some(KeyFilter::Equals(key.into()));
                    nf.value = Some(ValueFilter::AtMost(number));
                    Ok(nf)
                },
                Err(_) => Err("".into()) // TODO: pass on error message
            }
        },
        ">" => {
            match value.parse::<f32>() {
                Ok(number) => {
                    let mut nf = NodeFilter::new();
                    nf.key = Some(KeyFilter::Equals(key.into()));
                    nf.value = Some(ValueFilter::MoreThan(number));
                    Ok(nf)
                },
                Err(_) => Err("".into()) // TODO: pass on error message
            }
        },
        ">=" => {
            match value.parse::<f32>() {
                Ok(number) => {
                    let mut nf = NodeFilter::new();
                    nf.key = Some(KeyFilter::Equals(key.into()));
                    nf.value = Some(ValueFilter::AtLeast(number));
                    Ok(nf)
                },
                Err(_) => Err("".into()) // TODO: pass on error message
            }
        },
        &_ => Err("".into())
    }
}


