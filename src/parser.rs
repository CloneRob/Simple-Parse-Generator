use std::collections::HashMap;
use set_builder::SetBuilder;
use grammar::Grammar;

#[derive(Debug)]
pub enum InputErr {
    NotTerminated,
    Empty,
}

#[derive(Debug)]
pub enum ParseError {
    In(InputErr),
    TableErr,
    StackErr,
}

pub struct Parser<'rule> {
    grammar: Grammar<'rule>,
    ll1_table: HashMap<(char, char), Option<(char, &'rule str)>>,
}

impl<'rule> Parser<'rule> {
    pub fn new(grammar: Grammar<'rule>,
               set_builder: SetBuilder<'rule>)
               -> Result<Parser<'rule>, &'static str> {
        match Parser::build_table(&set_builder, &grammar) {
            Ok(table) => {
                Ok(Parser {
                    ll1_table: table,
                    grammar: grammar,
                })
            }
            Err(e) => Err(e),
        }
    }

    fn input_eof(src: &str) -> Result<&'static str, InputErr> {
        let last_symbol = src[..].chars().last();
        if let Some(symbol) = last_symbol {
            if symbol != '$' {
                return Err(InputErr::NotTerminated);
            } else {
                return Ok("src is $ terminated");
            }
        } else {
            return Err(InputErr::Empty);
        }
    }

    fn build_table
        (set_builder: &SetBuilder<'rule>,
         grammar: &Grammar<'rule>)
         -> Result<HashMap<(char, char), Option<(char, &'rule str)>>, &'static str> {

        match Parser::check_ll1(set_builder) {
            Err(msg) => return Err(msg),
            _ => (),
        }
        let mut table = HashMap::new();

        for nt in &grammar.non_terminals {
            for t in &grammar.terminals {
                table.insert((nt.clone(), t.clone()), None);
            }

            let rules = grammar.production_rules.get(nt).unwrap();
            for rule in rules {
                let mut eof_flag = false;
                let fps_terminals = set_builder.get_first_plus_set().get(&(*nt, *rule));
                if let Some(fps_terminals) = fps_terminals {
                    for fps_t in fps_terminals {
                        table.insert((nt.clone(), fps_t.clone()),
                                     Some((nt.clone(), rule.clone())));
                        if fps_t == &'$' {
                            eof_flag = true;
                        }
                    }
                }
                if eof_flag {
                    table.insert((nt.clone(), '$'), Some((nt.clone(), rule.clone())));
                }
            }
        }
        Ok(table)
    }

    fn check_ll1(set_builder: &SetBuilder<'rule>) -> Result<&'static str, &'static str> {
        for (outer_key, outer_val) in set_builder.get_first_plus_set() {
            for (inner_key, inner_val) in set_builder.get_first_plus_set() {
                if outer_key.0 != inner_key.0 {
                    continue;
                } else if outer_key.1 == inner_key.1 {
                    continue;
                } else {
                    if outer_val.intersection(inner_val).collect::<Vec<_>>().len() > 0 {
                        return Err("No LL(1) grammar");
                    }
                }

            }
        }
        Ok("Grammar is LL(1)")
    }

    pub fn parse(&self, src: &str) -> Result<&'static str, ParseError> {
        if let Err(e) = Parser::input_eof(src) {
            return Err(ParseError::In(e));
        }

        let mut input_iter = src.chars();
        let mut current_input = input_iter.next();
        let mut stack = vec!['$', self.grammar.start_symbol];

        loop {
            let focus_index = stack.len() - 1;
            let focus = stack[focus_index];
            if let Some(word) = current_input {
                println!("Current symbol and focus: {} {}, \nCurrent stack: {:?}",
                         word,
                         focus,
                         stack);
                if focus == '$' && word == '$' {
                    return Ok("ACCEPT");

                } else if self.grammar.terminals[..].contains(&focus) || focus == '$' {
                    if focus == word {
                        stack.pop();
                        current_input = input_iter.next();
                    } else {
                        return Err(ParseError::StackErr);
                    }
                } else {
                    if let Some(entry) = self.ll1_table.get(&(focus, word)) {
                        if let Some(rule) = *entry {

                            stack.pop();
                            // focus_index = stack.len();

                            for symbol in rule.1[..].chars().rev() {
                                if symbol != 'e' {
                                    stack.push(symbol);
                                }
                            }
                        }
                    } else {
                        return Err(ParseError::TableErr);
                    }
                }
            }
        }

    }
}
