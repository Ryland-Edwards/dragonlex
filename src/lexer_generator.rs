use std::fs;
use std::process::Command;
use crate::spec_parser::{Spec, Action};
use crate::regex_parser::parse_regex;
use crate::nfa::NFA;
use crate::dfa::DFA;

pub fn generate_lexer(spec: &Spec) -> Result<(), String> {
    // Build NFAs for each rule
    let mut nfas = Vec::new();

    for (index, rule) in spec.rules.iter().enumerate() {
        let regex_ast = parse_regex(&rule.regex)
            .map_err(|e| format!("Error parsing regex '{}': {}", rule.regex, e))?;

        let nfa = NFA::from_regex(&regex_ast);
        nfas.push((nfa, index));
    }

    // Convert to DFA
    let dfa = DFA::from_nfas(nfas);

    // Generate lexer source code
    let lexer_code = generate_lexer_code(spec, &dfa)?;

    // Write lexer source code
    fs::write("lexer.rs", lexer_code)
        .map_err(|e| format!("Error writing lexer.rs: {}", e))?;

    // Compile the lexer
    let output = Command::new("rustc")
        .args(&["lexer.rs", "-o", "lexer"])
        .output()
        .map_err(|e| format!("Error compiling lexer: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Compilation failed: {}", stderr));
    }

    Ok(())
}

