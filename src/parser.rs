use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, one_of, alpha1, alphanumeric1},
    combinator::{map, recognize, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(i32),
    Boolean(bool),
    Operater(BuiltinOp),
    Symbol(String),
}


#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    SelfEvaluation(Atom),
    Application(Box<Expr>, Vec<Expr>),
    Define(String, Box<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, val) = recognize(
        pair(
            opt(one_of("+-")),
            digit1
        )
    )(input)?;
    Ok((input, Atom::Num(val.parse::<i32>().unwrap())))
}

fn parse_bool(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, val) = alt((tag("#t"), tag("#f")))(input)?;
    Ok((
        input,
        match val {
            "#t" => Atom::Boolean(true),
            "#f" => Atom::Boolean(false),
            _ => unreachable!("Invalid input: not boolean"),
        },
    ))
}

fn parse_operater(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, val) = one_of("+-*/")(input)?;
    Ok((
        input,
        match val {
            '+' => Atom::Operater(BuiltinOp::Plus),
            '-' => Atom::Operater(BuiltinOp::Minus),
            '*' => Atom::Operater(BuiltinOp::Times),
            '/' => Atom::Operater(BuiltinOp::Divide),
            _ => unreachable!(),
        },
    ))
}

// シンボルをパースする関数
fn parse_symbol(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, name) = recognize(
        pair(
            alt((alpha1, tag("+"), tag("-"), tag("*"), tag("/"))),
            many0(alt((alphanumeric1, tag("-"), tag(">"))))
        )
    )(input)?;
    
    // 演算子と衝突しないか確認
    let result = match name {
        "+" | "-" | "*" | "/" => return parse_operater(input),
        _ => Atom::Symbol(name.to_string())
    };
    
    Ok((input, result))
}

fn parse_selfeval(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map(
        alt((parse_number, parse_bool, parse_operater, parse_symbol)),
        Expr::SelfEvaluation,
    )(input)
}

// define式をパースする関数
fn parse_define(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, _) = delimited(multispace0, tag("define"), multispace0)(input)?;
    let (input, symbol) = delimited(multispace0, 
        recognize(
            pair(
                alpha1,
                many0(alt((alphanumeric1, tag("-"), tag(">"))))
            )
        ), 
        multispace0
    )(input)?;
    let (input, value) = parse_expr(input)?;
    let (input, _) = delimited(multispace0, char(')'), multispace0)(input)?;
    
    Ok((input, Expr::Define(symbol.to_string(), Box::new(value))))
}

fn parse_procedure(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    
    // define特殊形式かどうかを確認
    if let Ok((input, _)) = tag::<_, _, VerboseError<&str>>("define")(input) {
        // define構文をパース
        let (_, _) = char('(')(input)?; // 元の入力に戻る
        return parse_define(input);
    }
    
    // 通常の手続き適用
    let (input, car) = context("string", delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, cdr) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Application(Box::new(car), cdr)))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((parse_define, parse_procedure, parse_selfeval))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::integer_positive("123", Ok(("", Atom::Num(123))))]
    #[case::integer_negative("-123", Ok(("", Atom::Num(-123))))]
    #[case::integer_negative("+123", Ok(("", Atom::Num(123))))]
    fn test_parse_number(#[case] input: &str, #[case] expected: IResult<&str, Atom, VerboseError<&str>>) {
        assert_eq!(parse_number(input), expected);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("#t"), Ok(("", Atom::Boolean(true))));
        assert_eq!(parse_bool("#f"), Ok(("", Atom::Boolean(false))));
    }

    #[test]
    fn test_parse_operater() {
        assert_eq!(
            parse_operater("+"),
            Ok(("", Atom::Operater(BuiltinOp::Plus)))
        );
        assert_eq!(
            parse_operater("-"),
            Ok(("", Atom::Operater(BuiltinOp::Minus)))
        );
        assert_eq!(
            parse_operater("*"),
            Ok(("", Atom::Operater(BuiltinOp::Times)))
        );
        assert_eq!(
            parse_operater("/"),
            Ok(("", Atom::Operater(BuiltinOp::Divide)))
        );
    }
    
    #[test]
    fn test_parse_symbol() {
        assert_eq!(
            parse_symbol("x"),
            Ok(("", Atom::Symbol("x".to_string())))
        );
        assert_eq!(
            parse_symbol("abc"),
            Ok(("", Atom::Symbol("abc".to_string())))
        );
        assert_eq!(
            parse_symbol("abc123"),
            Ok(("", Atom::Symbol("abc123".to_string())))
        );
    }

    #[test]
    fn test_parse_selfeval() {
        assert_eq!(
            parse_selfeval("123"),
            Ok(("", Expr::SelfEvaluation(Atom::Num(123))))
        );
        assert_eq!(
            parse_selfeval("#t"),
            Ok(("", Expr::SelfEvaluation(Atom::Boolean(true))))
        );
        assert_eq!(
            parse_selfeval("+"),
            Ok(("", Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))))
        );
        assert_eq!(
            parse_selfeval("x"),
            Ok(("", Expr::SelfEvaluation(Atom::Symbol("x".to_string()))))
        );
    }

    #[test]
    fn test_parse_define() {
        assert_eq!(
            parse_define("(define x 10)"),
            Ok((
                "",
                Expr::Define(
                    "x".to_string(),
                    Box::new(Expr::SelfEvaluation(Atom::Num(10)))
                )
            ))
        );
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(
            parse_procedure("(+ 1 2)"),
            Ok((
                "",
                Expr::Application(
                    Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                    vec![
                        Expr::SelfEvaluation(Atom::Num(1)),
                        Expr::SelfEvaluation(Atom::Num(2))
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_expr("(+(* 1 2)(- 3 4))"),
            Ok((
                "",
                Expr::Application(
                    Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                    vec![
                        Expr::Application(
                            Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Times))),
                            vec![
                                Expr::SelfEvaluation(Atom::Num(1)),
                                Expr::SelfEvaluation(Atom::Num(2))
                            ]
                        ),
                        Expr::Application(
                            Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Minus))),
                            vec![
                                Expr::SelfEvaluation(Atom::Num(3)),
                                Expr::SelfEvaluation(Atom::Num(4))
                            ]
                        )
                    ]
                )
            ))
        );
    }
    
    #[test]
    fn test_parse_define_expr() {
        assert_eq!(
            parse_expr("(define x 10)"),
            Ok((
                "",
                Expr::Define(
                    "x".to_string(),
                    Box::new(Expr::SelfEvaluation(Atom::Num(10)))
                )
            ))
        );
        
        assert_eq!(
            parse_expr("(define y (+ 1 2))"),
            Ok((
                "",
                Expr::Define(
                    "y".to_string(),
                    Box::new(Expr::Application(
                        Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                        vec![
                            Expr::SelfEvaluation(Atom::Num(1)),
                            Expr::SelfEvaluation(Atom::Num(2))
                        ]
                    ))
                )
            ))
        );
    }
}
