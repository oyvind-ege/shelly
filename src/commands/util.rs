pub fn parse_args_from_str_with_quotes(str: &str) -> Vec<String> {
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
mod tests {

    use super::*;

    #[test]
    fn test_with_word_outside() {
        let input = "'hellooooo    '      test";
        assert_eq!(
            vec!["hellooooo    ", "test"],
            parse_args_from_str_with_quotes(input)
        );
    }

    #[test]
    fn test_double_quote_successive() {
        let input = "'hello''test'";
        assert_eq!(vec!["hellotest"], parse_args_from_str_with_quotes(input));
    }

    #[test]
    fn test_basic() {
        let input = "'hello'";
        assert_eq!(vec!["hello"], parse_args_from_str_with_quotes(input));
    }

    #[test]
    fn test_basic_two_words() {
        let input = "'hello world'";
        assert_eq!(vec!["hello world"], parse_args_from_str_with_quotes(input));
    }

    #[test]
    fn test_single_with_spaces() {
        let input = "'hello   '";
        assert_eq!(vec!["hello   "], parse_args_from_str_with_quotes(input));
    }

    #[test]
    fn test_multiple_without_space() {
        let input = "'hello's";
        assert_eq!(vec!["hellos"], parse_args_from_str_with_quotes(input));
    }

    #[test]
    fn test_multiple() {
        let input = "'hello  's'test  t'";
        assert_eq!(
            vec!["hello  stest  t"],
            parse_args_from_str_with_quotes(input)
        );
    }

    #[test]
    fn test_multiple_with_spaces() {
        let input = "'hellooo' s 'again   t'";
        assert_eq!(
            vec!["hellooo", "s", "again   t"],
            parse_args_from_str_with_quotes(input)
        );
    }

    //  In a real shell, an odd number of single quotes could prompt the user for an input. Here, I just ignore it.
    #[test]
    fn test_odd_number_of_ticks() {
        let input = "'hellooo' s 'again   t";
        assert_eq!(
            vec!["hellooo", "s", "again", "t"],
            parse_args_from_str_with_quotes(input)
        );
    }
}
