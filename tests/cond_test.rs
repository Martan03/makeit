#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use makeit::parse::parser::Parser;

    #[test]
    fn null_check_tests() {
        let mut input = "{{ a ? \"a not null\" : \"a null\" }}
{{ b == \"hello\" ? \"hello b\" : \"what\" }}
{{ test ? \"test not null\" : \"test null\" }}
{{ a == b ? \"equal\" : \"not equal\" }}
{{ b == c ? \"equal\" : \"not equal\" }}"
            .chars()
            .map(Ok);
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "hello".to_string());
        vars.insert("b".to_string(), "test".to_string());
        vars.insert("c".to_string(), "test".to_string());

        let mut result = String::new();
        let mut parser = Parser::string(&mut input, &vars, &mut result);
        _ = parser.parse();
        assert_eq!(result, "a not null\nwhat\ntest null\nnot equal\nequal");
    }
}
