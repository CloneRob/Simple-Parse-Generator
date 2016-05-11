use std::collections::HashMap;
use grammar::Grammar;

pub fn build_test_grammars<'rule>() -> Vec<(Grammar<'rule>, &'static str)> {
    vec![(build_grammar_a(), "(+a)+a+a$"), (build_grammar_b(), "bbababa$"), (build_grammar_c(), "acbbc$"), (build_grammar_d(), "ababbaa$")]
}

fn build_grammar_a<'rule>() -> Grammar<'rule> {
    Grammar::new('S', vec!['S', 'E', 'T'], vec!['a', '+', '(', ')', 'e'], {
        let mut rules = HashMap::new();

        let s_rules = vec!["TE"];
        let e_rules = vec!["+TE", "e"];
        let t_rules = vec!["a", "(E)"];

        rules.insert('S', s_rules);
        rules.insert('E', e_rules);
        rules.insert('T', t_rules);

        rules
    })
}

fn build_grammar_b<'rule>() -> Grammar<'rule> {
    Grammar::new('S', vec!['S', 'T'], vec!['a', 'b'], {
        let mut rules = HashMap::new();

        let s_rules = vec!["aT", "TbS"];
        let t_rules = vec!["bT", "ba"];

        rules.insert('S', s_rules);
        rules.insert('T', t_rules);

        rules
    })
}

fn build_grammar_c<'rule>() -> Grammar<'rule> {
    Grammar::new('S', vec!['S', 'A'], vec!['a', 'b', 'c'], {
        let mut rules = HashMap::new();

        let s_rules = vec!["aAc", "b"];
        let a_rules = vec!["a", "cSb"];

        rules.insert('S', s_rules);
        rules.insert('A', a_rules);

        rules
    })
}

fn build_grammar_d<'rule>() -> Grammar<'rule> {
    Grammar::new('S', vec!['S', 'A'], vec!['a', 'b', 'e'], {
        let mut rules = HashMap::new();

        let s_rules = vec!["abA", "e"];
        let a_rules = vec!["Saa", "b"];

        rules.insert('S', s_rules);
        rules.insert('A', a_rules);

        rules
    })
}
