use std::collections::HashMap;
use std::collections::HashSet;

use set_state::SetState;
use grammar::Grammar;

pub struct SetBuilder<'rule> {
    first_set: HashMap<char, HashSet<char>>,
    follow_set: HashMap<char, HashSet<char>>,
    first_plus_set: HashMap<(char, &'rule str), HashSet<char>>,
}

impl<'rule> SetBuilder<'rule> {
    pub fn build(grammar: &Grammar<'rule>) -> SetBuilder<'rule> {
        let first_set = SetBuilder::build_first_set(&grammar);
        let follow_set = SetBuilder::build_follow_set(&grammar, &first_set);
        let first_plus_set = SetBuilder::build_first_plus_set(&grammar, &first_set, &follow_set);
        SetBuilder {
            first_set: first_set,
            follow_set: follow_set,
            first_plus_set: first_plus_set,
        }
    }

    pub fn get_sets(self)
                    -> (HashMap<char, HashSet<char>>,
                        HashMap<char, HashSet<char>>,
                        HashMap<(char, &'rule str), HashSet<char>>) {
        (self.first_set, self.follow_set, self.first_plus_set)
    }

    fn build_first_set(grammar: &Grammar<'rule>) -> HashMap<char, HashSet<char>> {
        let mut first_set: HashMap<char, HashSet<char>> = HashMap::new();
        {
            let mut hash_set = HashSet::new();
            for t in &grammar.terminals {
                hash_set.insert(t.clone());
                first_set.insert(t.clone(), hash_set.clone());
                hash_set.clear();
            }

            for nt in &grammar.non_terminals {
                first_set.insert(nt.clone(), hash_set.clone());
            }
        }

        let mut first_set_state = SetState::new(10);

        while first_set_state.get_state() {
            for (non_terminal, production) in &grammar.production_rules {
                for rule in production.iter() {

                    let rule_length = rule.len() - 1;
                    let symbols = rule.chars().collect::<Vec<_>>();

                    let mut rhs = if let Some(set) = first_set.get(&symbols[0]) {
                        let mut set = set.clone();
                        set.remove(&'e');
                        set
                    } else {
                        HashSet::new()
                    };

                    let mut i = 0;
                    while i < rule_length {
                        let beta_set = first_set.get(&symbols[i]);
                        if let Some(beta_set) = beta_set {
                            if beta_set.contains(&'e') {
                                if let Some(rhs_to_merge) = first_set.get(&symbols[i + 1]) {
                                    let mut rhs_to_merge = rhs_to_merge.clone();
                                    rhs_to_merge.remove(&'e');
                                    for item in rhs_to_merge {
                                        rhs.insert(item);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        i += 1;
                    }
                    if let Some(first_set) = first_set.get(&symbols[i]) {
                        if first_set.contains(&'e') {
                            rhs.insert('e');
                        }
                    } else if &symbols[i] == &'e' {
                        rhs.insert('e');
                    }
                    if let Some(mut ntfs) = first_set.get_mut(non_terminal) {
                        for item in &rhs {
                            first_set_state.is_changing(ntfs.insert(item.clone()));
                        }
                    }
                }
            }
            first_set_state.update_time();
        }
        first_set
    }

    fn build_follow_set(grammar: &Grammar<'rule>,
                        first_set: &HashMap<char, HashSet<char>>)
                        -> HashMap<char, HashSet<char>> {

        let mut follow_set = HashMap::new();
        for nt in &grammar.non_terminals {
            follow_set.insert(nt.clone(), HashSet::new());
        }

        {
            let start_symbol = follow_set.get_mut(&grammar.non_terminals[0]);
            if let Some(start_symbol) = start_symbol {
                start_symbol.insert('$');
            }
        }

        let mut follow_set_state = SetState::new(10);

        while follow_set_state.get_state() {
            for (non_terminal, production) in &grammar.production_rules {
                for rule in production.iter() {

                    let rule_length = rule.len() - 1;
                    let symbols = rule.chars().collect::<Vec<_>>();

                    let mut trailer = if let Some(follow_set) = follow_set.get(&non_terminal) {
                        follow_set.clone()
                    } else {
                        HashSet::new()
                    };

                    let mut i: i32 = rule_length as i32;
                    while i >= 0 {
                        let symbol = &symbols[i as usize];
                        if grammar.non_terminals.contains(symbol) {
                            if let Some(follow) = follow_set.get_mut(symbol) {
                                for item in &trailer {
                                    follow_set_state.is_changing(follow.insert(item.clone()));
                                }
                                if let Some(symbol_first_set) = first_set.get(symbol) {
                                    if symbol_first_set.contains(&'e') {
                                        let mut symbol_first_set = symbol_first_set.clone();
                                        symbol_first_set.remove(&'e');
                                        for item in symbol_first_set {
                                            trailer.insert(item);
                                        }
                                    } else {
                                        trailer = symbol_first_set.clone();
                                    }
                                }
                            }
                        } else {
                            if let Some(symbol_first_set) = first_set.get(symbol) {
                                trailer = symbol_first_set.clone();
                            }
                        }
                        i -= 1;
                    }
                }
            }
            follow_set_state.update_time();
        }
        follow_set
    }

    fn build_first_plus_set(grammar: &Grammar<'rule>,
                            first_set: &HashMap<char, HashSet<char>>,
                            follow_set: &HashMap<char, HashSet<char>>)
                            -> HashMap<(char, &'rule str), HashSet<char>> {
                                
        let mut first_plust_set: HashMap<(char, &str), HashSet<char>> = HashMap::new();
        let mut hash_set = HashSet::new();
        for (non_terminal, production_rule) in &grammar.production_rules {
            for rule in production_rule {
                if let Some(first_symbols) = first_set.get(&rule[..].chars().next().unwrap()) {
                    for symbol in first_symbols {
                        hash_set.insert(symbol.clone());
                    }

                    if hash_set.contains(&'e') {
                        if let Some(follow_symbols) = follow_set.get(non_terminal) {
                            for symbol in follow_symbols {
                                hash_set.insert(symbol.clone());
                            }
                        }
                    }
                    if hash_set.len() > 0 {
                        first_plust_set.insert((non_terminal.clone(), rule.clone()),
                                               hash_set.clone());
                        hash_set.clear();
                    }
                }
            }
        }
        first_plust_set
    }

    pub fn get_first_set(&self) -> &HashMap<char, HashSet<char>> {
        &self.first_set
    }
    pub fn get_follow_set(&self) -> &HashMap<char, HashSet<char>> {
        &self.follow_set
    }
    pub fn get_first_plus_set(&self) -> &HashMap<(char, &str), HashSet<char>> {
        &self.first_plus_set
    }

    pub fn print_first_set(&self) {
        println!("First Set");
        for (key, val) in &self.first_set {
            print!("{} = ", key);
            for v in val {
                print!("{}, ", v);
            }
            println!("");
        }
        println!("");
    }
    pub fn print_follow_set(&self) {
        println!("Follow Set");
        for (key, val) in &self.follow_set {
            print!("{} = ", key);
            for v in val {
                print!("{}, ", v);
            }
            println!("");
        }
        println!("");
    }
    pub fn print_first_plus_set(&self) {
        println!("First+ Set", );
        for (key, val) in &self.first_plus_set {
            print!("{} -> {} = ", key.0, key.1);
            for v in val {
                print!("{}, ", v);
            }
            println!("");
        }
        println!("");
    }
}
