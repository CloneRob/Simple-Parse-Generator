
mod grammar;
mod set_builder;
mod set_state;
mod parser;

mod u6;

use set_builder::SetBuilder;
use parser::Parser;


fn main() {
    let grammars = u6::build_test_grammars();
    for grammar in grammars {
        //The Type of grammar is Tuple(Grammar, ImputString)
        grammar.0.print();
        let set_builder = SetBuilder::build(&grammar.0);

        println!("");
        set_builder.print_first_set();
        set_builder.print_follow_set();
        set_builder.print_first_plus_set();

        match Parser::new(grammar.0, set_builder) {
            Ok(parser) => {
                println!("Test Input: {}", grammar.1);
                match parser.parse(grammar.1) {
                    Ok(v) => println!("{:?}", v),
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        println!("------------------------------------------------\n");
    }
}
// More complex expression grammar
// let expr_grammar = Grammar::new(
// 'G',
// vec!('G', 'E', 'X', 'T', 'U', 'F'),
// vec!('-', '+', '(', ')', '*', '/', 'n', 'a', 'e'),
// {
// let mut rules = HashMap::new();
//
// let goal_rules = vec!("E");
// let expr_rules = vec!("TX");
// let expr_prime_rules = vec!("+TX", "-TX", "e");
// let term_rules = vec!("FU");
// let term_prime_rules = vec!("*FU", "/FU", "e");
// let factor_rules = vec!("(E)", "n", "a");
//
// rules.insert('G', goal_rules);
// rules.insert('E', expr_rules);
// rules.insert('X', expr_prime_rules);
// rules.insert('T', term_rules);
// rules.insert('U', term_prime_rules);
// rules.insert('F', factor_rules);
//
// rules
// },
// );
//

// fn convert_str(s: &str) -> String {
// let mut output_string = String::new();
// for c in s.chars() {
// output_string.push_str(&convert_char(c));
// output_string.push_str(" ");
// }
// output_string
// }
//
// fn convert_char(c: char) -> String {
// let val = match c {
// 'a' => String::from("name"),
// 'n' => String::from("num"),
// 's' => String::from("eof"),
// 'G' => String::from("Goal"),
// 'E' => String::from("Expr"),
// 'X' => String::from("ExprPrime"),
// 'T' => String::from("Term"),
// 'U' => String::from("TermPrime"),
// 'F' => String::from("Factor"),
// _ => c.to_string(),
// };
//
// val
// }
//
