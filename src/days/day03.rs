struct Command {
    command: CommandType,
    x: i64,
    y: i64
}

enum CommandType{
    Mult
}

#[derive(Debug, Clone)]
enum LexerState {
    Start,
    InNumber,
    InIdentifier,
    InString,
    InComment,
}

pub struct StateMachineTokenizer {
    input: Vec<char>,
    position: usize,
    state: LexerState,
    current_token: String,
}

impl StateMachineTokenizer {
    fn new(input: &str) -> Self {
        Self {
            input,
            state: LexerState::Start,
            position: 0,
            current_token
        }
    }

    fn tokenize (&mut self) {
       while self.next() {}
    }
}

impl Iterator for Tokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_char()? {
            '0'..='9' => Some(self.read_number()),
            'a'..='z' | '_' => Some(self.read_identifier()),
            '(' | ')' | ',' => {
                let ch = self.current_char().unwrap();
                self.advance();
                Some(TokenType::Punctuation(ch))
            }
            _ => None,
        }
    }
}

fn parse_input (input: &str) -> Vec<Command> {

}

fn part_1(input: &str) -> i64 {
    42
}

#[cfg(test)]
mod test {
    use super::*;

    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(test_input), 42);
    }
}
