pub fn parse_args(str: &str) -> Vec<String> {
    if str.bytes().all(|b| b == b' ') {
        return vec![];
    }

    let mut parsed: Vec<u8> = vec![];
    let mut double_quote_state = false;
    let mut single_quote_state = false;
    let mut escaped: bool = false;
    let mut done = false;
    static SINGLE_QUOTE: u8 = b'\'';
    static DOUBLE_QUOTE: u8 = b'\"';

    static SPECIAL_CHARS: [u8; 4] = [b'`', b'\\', b'\"', b'$'];

    let mut result: Vec<String> = vec![];
    for (index, char) in str.bytes().enumerate() {
        if done {
            result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encounted."));
            parsed.clear();
            done = false;
        }

        if double_quote_state {
            if escaped {
                if SPECIAL_CHARS.contains(&char) {
                    parsed.push(char);
                } else {
                    parsed.push(b'\\');
                    parsed.push(char);
                }
                escaped = false;
            } else if char == DOUBLE_QUOTE {
                double_quote_state = false;
            } else if char == b'\\' {
                escaped = true;
            } else {
                parsed.push(char);
            }
        } else if single_quote_state {
            if char == SINGLE_QUOTE {
                single_quote_state = false;
            } else {
                parsed.push(char);
            }
        } else {
            match char {
                _ if escaped => {
                    parsed.push(char);
                    escaped = false;
                }
                b' ' if !parsed.is_empty() => {
                    done = true;
                }
                b'\\' => escaped = true,
                b' ' if !result.is_empty() => continue,
                b'\'' if str[index + 1..].contains("\'") => {
                    single_quote_state = true;
                }
                b'\'' => continue,
                b'\"' if str[index + 1..].contains("\"") => {
                    double_quote_state = true;
                }
                b'\"' => continue,
                _ => {
                    parsed.push(char);
                }
            }
        }
    }
    if !parsed.is_empty() {
        result.push(String::from_utf8(parsed.clone()).expect("Non-UTF8 encountered."));
        parsed.clear();
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
