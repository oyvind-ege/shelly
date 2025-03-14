pub fn parse_args(str: &str) -> Vec<String> {
    // let mut tick_stack: Vec<u8> = vec![];
    let mut parsed: Vec<u8> = vec![];
    let mut double_quote_state = false;
    let mut single_quote_state = false;
    let mut escaped: bool = false;

    static SINGLE_QUOTE: u8 = b'\'';
    static DOUBLE_QUOTE: u8 = b'\"';

    static SPECIAL_CHARS: [u8; 4] = [b'`', b'\\', b'\"', b'$'];

    let mut result: Vec<String> = vec![];

    println!("Str: {str:?}");

    for char in str.bytes() {
        if double_quote_state {
            if char == DOUBLE_QUOTE && !escaped {
                double_quote_state = false;
                result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
                parsed.clear();
                continue;
            } else if char == b'\\' && !escaped {
                escaped = true;
                continue;
            } else {
                parsed.push(char);
                continue;
            }
        } else if single_quote_state {
            if char == SINGLE_QUOTE {
                single_quote_state = false;
                result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
                parsed.clear();
                continue;
            } else {
                parsed.push(char);
                continue;
            }
        } else {
            match char {
                b' ' if !parsed.is_empty() => {
                    result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
                    parsed.clear();
                    continue;
                }
                b'\'' => single_quote_state = true,
                b'\"' => double_quote_state = true,
                _ => parsed.push(char),
            }
        }
        /* match; char {
            b'\'' => match tick_stack.last() {
                Some(b'\"') => parsed.push(char),
                Some(b'\'') if str[index + 1..].contains("\'") => tick_stack.push(char),
                Some(b'\'') => {
                    tick_stack.pop();
                    continue;
                }
                _ => tick_stack.push(char),
            },
            b'\"' => match tick_stack.last() {
                Some(b'\"') if str[index + 1..].contains("\"") => tick_stack.push(char),
                Some(b'\"') => {
                    tick_stack.pop();
                    continue;
                }
                _ => parsed.push(char),
            },
            b' ' => {
                if tick_stack.last().is_some() {
                    parsed.push(char);
                } else if !parsed.is_empty() {
                    result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
                    parsed.clear();
                }
            }
            _ => parsed.push(char),
        } */
    }
    /* if !parsed.is_empty() {
        result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encountered."));
        parsed.clear();
    } */
    println!("Result:  {result:?}");
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
    fn test_double_quotes_with_internal() {
        let input = "\"hellooo'\"";
        assert_eq!(vec!["hellooo'"], parse_args(input));
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
    fn test_lone_double_quote_without_escape() {
        let input = "\"hello";
        assert_eq!(vec!["hello"], parse_args(input));
    }

    #[test]
    fn test_lone_double_quote_at_end() {
        let input = "hello\"";
        assert_eq!(vec!["hello"], parse_args(input));
    }
}
