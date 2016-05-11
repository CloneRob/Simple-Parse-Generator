use std::collections::HashMap;

#[derive(Clone)]
pub struct Grammar<'rule> {
    pub start_symbol: char,
    pub non_terminals: Vec<char>,
    pub terminals: Vec<char>,
    pub production_rules: HashMap<char, Vec<&'rule str>>,
}

impl<'rule> Grammar<'rule> {
    pub fn new(start_symbol: char,
               non_terminals: Vec<char>,
               terminals: Vec<char>,
               production_rules: HashMap<char, Vec<&str>>)
               -> Grammar {
        Grammar {
            start_symbol: start_symbol,
            non_terminals: non_terminals,
            terminals: terminals,
            production_rules: production_rules,
        }
    }

    pub fn print(&self) {
        println!("Grammar: ");
        println!("Nonterminals: {:?}", self.non_terminals);
        println!("Terminals: {:?}", self.terminals);

        for (key, val) in &self.production_rules {
            print!("\t{} -> ", key);
            for v in val {
                print!("{} | ", v);
            }
            println!("");
        }
        println!("");
    }
}
