//! Parser to read a filter expression
//!

use pest::Parser;

#[derive(Parser)]
#[grammar = "fex.pest"]
pub struct FexParser;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fex() {
        let input = "name~ium";
        let fex = FexParser::parse(Rule::fex, &input)
            .expect("unsuccessful parse")
            .next().unwrap();

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
