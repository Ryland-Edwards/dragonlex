#[derive(Debug, Clone, PartialEq)]
pub enum RegexNode {
    Char(char),
    Dot,
    Concatenation(Box<RegexNode>, Box<RegexNode>),
    Alternation(Box<RegexNode>, Box<RegexNode>),
    Kleene(Box<RegexNode>),
    Plus(Box<RegexNode>),
    Optional(Box<RegexNode>),
    CharClass(Vec<char>),
    NegatedCharClass(Vec<char>),
}

pub fn parse_regex(regex: &str) -> Result<RegexNode, String> {
    let mut parser = RegexParser::new(regex);
    parser.parse_alternation()
}

struct RegexParser {
    chars: Vec<char>,
    pos: usize,
}

impl RegexParser {
    fn new(regex: &str) -> Self {
        Self {
            chars: regex.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current();
        self.pos += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn parse_alternation(&mut self) -> Result<RegexNode, String> {
        let mut left = self.parse_concatenation()?;

        while self.current() == Some('|') {
            self.advance(); // consume '|'
            let right = self.parse_concatenation()?;
            left = RegexNode::Alternation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_concatenation(&mut self) -> Result<RegexNode, String> {
        let mut nodes = Vec::new();

        while let Some(ch) = self.current() {
            if ch == '|' || ch == ')' {
                break;
            }
            nodes.push(self.parse_postfix()?);
        }

        if nodes.is_empty() {
            return Err("Empty concatenation".to_string());
        }

        let mut iter = nodes.into_iter();
        let mut result = iter.next().unwrap();
        for node in iter {
            result = RegexNode::Concatenation(Box::new(result), Box::new(node));
        }

        Ok(result)
    }

    fn parse_postfix(&mut self) -> Result<RegexNode, String> {
        let mut node = self.parse_primary()?;

        while let Some(ch) = self.current() {
            match ch {
                '*' => {
                    self.advance();
                    node = RegexNode::Kleene(Box::new(node));
                }
                '+' => {
                    self.advance();
                    node = RegexNode::Plus(Box::new(node));
                }
                '?' => {
                    self.advance();
                    node = RegexNode::Optional(Box::new(node));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<RegexNode, String> {
        match self.current() {
            Some('(') => {
                self.advance(); // consume '('
                let node = self.parse_alternation()?;
                if self.current() != Some(')') {
                    return Err("Missing closing parenthesis".to_string());
                }
                self.advance(); // consume ')'
                Ok(node)
            }
            Some('[') => self.parse_char_class(),
            Some('.') => {
                self.advance();
                Ok(RegexNode::Dot)
            }
            Some('\\') => self.parse_escape(),
            Some(ch) if ch != '|' && ch != ')' && ch != '*' && ch != '+' && ch != '?' => {
                self.advance();
                Ok(RegexNode::Char(ch))
            }
            Some(ch) => Err(format!("Unexpected character: {}", ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_escape(&mut self) -> Result<RegexNode, String> {
        self.advance(); // consume '\'
        match self.current() {
            Some('t') => {
                self.advance();
                Ok(RegexNode::Char('\t'))
            }
            Some('n') => {
                self.advance();
                Ok(RegexNode::Char('\n'))
            }
            Some('_') => {
                self.advance();
                Ok(RegexNode::Char(' '))
            }
            Some('"') => {
                self.advance();
                Ok(RegexNode::Char('"'))
            }
            Some('\'') => {
                self.advance();
                Ok(RegexNode::Char('\''))
            }
            Some('\\') => {
                self.advance();
                Ok(RegexNode::Char('\\'))
            }
            Some(ch) => {
                self.advance();
                Ok(RegexNode::Char(ch))
            }
            None => Err("Incomplete escape sequence".to_string()),
        }
    }

    fn parse_char_class(&mut self) -> Result<RegexNode, String> {
        self.advance(); // consume '['

        let negated = if self.current() == Some('^') {
            self.advance();
            true
        } else {
            false
        };

        let mut chars = Vec::new();

        while let Some(ch) = self.current() {
            if ch == ']' {
                self.advance();
                break;
            }

            if ch == '-' && !chars.is_empty() && self.peek().is_some() && self.peek() != Some(']') {
                // Range
                self.advance(); // consume '-'
                let end_char = self.advance().unwrap();
                let start_char = chars.pop().unwrap();

                for c in (start_char as u8)..=(end_char as u8) {
                    chars.push(c as char);
                }
            } else {
                chars.push(ch);
                self.advance();
            }
        }

        if negated {
            Ok(RegexNode::NegatedCharClass(chars))
        } else {
            Ok(RegexNode::CharClass(chars))
        }
    }
}
