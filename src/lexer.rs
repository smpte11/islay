use regex::Regex;

pub fn lex(source: &str) -> Vec<String> {
    let mut expressions = vec![];
    let op_rex = Regex::new(r#"(\+|\*|\\|-)"#).unwrap();
    let literal_rex = Regex::new(r#"(\d|True|False)"#).unwrap();

    for i in source.chars() {
        if op_rex.is_match(&i.to_string()) {
            expressions.push(format!("[op:{}]", i));
        } else if literal_rex.is_match(&i.to_string()) {
            expressions.push(format!("[lit:{}]", i))
        }
    }
    expressions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_operators() {
        assert_eq!(1, lex("+").len());
        assert_eq!(3, lex("1 + 2").len());
        assert_eq!(3, lex("- + 2").len());
    }
}
