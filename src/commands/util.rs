enum ParseState {
    Normal,
    SingleQuote,
    DoubleQuote,
}

pub fn parse_args(str: &str) -> Vec<String> {
    if str.bytes().all(|b| b == b' ') {
        return vec![];
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
    const ENDLINE: u8 = b'$';

    static SPECIAL_CHARS: [u8; 4] = [GRAVE, BACKSPACE, DOUBLE_QUOTE, ENDLINE];

    let mut result: Vec<String> = vec![];
    for (index, char) in str.bytes().enumerate() {
        if is_word_done {
            result.push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encounted."));
            parsed_buffer.clear();
            is_word_done = false;
        }

        match parse_state {
            ParseState::DoubleQuote => {
                if escaped {
                    if !SPECIAL_CHARS.contains(&char) {
                        parsed_buffer.push(b'\\');
                    }
                    parsed_buffer.push(char);
                    escaped = false;
                } else {
                    match char {
                        DOUBLE_QUOTE => parse_state = ParseState::Normal,
                        b'\\' => escaped = true,
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
                WHITESPACE if !parsed_buffer.is_empty() => {
                    is_word_done = true;
                }
                WHITESPACE if !result.is_empty() => continue,
                BACKSPACE => escaped = true,
                SINGLE_QUOTE if str[index + 1..].contains("\'") => {
                    parse_state = ParseState::SingleQuote;
                }
                SINGLE_QUOTE => continue,
                DOUBLE_QUOTE if str[index + 1..].contains("\"") => {
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
        result.push(String::from_utf8(parsed_buffer.clone()).expect("Non-UTF8 encountered."));
        parsed_buffer.clear();
    }
    result
}

#[cfg(test)]
mod without_quotes {
    use super::*;

    #[test]
    fn backslash() {
        let input = r#"test\ \ \ "#;
        assert_eq!(vec!["test   "], parse_args(input));
    }

    #[test]
    fn backslash_and_other_stuff() {
        let input = r#"test\'\'\'"#;
        assert_eq!(vec!["test'''"], parse_args(input));
    }
}

#[cfg(test)]
mod single_quotes {

    use super::*;

    #[test]
    fn with_word_outside() {
        let input = "'hellooooo    '      test";
        assert_eq!(vec!["hellooooo    ", "test"], parse_args(input));
    }

    #[test]
    fn double_quote_successive() {
        let input = "'hello''test'";
        assert_eq!(vec!["hellotest"], parse_args(input));
    }

    #[test]
    fn basic() {
        let input = "'hello'";
        assert_eq!(vec!["hello"], parse_args(input));
    }

    #[test]
    fn basic_two_words() {
        let input = "'hello world'";
        assert_eq!(vec!["hello world"], parse_args(input));
    }

    #[test]
    fn single_with_spaces() {
        let input = "'hello   '";
        assert_eq!(vec!["hello   "], parse_args(input));
    }

    #[test]
    fn multiple_without_space() {
        let input = "'hello's";
        assert_eq!(vec!["hellos"], parse_args(input));
    }

    #[test]
    fn multiple() {
        let input = "'hello  's'test  t'";
        assert_eq!(vec!["hello  stest  t"], parse_args(input));
    }

    #[test]
    fn multiple_with_spaces() {
        let input = "'hellooo' s 'again   t'";
        assert_eq!(vec!["hellooo", "s", "again   t"], parse_args(input));
    }

    //  In a real shell, an odd number of single quotes could prompt the user for an input. Here, I just ignore it.
    #[test]
    fn odd_number_of_ticks() {
        let input = "'hellooo' s 'again   t";
        assert_eq!(vec!["hellooo", "s", "again", "t"], parse_args(input));
    }
}

#[cfg(test)]
mod double_quotes {
    use super::*;

    #[test]
    fn simple_double_quotes() {
        let input = r#""\"hellooo\"""#;
        assert_eq!(vec![r#""hellooo""#], parse_args(input));
    }

    #[test]
    fn with_spacing() {
        let input = r#""\"hellooo test\"""#;
        assert_eq!(vec![r#""hellooo test""#], parse_args(input));
    }

    #[test]
    fn with_single_quote() {
        let input = r#""\"hellooo' test\"""#;
        assert_eq!(vec![r#""hellooo' test""#], parse_args(input));
    }

    #[test]
    fn with_special_character_backslash() {
        let input = r#""\"hellooo \\ test\"""#;
        assert_eq!(vec![r#""hellooo \ test""#], parse_args(input));
    }

    #[test]
    fn with_backslash_non_escaped() {
        let input = r#""\"hellooo \' test\"""#;
        assert_eq!(vec![r#""hellooo \' test""#], parse_args(input));
    }
}
