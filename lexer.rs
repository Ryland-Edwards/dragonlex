use std::env;
use std::fs;
use std::collections::HashMap;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let input = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            process::exit(1);
        }
    };

    let tokens = tokenize(&input);
    for token in tokens {
        println!("{}", token);
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;
    let mut pos = 0;
    let chars: Vec<char> = input.chars().collect();

    let mut transitions = HashMap::new();
    transitions.insert((0, '8'), 1);
    transitions.insert((0, 'D'), 1);
    transitions.insert((0, '{'), 1);
    transitions.insert((0, 'n'), 9);
    transitions.insert((0, ';'), 1);
    transitions.insert((9, '\\'), 15);
    transitions.insert((0, '%'), 1);
    transitions.insert((0, '7'), 1);
    transitions.insert((0, 'p'), 1);
    transitions.insert((0, 'F'), 1);
    transitions.insert((0, 'X'), 1);
    transitions.insert((0, 'Z'), 1);
    transitions.insert((0, 'r'), 9);
    transitions.insert((0, 'V'), 1);
    transitions.insert((11, 'r'), 12);
    transitions.insert((0, 'q'), 1);
    transitions.insert((25, 't'), 26);
    transitions.insert((0, 'R'), 1);
    transitions.insert((0, '$'), 1);
    transitions.insert((0, '\''), 1);
    transitions.insert((0, 't'), 9);
    transitions.insert((16, 'n'), 18);
    transitions.insert((0, ','), 1);
    transitions.insert((0, 'H'), 1);
    transitions.insert((0, 'J'), 1);
    transitions.insert((0, '['), 1);
    transitions.insert((0, '?'), 1);
    transitions.insert((0, 'M'), 1);
    transitions.insert((0, 'b'), 5);
    transitions.insert((0, 'f'), 1);
    transitions.insert((0, 'k'), 1);
    transitions.insert((0, '#'), 1);
    transitions.insert((0, '*'), 1);
    transitions.insert((0, '~'), 1);
    transitions.insert((0, 'G'), 1);
    transitions.insert((0, ' '), 1);
    transitions.insert((0, 's'), 1);
    transitions.insert((6, 'o'), 23);
    transitions.insert((0, 'y'), 1);
    transitions.insert((3, 'n'), 15);
    transitions.insert((3, 't'), 15);
    transitions.insert((0, '1'), 1);
    transitions.insert((0, '2'), 1);
    transitions.insert((0, 'W'), 1);
    transitions.insert((0, 'T'), 1);
    transitions.insert((19, 'l'), 20);
    transitions.insert((0, 'B'), 1);
    transitions.insert((0, 'i'), 1);
    transitions.insert((0, '|'), 1);
    transitions.insert((12, 'l'), 13);
    transitions.insert((17, 'n'), 18);
    transitions.insert((0, '='), 1);
    transitions.insert((0, '_'), 4);
    transitions.insert((0, 'K'), 1);
    transitions.insert((0, '5'), 1);
    transitions.insert((0, 'L'), 1);
    transitions.insert((27, 's'), 28);
    transitions.insert((0, 'z'), 1);
    transitions.insert((0, 'N'), 1);
    transitions.insert((0, '^'), 1);
    transitions.insert((0, 'm'), 8);
    transitions.insert((0, '&'), 1);
    transitions.insert((0, '>'), 1);
    transitions.insert((0, 'I'), 1);
    transitions.insert((0, 'u'), 1);
    transitions.insert((0, 'v'), 1);
    transitions.insert((0, ')'), 1);
    transitions.insert((0, '<'), 1);
    transitions.insert((0, '3'), 1);
    transitions.insert((0, '4'), 1);
    transitions.insert((0, '@'), 1);
    transitions.insert((0, 'E'), 1);
    transitions.insert((0, 'S'), 1);
    transitions.insert((0, 'x'), 1);
    transitions.insert((9, 'r'), 15);
    transitions.insert((10, 'o'), 11);
    transitions.insert((0, 'j'), 1);
    transitions.insert((0, 'Y'), 1);
    transitions.insert((0, '!'), 2);
    transitions.insert((0, 'O'), 1);
    transitions.insert((0, '.'), 1);
    transitions.insert((0, '0'), 1);
    transitions.insert((21, 'o'), 22);
    transitions.insert((0, 'c'), 1);
    transitions.insert((0, '\"'), 1);
    transitions.insert((8, 'a'), 16);
    transitions.insert((0, '}'), 1);
    transitions.insert((3, '\\'), 15);
    transitions.insert((0, 'e'), 1);
    transitions.insert((0, 'U'), 1);
    transitions.insert((0, ':'), 1);
    transitions.insert((0, '('), 1);
    transitions.insert((0, 'a'), 1);
    transitions.insert((0, 'A'), 1);
    transitions.insert((15, '\\'), 15);
    transitions.insert((0, '/'), 1);
    transitions.insert((15, 'n'), 15);
    transitions.insert((9, 't'), 15);
    transitions.insert((0, ']'), 1);
    transitions.insert((0, 'g'), 1);
    transitions.insert((3, 'r'), 15);
    transitions.insert((0, '6'), 1);
    transitions.insert((0, '-'), 1);
    transitions.insert((0, 'Q'), 1);
    transitions.insert((9, 'n'), 15);
    transitions.insert((15, 't'), 15);
    transitions.insert((23, 'g'), 24);
    transitions.insert((20, 'l'), 21);
    transitions.insert((0, 'w'), 10);
    transitions.insert((0, '9'), 1);
    transitions.insert((0, 'P'), 1);
    transitions.insert((0, 'h'), 7);
    transitions.insert((0, 'o'), 1);
    transitions.insert((13, 'd'), 14);
    transitions.insert((15, 'r'), 15);
    transitions.insert((26, 'e'), 27);
    transitions.insert((0, '`'), 1);
    transitions.insert((0, 'd'), 6);
    transitions.insert((0, 'C'), 1);
    transitions.insert((7, 'e'), 19);
    transitions.insert((16, 'a'), 17);
    transitions.insert((0, 'l'), 1);
    transitions.insert((5, 'i'), 25);
    transitions.insert((17, 'a'), 17);
    transitions.insert((0, '+'), 1);
    transitions.insert((0, '\\'), 3);

    let mut accepting_states = HashMap::new();
    accepting_states.insert(6, 8);
    accepting_states.insert(4, 7);
    accepting_states.insert(10, 8);
    accepting_states.insert(18, 2);
    accepting_states.insert(14, 4);
    accepting_states.insert(9, 6);
    accepting_states.insert(8, 8);
    accepting_states.insert(24, 0);
    accepting_states.insert(1, 8);
    accepting_states.insert(5, 8);
    accepting_states.insert(3, 6);
    accepting_states.insert(22, 3);
    accepting_states.insert(28, 1);
    accepting_states.insert(2, 5);
    accepting_states.insert(7, 8);
    accepting_states.insert(15, 6);

    let rules = vec![
        RuleAction::Token { name: "NOUN".to_string(), keep_lexeme: true },
        RuleAction::Token { name: "VERB".to_string(), keep_lexeme: true },
        RuleAction::Token { name: "NOUN".to_string(), keep_lexeme: true },
        RuleAction::Token { name: "GREETING".to_string(), keep_lexeme: true },
        RuleAction::Token { name: "NOUN".to_string(), keep_lexeme: true },
        RuleAction::Token { name: "PUNCT".to_string(), keep_lexeme: true },
        RuleAction::Skip,
        RuleAction::Skip,
        RuleAction::Error("bad input".to_string()),
    ];

    while pos < chars.len() {
        let (token_length, rule_index) = longest_match(&chars[pos..], &transitions, &accepting_states);

        if token_length > 0 {
            let lexeme: String = chars[pos..pos + token_length].iter().collect();
            
            if let Some(rule_idx) = rule_index {
                match &rules[rule_idx] {
                    RuleAction::Skip => {},
                    RuleAction::Error(msg) => {
                        eprintln!("{}", msg);
                    },
                    RuleAction::Token { name, keep_lexeme } => {
                        let token_str = if *keep_lexeme {
                            format!("{}:{} [{},{}]", name, lexeme, line, column)
                        } else {
                            format!("{} [{},{}]", name, line, column)
                        };
                        tokens.push(token_str);
                    },
                }
            }

            // Update position
            for i in pos..pos + token_length {
                if chars[i] == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
            pos += token_length;
        } else {
            // No match found, skip character
            if chars[pos] == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            pos += 1;
        }
    }

    // Add EOF token
    tokens.push(format!("EOF [{},{}]", line, column));
    tokens
}

#[derive(Debug, Clone)]
enum RuleAction {
    Skip,
    Error(String),
    Token { name: String, keep_lexeme: bool },
}

fn longest_match(
    input: &[char],
    transitions: &HashMap<(usize, char), usize>,
    accepting_states: &HashMap<usize, usize>
) -> (usize, Option<usize>) {
    let mut current_state = 0;
    let mut last_accepting_pos = 0;
    let mut last_accepting_rule = None;

    // Check if start state is accepting
    if let Some(&rule_index) = accepting_states.get(&current_state) {
        last_accepting_pos = 0;
        last_accepting_rule = Some(rule_index);
    }

    for (pos, &ch) in input.iter().enumerate() {
        if let Some(&next_state) = transitions.get(&(current_state, ch)) {
            current_state = next_state;
            
            if let Some(&rule_index) = accepting_states.get(&current_state) {
                last_accepting_pos = pos + 1;
                last_accepting_rule = Some(rule_index);
            }
        } else {
            break;
        }
    }

    (last_accepting_pos, last_accepting_rule)
}
