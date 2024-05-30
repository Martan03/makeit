#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use makeit::parse::parser::Parser;

    #[test]
    fn plus_test() {
        let mut input = "{{ a + \" \" + b == \"hello world\" }}
{{ \"this \" + b + \" \" + (c ?? \"crazy\") }}"
            .chars()
            .map(Ok);
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "hello".to_string());
        vars.insert("b".to_string(), "world".to_string());

        let mut result = String::new();
        let mut parser = Parser::string(&mut input, &vars, &mut result);
        _ = parser.parse();
        assert_eq!(result, "true\nthis world crazy");
    }
}