fn generate_lexer_code(spec: &Spec, dfa: &DFA) -> Result<String, String> {
    let mut code = String::new();

    // Add imports and basic structure
    code.push_str("use std::env;\n");
    code.push_str("use std::fs;\n");
    code.push_str("use std::collections::HashMap;\n");
    code.push_str("use std::process;\n\n");

    // Generate DFA transition table
    code.push_str("fn main() {\n");
    code.push_str("    let args: Vec<String> = env::args().collect();\n");
    code.push_str("    if args.len() != 2 {\n");
    code.push_str("        eprintln!(\"Usage: {} <input_file>\", args[0]);\n");
    code.push_str("        process::exit(1);\n");
    code.push_str("    }\n\n");

    code.push_str("    let input_file = &args[1];\n");
    code.push_str("    let input = match fs::read_to_string(input_file) {\n");
    code.push_str("        Ok(content) => content,\n");
    code.push_str("        Err(err) => {\n");
    code.push_str("            eprintln!(\"Error reading input file: {}\", err);\n");
    code.push_str("            process::exit(1);\n");
    code.push_str("        }\n");
    code.push_str("    };\n\n");

    code.push_str("    let tokens = tokenize(&input);\n");
    code.push_str("    for token in tokens {\n");
    code.push_str("        println!(\"{}\", token);\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate tokenize function
    code.push_str("fn tokenize(input: &str) -> Vec<String> {\n");
    code.push_str("    let mut tokens = Vec::new();\n");
    code.push_str("    let mut line = 1;\n");
    code.push_str("    let mut column = 1;\n");
    code.push_str("    let mut pos = 0;\n");
    code.push_str("    let chars: Vec<char> = input.chars().collect();\n\n");

    // Generate transition table
    code.push_str("    let mut transitions = HashMap::new();\n");
    for ((from_state, ch), to_state) in &dfa.transitions {
        code.push_str(&format!(
            "    transitions.insert(({}, '{}'), {});\n",
            from_state.0, escape_char(*ch), to_state.0
        ));
    }
    code.push_str("\n");

    // Generate accepting states
    code.push_str("    let mut accepting_states = HashMap::new();\n");
    for (state_id, state) in &dfa.states {
        if state.is_accepting {
            if let Some(rule_index) = state.rule_index {
                code.push_str(&format!(
                    "    accepting_states.insert({}, {});\n",
                    state_id.0, rule_index
                ));
            }
        }
    }
    code.push_str("\n");

    // Generate rule actions
    code.push_str("    let rules = vec![\n");
    for rule in &spec.rules {
        match &rule.action {
            Action::Skip => {
                code.push_str("        RuleAction::Skip,\n");
            }
            Action::Error(msg) => {
                code.push_str(&format!("        RuleAction::Error(\"{}\".to_string()),\n", escape_string(msg)));
            }
            Action::Token { name, keep_lexeme } => {
                code.push_str(&format!(
                    "        RuleAction::Token {{ name: \"{}\".to_string(), keep_lexeme: {} }},\n",
                    name, keep_lexeme
                ));
            }
        }
    }
    code.push_str("    ];\n\n");

    // Main tokenization loop
    code.push_str("    while pos < chars.len() {\n");
    code.push_str("        let (token_length, rule_index) = longest_match(&chars[pos..], &transitions, &accepting_states);\n\n");

    code.push_str("        if token_length > 0 {\n");
    code.push_str("            let lexeme: String = chars[pos..pos + token_length].iter().collect();\n");
    code.push_str("            \n");
    code.push_str("            if let Some(rule_idx) = rule_index {\n");
    code.push_str("                match &rules[rule_idx] {\n");
    code.push_str("                    RuleAction::Skip => {},\n");
    code.push_str("                    RuleAction::Error(msg) => {\n");
    code.push_str("                        eprintln!(\"{}\", msg);\n");
    code.push_str("                    },\n");
    code.push_str("                    RuleAction::Token { name, keep_lexeme } => {\n");
    code.push_str("                        let token_str = if *keep_lexeme {\n");
    code.push_str("                            format!(\"{}:{} [{},{}]\", name, lexeme, line, column)\n");
    code.push_str("                        } else {\n");
    code.push_str("                            format!(\"{} [{},{}]\", name, line, column)\n");
    code.push_str("                        };\n");
    code.push_str("                        tokens.push(token_str);\n");
    code.push_str("                    },\n");
    code.push_str("                }\n");
    code.push_str("            }\n\n");

    code.push_str("            // Update position\n");
    code.push_str("            for i in pos..pos + token_length {\n");
    code.push_str("                if chars[i] == '\\n' {\n");
    code.push_str("                    line += 1;\n");
    code.push_str("                    column = 1;\n");
    code.push_str("                } else {\n");
    code.push_str("                    column += 1;\n");
    code.push_str("                }\n");
    code.push_str("            }\n");
    code.push_str("            pos += token_length;\n");
    code.push_str("        } else {\n");
    code.push_str("            // No match found, skip character\n");
    code.push_str("            if chars[pos] == '\\n' {\n");
    code.push_str("                line += 1;\n");
    code.push_str("                column = 1;\n");
    code.push_str("            } else {\n");
    code.push_str("                column += 1;\n");
    code.push_str("            }\n");
    code.push_str("            pos += 1;\n");
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    code.push_str("    // Add EOF token\n");
    code.push_str("    tokens.push(format!(\"EOF [{},{}]\", line, column));\n");
    code.push_str("    tokens\n");
    code.push_str("}\n\n");

    // Add helper types and functions
    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("enum RuleAction {\n");
    code.push_str("    Skip,\n");
    code.push_str("    Error(String),\n");
    code.push_str("    Token { name: String, keep_lexeme: bool },\n");
    code.push_str("}\n\n");

    code.push_str("fn longest_match(\n");
    code.push_str("    input: &[char],\n");
    code.push_str("    transitions: &HashMap<(usize, char), usize>,\n");
    code.push_str("    accepting_states: &HashMap<usize, usize>\n");
    code.push_str(") -> (usize, Option<usize>) {\n");
    code.push_str(&format!("    let mut current_state = {};\n", dfa.start_state.0));
    code.push_str("    let mut last_accepting_pos = 0;\n");
    code.push_str("    let mut last_accepting_rule = None;\n\n");

    code.push_str("    // Check if start state is accepting\n");
    code.push_str("    if let Some(&rule_index) = accepting_states.get(&current_state) {\n");
    code.push_str("        last_accepting_pos = 0;\n");
    code.push_str("        last_accepting_rule = Some(rule_index);\n");
    code.push_str("    }\n\n");

    code.push_str("    for (pos, &ch) in input.iter().enumerate() {\n");
    code.push_str("        if let Some(&next_state) = transitions.get(&(current_state, ch)) {\n");
    code.push_str("            current_state = next_state;\n");
    code.push_str("            \n");
    code.push_str("            if let Some(&rule_index) = accepting_states.get(&current_state) {\n");
    code.push_str("                last_accepting_pos = pos + 1;\n");
    code.push_str("                last_accepting_rule = Some(rule_index);\n");
    code.push_str("            }\n");
    code.push_str("        } else {\n");
    code.push_str("            break;\n");
    code.push_str("        }\n");
    code.push_str("    }\n\n");

    code.push_str("    (last_accepting_pos, last_accepting_rule)\n");
    code.push_str("}\n");

    Ok(code)
}

fn escape_char(ch: char) -> String {
    match ch {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '"' => "\\\"".to_string(),
        _ => ch.to_string(),
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}
