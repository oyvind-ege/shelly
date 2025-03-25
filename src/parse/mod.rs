use std::fs::File;
use std::io::{self, Error, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CommandOptions {
    pub cmd: Option<String>,
    pub args: Option<Vec<String>>,
    output: Option<PathBuf>,
    errorout: Option<PathBuf>,
}

impl CommandOptions {
    pub fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        match self.output {
            Some(ref path) => File::open(path).map(|f| Box::new(f) as Box<dyn Write>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}

enum ParseState {
    Normal,
    SingleQuote,
    DoubleQuote,
    PotentialRedirect,
    Redirect,
}

pub fn parse_input(input: &str) -> CommandOptions {
    if input.bytes().all(|b| b == WHITESPACE) {
        return CommandOptions::default();
    }

    let mut parsed_buffer: Vec<u8> = vec![];
    let mut parse_state: ParseState = ParseState::Normal;
    let mut escaped: bool = false;
    let mut is_word_done = false;

    const SINGLE_QUOTE: u8 = b'\'';
    const DOUBLE_QUOTE: u8 = b'\"';
    const WHITESPACE: u8 = b' ';
    const BACKSPACE: u8 = b'\\';
    const GRAVE: u8 = b'`';
    const DOLLAR_SIGN: u8 = b'$';
    const REDIRECT: u8 = b'>';

    static SPECIAL_CHARS: [u8; 4] = [GRAVE, BACKSPACE, DOUBLE_QUOTE, DOLLAR_SIGN];

    let mut final_parsed_input: Vec<String> = vec![];
    let mut parsed_redirect: Vec<String> = vec![];

    for (index, char) in input.bytes().enumerate() {
        if is_word_done {
            if let ParseState::Redirect = parse_state {
                parsed_redirect
                    .push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encountered."));
                parse_state = ParseState::Normal;
            } else {
                final_parsed_input
                    .push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encountered."));
            }
            parsed_buffer.clear();
            is_word_done = false;
        }

        match parse_state {
            ParseState::PotentialRedirect => match char {
                REDIRECT => {
                    // Since this was truly a redirect, we pop the number from the parsed buffer
                    parsed_buffer.pop();
                    parse_state = ParseState::Redirect
                }
                _ => parse_state = ParseState::Normal,
            },
            ParseState::Redirect => match char {
                WHITESPACE if !parsed_buffer.is_empty() => is_word_done = true,
                WHITESPACE => continue,
                _ => parsed_buffer.push(char),
            },
            ParseState::DoubleQuote => {
                if escaped {
                    if !SPECIAL_CHARS.contains(&char) {
                        // Since the previous char was backspace, and the current char is not special, we add backspace.
                        parsed_buffer.push(BACKSPACE);
                    }
                    // We add the current char, whether it is special or not.
                    parsed_buffer.push(char);
                    escaped = false;
                } else {
                    match char {
                        // We exit DoubleQuote state if new char is double quote; enter escaped state if current char is backspace.
                        DOUBLE_QUOTE => parse_state = ParseState::Normal,
                        BACKSPACE => escaped = true,
                        _ => parsed_buffer.push(char),
                    }
                }
            }
            ParseState::SingleQuote => {
                if char == SINGLE_QUOTE {
                    parse_state = ParseState::Normal;
                } else {
                    parsed_buffer.push(char);
                }
            }
            ParseState::Normal => match char {
                _ if escaped => {
                    parsed_buffer.push(char);
                    escaped = false;
                }
                b'1' | b'2' => {
                    // We will later pop the char if this is truly a redirect (will be known at next iteration)
                    parsed_buffer.push(char);
                    parse_state = ParseState::PotentialRedirect;
                }
                REDIRECT => parse_state = ParseState::Redirect,
                WHITESPACE if !parsed_buffer.is_empty() => {
                    is_word_done = true;
                }
                WHITESPACE if !final_parsed_input.is_empty() => continue,
                BACKSPACE => escaped = true,
                SINGLE_QUOTE if input[index + 1..].contains("\'") => {
                    parse_state = ParseState::SingleQuote;
                }
                SINGLE_QUOTE => continue,
                DOUBLE_QUOTE if input[index + 1..].contains("\"") => {
                    parse_state = ParseState::DoubleQuote;
                }
                DOUBLE_QUOTE => continue,
                _ => {
                    parsed_buffer.push(char);
                }
            },
        }
    }

    if !parsed_buffer.is_empty() {
        match parse_state {
            ParseState::Redirect => parsed_redirect
                .push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encountered.")),
            _ => final_parsed_input
                .push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encountered.")),
        }
        parsed_buffer.clear();
    }

    CommandOptions {
        cmd: Some(final_parsed_input[0].clone()),
        args: if final_parsed_input.len() > 1 {
            Some(final_parsed_input[1..].to_vec())
        } else {
            None
        },
        output: if parsed_redirect.is_empty() {
            None
        } else {
            Some(PathBuf::from(parsed_redirect[0].clone().trim()))
        },
        errorout: None,
    }
}

#[cfg(test)]
mod without_quotes {
    use super::*;

    #[test]
    fn and_with_backslash() {
        let input = r#"test\ \ \ "#;
        let result = parse_input(input);
        let expected = "test   ".to_string();

        assert_eq!(
            result.cmd.expect("Cmd not found: {input}, {result.cmd}"),
            expected,
        );
    }

    #[test]
    fn and_with_backslash_and_other_stuff() {
        let input = r#"test\'\'\'"#;
        let result = parse_input(input);
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
        let input = "'hellooooo    '      test";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hellooooo    ".to_string()),
            args: Some(vec!["test".to_string()]),
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd_and_separately_quoted_arg() {
        let input = "'hello''test'";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hellotest".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd() {
        let input = "'hello'";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hello".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_two_word_cmd() {
        let input = "'hello world'";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hello world".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn single_quoted_cmd_with_spaces() {
        let input = "'hello   '";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hello   ".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn quoted_cmd_immediately_followed_by_char() {
        let input = "'hello's";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hellos".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_quoted_and_unquoted_should_lead_to_cmd() {
        let input = "'hello  's'test  t'";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("hello  stest  t".to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_with_spaces() {
        let input = "'hellooo' s 'again   t'";
        let result = parse_input(input);
        let expected = CommandOptions {
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
        let input = "'hellooo' s 'again   t";
        let result = parse_input(input);
        let expected = CommandOptions {
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
        let input = r#""\"hellooo\"""#;
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some(r#""hellooo""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_inside_whitespace_leads_to_cmd() {
        let input = r#""\"hellooo test\"""#;
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some(r#""hellooo test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_single_quote_inside() {
        let input = r#""\"hellooo' test\"""#;
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some(r#""hellooo' test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_special_character_backslash() {
        let input = r#""\"hellooo \\ test\"""#;
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some(r#""hellooo \ test""#.to_string()),
            args: None,
            output: None,
            errorout: None,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn with_backslash_non_escaped() {
        let input = r#""\"hellooo \' test\"""#;
        let result = parse_input(input);
        let expected = CommandOptions {
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
        let input = "echo > test.txt";
        let result = parse_input(input);
        let expected = CommandOptions {
            cmd: Some("echo".to_string()),
            args: None,
            output: Some(PathBuf::from("test.txt")),
            errorout: None,
        };
        assert_eq!(result, expected);
    }
}
