#[derive(Debug, PartialEq)]
enum Token {
    LPAREN,
    RPAREN,
    INTEGER(i32),
    FLOAT(f32),
    CHAR(char),
    STRING(String),
}

fn tokenize(code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in code.chars() {
        match c {}
    }
}

// fn parse(tokens: Vec<Token>) -> Obj {
// }
//
// fn eval(root: Obj, eval: Obj) -> Obj {
//
// }

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_atom_integer() {
        assert_eq!(tokenize("1".to_string()), vec![Token::INTEGER(1)]);
        assert_eq!(tokenize("12345".to_string()), vec![Token::INTEGER(12345)]);
    }
    #[test]
    fn tokenize_atom_float() {
        assert_eq!(tokenize("0.123".to_string()), vec![Token::FLOAT(0.123)]);
        assert_eq!(tokenize("12.0".to_string()), vec![Token::FLOAT(12.0)]);
    }
    #[test]
    fn tokenize_atom_char() {
        assert_eq!(tokenize("#\\a".to_string()), vec![Token::CHAR('a')]);
        assert_eq!(tokenize("#\\z".to_string()), vec![Token::CHAR('z')]);
        assert_eq!(tokenize("#\\A".to_string()), vec![Token::CHAR('A')]);
        assert_eq!(tokenize("#\\Z".to_string()), vec![Token::CHAR('Z')]);
    }
    #[test]
    fn tokenize_atom_string() {
        assert_eq!(tokenize("\"a\"".to_string()), vec![Token::STRING("a".to_string())]);
        assert_eq!(tokenize("\"hello\"".to_string()), vec![Token::STRING("hello".to_string())]);
    }
    #[test]
    fn tokenize_emptylist() {
        assert_eq!(tokenize("()".to_string()), vec![Token::LPAREN, Token::RPAREN]);
    }
}
