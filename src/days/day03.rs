use crate::aoc_solution;
use crate::util::aoc::Solution;

aoc_solution!(3, part_1, part_2);

#[derive(Debug)]
enum Command {
    Multiply(i64, i64),
    Do,
    Dont,
}

#[derive(Debug, Clone)]
enum TokenType {
    Number(i64),
    Identifier(String),
    Punctuation(char),
}

struct StateMachineTokenizer {
    input: Vec<char>,
    position: usize,
}

impl StateMachineTokenizer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn read_number(&mut self) -> TokenType {
        let mut number = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        TokenType::Number(number.parse().unwrap())
    }

    fn read_identifier(&mut self) -> TokenType {
        let mut identifier = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        TokenType::Identifier(identifier)
    }

    fn read_punctuation(&mut self) -> TokenType {
        let ch = self.current_char().unwrap();
        self.advance();
        TokenType::Punctuation(ch)
    }

    fn tokenize(&mut self) -> Vec<TokenType> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next() {
                tokens.push(token);
            }
        }
        tokens
    }
}

impl Iterator for StateMachineTokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_char()? {
            '0'..='9' => Some(self.read_number()),
            'a'..='z' | '_' | '\'' => Some(self.read_identifier()),
            '(' | ')' | ',' => Some(self.read_punctuation()),
            _ => {
                self.advance();
                self.next()
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut tokenizer = StateMachineTokenizer::new(input);
    let tokens = tokenizer.tokenize();

    let mut i = 0;
    while i < tokens.len() {
        if let (
            Some(TokenType::Identifier(name)),
            Some(TokenType::Punctuation('(')),
            Some(TokenType::Punctuation(')')),
        ) = (tokens.get(i), tokens.get(i + 1), tokens.get(i + 2))
        {
            match name.as_str() {
                n if n.ends_with("do") => {
                    commands.push(Command::Do);
                    i += 3;
                    continue;
                }
                n if n.ends_with("don't") => {
                    commands.push(Command::Dont);
                    i += 3;
                    continue;
                }
                _ => {}
            }
        }
        if let (
            Some(TokenType::Identifier(name)),
            Some(TokenType::Punctuation('(')),
            Some(TokenType::Number(x)),
            Some(TokenType::Punctuation(',')),
            Some(TokenType::Number(y)),
            Some(TokenType::Punctuation(')')),
        ) = (
            tokens.get(i),
            tokens.get(i + 1),
            tokens.get(i + 2),
            tokens.get(i + 3),
            tokens.get(i + 4),
            tokens.get(i + 5),
        ) {
            if name.ends_with("mul") {
                commands.push(Command::Multiply(*x, *y));
                i += 6;
            } else {
                i += 1
            }
        } else {
            i += 1
        }
    }
    commands
}

pub fn part_1(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut sum = 0;
    for command in commands {
        match command {
            Command::Multiply(x, y) => {
                sum += x * y;
            }
            _ => {}
        }
    }
    sum
}
pub fn part_2(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut is_enabled = true;
    let mut sum = 0;
    for command in commands {
        match command {
            Command::Multiply(x, y) => {
                if is_enabled {
                    sum += x * y;
                }
            }
            Command::Do => {
                is_enabled = true;
            }
            Command::Dont => {
                is_enabled = false;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT1), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT2), 48);
    }

    #[test]
    fn test_tokenizer() {
        let mut tokenizer = StateMachineTokenizer::new("mul(2,4)");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 6);
        match &tokens[0] {
            TokenType::Identifier(name) => assert_eq!(name, "mul"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_parse_commands_concise() {
        let commands = parse_input("mul(2,4)test mul(5,5)do()don't()");
        assert_eq!(commands.len(), 4);

        if let Command::Multiply(x, y) = &commands[0] {
            assert_eq!(*x, 2);
            assert_eq!(*y, 4);
        } else {
            panic!("Expected Multiply command");
        }

        if let Command::Multiply(x, y) = &commands[1] {
            assert_eq!(*x, 5);
            assert_eq!(*y, 5);
        } else {
            panic!("Expected Multiply command");
        }

        assert!(matches!(commands[2], Command::Do));
        assert!(matches!(commands[3], Command::Dont));
    }

    // Test specifically for corrupted commands
    #[test]
    fn test_corrupted_commands() {
        let commands = parse_input("xmul(1,2)some_do()trash_don't()");
        assert_eq!(commands.len(), 3);

        assert!(matches!(commands[0], Command::Multiply(1, 2)));
        assert!(matches!(commands[1], Command::Do));
        assert!(matches!(commands[2], Command::Dont));
    }
}
