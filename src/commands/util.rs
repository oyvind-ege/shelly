pub fn parse_quotes(str: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_with_word_outside() {
        let input = "'hellooooo    '      test";
        assert_eq!("hellooooo     test", parse_quotes(input));
    }

    #[test]
    fn test_basic() {
        let input = "'hello'";
        assert_eq!("hello", parse_quotes(input));
    }

    #[test]
    fn test_basic_two_words() {
        let input = "'hello world'";
        assert_eq!("hello world", parse_quotes(input));
    }

    #[test]
    fn test_single_with_spaces() {
        let input = "'hello   '";
        assert_eq!("hello   ", parse_quotes(input));
    }

    #[test]
    fn test_multiple_without_space() {
        let input = "'hello's";
        assert_eq!("hellos", parse_quotes(input));
    }

    #[test]
    fn test_multiple() {
        let input = "'hello  's'test  t'";
        assert_eq!("hello   stest  t", parse_quotes(input));
    }

    #[test]
    fn test_multiple_with_spaces() {
        let input = "'hellooo' s 'again   t'";
        assert_eq!("hello s again   t", parse_quotes(input));
    }

    //  In a real shell, an odd number of single quotes would prompt the user for an input. Here, I just ignore it.
    #[test]
    fn test_odd_number_of_ticks() {
        let input = "'hellooo' s 'again   t";
        assert_eq!("hello s again t", parse_quotes(input));
    }
}
