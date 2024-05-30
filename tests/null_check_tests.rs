#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use makeit::parse::parser::Parser;

    #[test]
    fn null_check_tests() {
        let mut input = "{{ a ?? \"a null\" }}
{{ b ?? \"b null\" }}
{{ test ?? \"test null\" }}
{{ hello ?? c }}
{{ name ?? test ?? b }}
{{ name ?? a ?? \"ops\" }}"
            .chars()
            .map(Ok);
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "a not null".to_string());
        vars.insert("b".to_string(), "behave".to_string());
        vars.insert("c".to_string(), "test".to_string());

        let mut result = String::new();
        let mut parser = Parser::string(&mut input, &vars, &mut result);
        _ = parser.parse();
        assert_eq!(
            result,
            "a not null\nbehave\ntest null\ntest\nbehave\na not null"
        );
    }
}
