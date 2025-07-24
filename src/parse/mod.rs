use std::fs::File;
use std::io::{self, Error, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ParsedCommand {
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
    pub output: Option<PathBuf>,
    pub errorout: Option<PathBuf>,
}

impl ParsedCommand {
    pub fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        match self.output {
            Some(ref path) => File::create(path).map(|f| Box::new(f) as Box<dyn Write>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}

// TODO: impl parse_input to this one
#[derive(Default)]
pub struct Parser {
    parsing_data: ParseInfo,
}

#[derive(Default)]
struct ParseInfo {
    parse_state: ParseState,
    parsed_buffer: Vec<u8>,
    parsed_redirect: Vec<String>,
    final_parsed_input: Vec<String>,
    escaped: bool,
    is_word_done: bool,
}

#[derive(Default)]
enum ParseState {
    #[default]
    Normal,
    SingleQuote,
    DoubleQuote,
    PotentialRedirect,
    Redirect,
}

impl Parser {
    const SINGLE_QUOTE: u8 = b'\'';
    const DOUBLE_QUOTE: u8 = b'\"';
    const WHITESPACE: u8 = b' ';
    const BACKSLASH: u8 = b'\\';
    const GRAVE: u8 = b'`';
    const DOLLAR_SIGN: u8 = b'$';
    const REDIRECT: u8 = b'>';

    const SPECIAL_CHARS: [u8; 4] = [
        Self::GRAVE,
        Self::BACKSLASH,
        Self::DOUBLE_QUOTE,
        Self::DOLLAR_SIGN,
    ];

    // This is absolutely atrocious.
    pub fn parse_input(&mut self, input: &str) -> ParsedCommand {
        if input.bytes().all(|b| b == Parser::WHITESPACE) {
            return ParsedCommand::default();
        }

        self.parsing_data = ParseInfo::default();

        for (index, char) in input.bytes().enumerate() {
            if self.parsing_data.is_word_done {
                self.handle_word_done();
            }

            match self.parsing_data.parse_state {
                ParseState::PotentialRedirect => self.handle_potential_redirect(char),
                ParseState::Redirect => self.handle_redirect(char),
                ParseState::DoubleQuote => self.handle_double_quote(char),
                ParseState::SingleQuote => self.handle_single_quote(char),
                ParseState::Normal => self.handle_normal_parse_state(char, input, index),
            }
        }

        if !self.parsing_data.parsed_buffer.is_empty() {
            self.handle_non_empty_parsed_buffer();
        }

        self.get_parse_result()
    }

    fn get_parse_result(&self) -> ParsedCommand {
        ParsedCommand {
            cmd: Some(self.parsing_data.final_parsed_input[0].clone()),
            args: if self.parsing_data.final_parsed_input.len() > 1 {
                Some(self.parsing_data.final_parsed_input[1..].to_vec())
            } else {
                None
            },
            output: if self.parsing_data.parsed_redirect.is_empty() {
                None
            } else {
                Some(PathBuf::from(
                    self.parsing_data.parsed_redirect[0].clone().trim(),
                ))
            },
            errorout: None,
        }
    }

    fn handle_non_empty_parsed_buffer(&mut self) {
        match self.parsing_data.parse_state {
            ParseState::Redirect => self.parsing_data.parsed_redirect.push(
                String::from_utf8(self.parsing_data.parsed_buffer.clone())
                    .expect("Non-UTF8 encountered."),
            ),
            _ => self.parsing_data.final_parsed_input.push(
                String::from_utf8(self.parsing_data.parsed_buffer.clone())
                    .expect("Non-UTF8 encountered."),
            ),
        }
        self.parsing_data.parsed_buffer.clear();
    }

    fn handle_potential_redirect(&mut self, char: u8) {
        match char {
            Parser::REDIRECT => {
                // Since this was truly a redirect, we pop the number from the parsed buffer
                self.parsing_data.parsed_buffer.pop();
                self.parsing_data.parse_state = ParseState::Redirect
            }
            _ => self.parsing_data.parse_state = ParseState::Normal,
        }
    }

    fn handle_single_quote(&mut self, char: u8) {
        if char == Parser::SINGLE_QUOTE {
            self.parsing_data.parse_state = ParseState::Normal;
        } else {
            self.parsing_data.parsed_buffer.push(char);
        }
    }

    fn handle_double_quote(&mut self, char: u8) {
        if self.parsing_data.escaped {
            if !Parser::SPECIAL_CHARS.contains(&char) {
                // Since the previous char was backslash - we know this since we are in Escaped mode -, and the current char is not special, we add an actual backslash.
                self.parsing_data.parsed_buffer.push(Parser::BACKSLASH);
            }
            // We add the current char, whether it is special or not.
            self.parsing_data.parsed_buffer.push(char);
            self.parsing_data.escaped = false;
        } else {
            match char {
                // We exit DoubleQuote state if new char is double quote; enter escaped state if current char is backslash.
                Parser::DOUBLE_QUOTE => self.parsing_data.parse_state = ParseState::Normal,
                Parser::BACKSLASH => self.parsing_data.escaped = true,
                _ => self.parsing_data.parsed_buffer.push(char),
            }
        }
    }

    fn handle_word_done(&mut self) {
        if let ParseState::Redirect = self.parsing_data.parse_state {
            self.parsing_data.parsed_redirect.push(
                String::from_utf8(self.parsing_data.parsed_buffer.clone())
                    .expect("Non-UTF8 encountered."),
            );
            self.parsing_data.parse_state = ParseState::Normal;
        } else {
            self.parsing_data.final_parsed_input.push(
                String::from_utf8(self.parsing_data.parsed_buffer.clone())
                    .expect("Non-UTF8 encountered."),
            );
        }
        self.parsing_data.parsed_buffer.clear();
        self.parsing_data.is_word_done = false;
    }

    fn handle_redirect(&mut self, char: u8) {
        match char {
            Parser::WHITESPACE if !self.parsing_data.parsed_buffer.is_empty() => {
                self.parsing_data.is_word_done = true;
            }

            Parser::WHITESPACE => (),
            _ => {
                self.parsing_data.parsed_buffer.push(char);
            }
        }
    }

    fn handle_normal_parse_state(&mut self, char: u8, input: &str, index: usize) {
        match char {
            _ if self.parsing_data.escaped => {
                self.parsing_data.parsed_buffer.push(char);
                self.parsing_data.escaped = false;
            }
            b'1' | b'2' => {
                // We will later pop the char if this is truly a redirect (will be known at next iteration)
                self.parsing_data.parsed_buffer.push(char);
                self.parsing_data.parse_state = ParseState::PotentialRedirect;
            }
            Parser::REDIRECT => {
                self.parsing_data.parse_state = ParseState::Redirect;
            }
            Parser::WHITESPACE if !self.parsing_data.parsed_buffer.is_empty() => {
                self.parsing_data.is_word_done = true;
            }
            Parser::WHITESPACE if !self.parsing_data.final_parsed_input.is_empty() => (),
            Parser::BACKSLASH => {
                self.parsing_data.escaped = true;
            }
            Parser::SINGLE_QUOTE if input[index + 1..].contains("\'") => {
                self.parsing_data.parse_state = ParseState::SingleQuote;
            }
            Parser::SINGLE_QUOTE => (),
            Parser::DOUBLE_QUOTE if input[index + 1..].contains("\"") => {
                self.parsing_data.parse_state = ParseState::DoubleQuote;
            }
            Parser::DOUBLE_QUOTE => (),
            _ => {
                self.parsing_data.parsed_buffer.push(char);
            }
        }
    }
}

#[cfg(test)]
mod without_quotes {
    use super::*;

    #[test]
    fn and_with_backslash() {
        let mut parser = Parser::default();
        let input = r#"test\ \ \ "#;
        let result = parser.parse_input(input);
        let expected = "test   ".to_string();

        assert_eq!(
            result.cmd.expect("Cmd not found: {input}, {result.cmd}"),
            expected,
        );
    }

    #[test]
    fn and_with_backslash_and_other_stuff() {
        let mut parser = Parser::default();
        let input = r#"test\'\'\'"#;
        let result = parser.parse_input(input);
        let expected = "test'''".to_string();
        assert_eq!(
            result.cmd.expect("Cmd not found: {input}, {result.cmd}"),
            expected
        );
    }
}

#[cfg(test)]
mod single_quotes {

    use super::*;

    #[test]
    fn cmd_single_quoted_with_argument_outside_quotes() {
        let mut parser = Parser::default();
        let input = "'hellooooo    '      test";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hellooooo    ".to_string()),
            args: Some(vec!["test".to_string()]),
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd_and_separately_quoted_arg() {
        let mut parser = Parser::default();
        let input = "'hello''test'";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hellotest".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd() {
        let mut parser = Parser::default();
        let input = "'hello'";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hello".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_two_word_cmd() {
        let mut parser = Parser::default();
        let input = "'hello world'";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hello world".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd_with_spaces() {
        let mut parser = Parser::default();
        let input = "'hello   '";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hello   ".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn quoted_cmd_immediately_followed_by_char() {
        let mut parser = Parser::default();
        let input = "'hello's";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hellos".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_quoted_and_unquoted_should_lead_to_cmd() {
        let mut parser = Parser::default();
        let input = "'hello  's'test  t'";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hello  stest  t".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_with_spaces() {
        let mut parser = Parser::default();
        let input = "'hellooo' s 'again   t'";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hellooo".to_string()),
            args: Some(vec!["s".to_string(), "again   t".to_string()]),
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    //  In a real shell, an odd number of single quotes could prompt the user for input. Here, I just ignore it.
    #[test]
    fn odd_number_of_single_quotes() {
        let mut parser = Parser::default();
        let input = "'hellooo' s 'again   t";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("hellooo".to_string()),
            args: Some(vec!["s".to_string(), "again".to_string(), "t".to_string()]),
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod double_quotes {
    use super::*;

    #[test]
    fn simple_double_quotes() {
        let mut parser = Parser::default();
        let input = r#""\"hellooo\"""#;
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some(r#""hellooo""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_inside_whitespace_leads_to_cmd() {
        let mut parser = Parser::default();
        let input = r#""\"hellooo test\"""#;
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some(r#""hellooo test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_single_quote_inside() {
        let mut parser = Parser::default();
        let input = r#""\"hellooo' test\"""#;
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some(r#""hellooo' test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_special_character_backslash() {
        let mut parser = Parser::default();
        let input = r#""\"hellooo \\ test\"""#;
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some(r#""hellooo \ test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_backslash_non_escaped() {
        let mut parser = Parser::default();
        let input = r#""\"hellooo \' test\"""#;
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some(r#""hellooo \' test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod redirect {
    use super::*;

    #[test]
    fn simple() {
        let mut parser = Parser::default();
        let input = "echo > test.txt";
        let result = parser.parse_input(input);
        let expected = ParsedCommand {
            cmd: Some("echo".to_string()),
            args: None,
            output: Some(PathBuf::from("test.txt")),
            errorout: None,
        };
        assert_eq!(result, expected);
    }
}
