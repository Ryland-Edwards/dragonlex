#[derive(Debug, Clone)]
pub enum Action {
    Skip,
    Error(String),
    Token { name: String, keep_lexeme: bool },
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub regex: String,
    pub action: Action,
}

#[derive(Debug)]
pub struct Spec {
    pub rules: Vec<Rule>,
}

pub fn parse_spec(content: &str) -> Result<Spec, String> {
    let mut rules = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let rule = parse_rule(line, line_num + 1)?;
        rules.push(rule);
    }

    Ok(Spec { rules })
}

fn parse_rule(line: &str, line_num: usize) -> Result<Rule, String> {
    // Find the last space to split regex from action
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() < 2 {
        return Err(format!("Line {}: Invalid rule format", line_num));
    }

    let regex = parts[0].to_string();
    let action_str = parts[1];

    let action = parse_action(action_str, line_num)?;

    Ok(Rule { regex, action })
}

fn parse_action(action_str: &str, line_num: usize) -> Result<Action, String> {
    let action_str = action_str.trim();

    if action_str == "(SKIP)" {
        return Ok(Action::Skip);
    }

    if action_str.starts_with("(ERR)") {
        let err_part = action_str.strip_prefix("(ERR)").unwrap().trim();
        return if err_part.starts_with('"') && err_part.ends_with('"') {
            let message = err_part[1..err_part.len() - 1].to_string();
            Ok(Action::Error(message))
        } else {
            Err(format!("Line {}: Error action must have quoted message", line_num))
        }
    }

    // Parse token action: <token> <keep>
    let parts: Vec<&str> = action_str.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(format!("Line {}: Invalid action format", line_num));
    }

    let token_name = parts[0].to_string();
    let keep_str = parts[1];

    let keep_lexeme = match keep_str {
        "true" => true,
        "false" => false,
        _ => return Err(format!("Line {}: Keep value must be 'true' or 'false'", line_num)),
    };

    Ok(Action::Token {
        name: token_name,
        keep_lexeme,
    })
}
