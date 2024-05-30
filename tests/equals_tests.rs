#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use makeit::parse::parser::Parser;

    #[test]
    fn string_equals() {
        let mut input = "{{ \"test\" == \"test\" }}
{{ \"test\" == \"hello\" }}"
            .chars()
            .map(Ok);
        let vars = HashMap::new();

        let mut result = String::new();
        let mut parser = Parser::string(&mut input, &vars, &mut result);
        _ = parser.parse();
        assert_eq!(result, "true\nfalse");
    }

    #[test]
    fn var_equals() {
        let mut input = "{{ a == b }}
{{ a == c }}
{{ a == \"hello\" }}
{{ b == \"test\" }}
{{ \"test\" == c }}"
            .chars()
            .map(Ok);
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "hello".to_string());
        vars.insert("b".to_string(), "hello".to_string());
        vars.insert("c".to_string(), "test".to_string());

        let mut result = String::new();
        let mut parser = Parser::string(&mut input, &vars, &mut result);
        _ = parser.parse();
        assert_eq!(result, "true\nfalse\ntrue\nfalse\ntrue");
    }
}
