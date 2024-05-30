#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use makeit::parse::parser::Parser;

    #[test]
    fn paren_test() {
        let mut input = "{{b==(c?\"hello\":\"test\")}}
{{ \"hello\" == (b ? \"hello\" : \"what\") }}"
            .chars()
            .map(Ok);
        let mut vars = HashMap::new();
        vars.insert("b".to_string(), "test".to_string());
        vars.insert("c".to_string(), "test".to_string());

        let mut result = String::new();
        _ = Parser::string(&mut input, &vars, &mut result);
        assert_eq!(result, "false\ntrue");
    }
}
