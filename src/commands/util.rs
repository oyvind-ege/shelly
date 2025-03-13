pub fn parse_args(str: &str) -> Vec<String> {
    let mut tick_stack = vec![];
    let mut parsed: Vec<u8> = vec![];

    let mut result: Vec<String> = vec![];
    for (index, char) in str.bytes().enumerate() {
        match char {
            b'\'' => {
                if let Some(b'\'') = tick_stack.last() {
                    tick_stack.pop();
                    continue;
                } else if str[index + 1..].contains("\'") {
                    tick_stack.push(char);
                }
                // Ignoring lone single quote for now.
            }
            b'\"' => {
                if let Some(b'\"') = tick_stack.last() {
                    tick_stack.pop();
                    continue;
                }
            }
            b' ' => {
                if let Some(b'\'') = tick_stack.last() {
                    parsed.push(char);
                } else if !parsed.is_empty() {
                    result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
                    parsed.clear();
                }
            }
            _ => parsed.push(char),
        }
    }
    if !parsed.is_empty() {
        result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encountered."));
        parsed.clear();
    }
    result
}

#[cfg(test)]
mod single_quotes {

    use super::*;

    #[test]
    fn test_with_word_outside() {
        let input = "'hellooooo    '      test";
        assert_eq!(vec!["hellooooo    ", "test"], parse_args(input));
    }

    #[test]
    fn test_double_quote_successive() {
        let input = "'hello''test'";
        assert_eq!(vec!["hellotest"], parse_args(input));
    }

    #[test]
    fn test_basic() {
        let input = "'hello'";
        assert_eq!(vec!["hello"], parse_args(input));
    }

    #[test]
    fn test_basic_two_words() {
        let input = "'hello world'";
        assert_eq!(vec!["hello world"], parse_args(input));
    }

    #[test]
    fn test_single_with_spaces() {
        let input = "'hello   '";
        assert_eq!(vec!["hello   "], parse_args(input));
    }

    #[test]
    fn test_multiple_without_space() {
        let input = "'hello's";
        assert_eq!(vec!["hellos"], parse_args(input));
    }

    #[test]
    fn test_multiple() {
        let input = "'hello  's'test  t'";
        assert_eq!(vec!["hello  stest  t"], parse_args(input));
    }

    #[test]
    fn test_multiple_with_spaces() {
        let input = "'hellooo' s 'again   t'";
        assert_eq!(vec!["hellooo", "s", "again   t"], parse_args(input));
    }

    //  In a real shell, an odd number of single quotes could prompt the user for an input. Here, I just ignore it.
    #[test]
    fn test_odd_number_of_ticks() {
        let input = "'hellooo' s 'again   t";
        assert_eq!(vec!["hellooo", "s", "again", "t"], parse_args(input));
    }
}

#[cfg(test)]
mod double_quotes {
    use super::*;
    #[test]
    fn test_double_quotes() {
        let input = "\"hellooo\"";
        assert_eq!(vec!["hellooo"], parse_args(input));
    }

    #[test]
    fn test_double_quotes_simple_spacing() {
        let input = "\"hello test\"";
        assert_eq!(vec!["hello test"], parse_args(input));
    }

    #[test]
    fn test_double_quot_single_backslash() {
        let input = "\"hello \\ test\"";
        assert_eq!(vec!["hello \\ test"], parse_args(input));
    }

    #[test]
    fn test_double_quote_escaped_backslash() {
        let input = "\"hello '\n' test\"";
        assert_eq!(vec!["hello '\n' test"], parse_args(input));
    }

    #[test]
    fn test_double_quote_backslash_for_single_quote() {
        let input = "\"hello \" test\"";
        assert_eq!(vec!["hello \" test"], parse_args(input));
    }

    #[test]
    fn test_double_quote_backslash_for_escape() {
        let input = r"hello \$ test";
        assert_eq!(vec![r"hello \$ test"], parse_args(input));
    }

    #[test]
    fn test_lone_single_quote_without_escape() {
        let input = "\"hello";
        assert_eq!(vec!["hello"], parse_args(input));
    }
}
