pub fn trim_single_quotes(str: &str) -> &str {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_single_quotes_simple() {
        let expected = "hello world";
        assert_eq!(expected, trim_single_quotes("'hello world'"));
    }

    #[test]
    fn test_embedded_trim() {
        let expected = " ";
        assert_eq!(expected, trim_single_quotes("' ''"))
    }
}
