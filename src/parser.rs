use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, opt, recognize},
    error::{context, ErrorKind, VerboseError, VerboseErrorKind},
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    SelfEvaluation(Atom),
    Procedure(Box<Expr>, Vec<Expr>), // (operator args)
    Lambda(Vec<Expr>, Box<Expr>),    // (lambda (x y z) (expr))
    Symbol(String),                  // (lambda (x y z) (expr))
    // Special Syntax
    Define(String, Box<Expr>),
}

fn is_reserved(s: &str) -> bool {
    matches!(s, "define" | "lambda" | "#t" | "#f" | "if")
}

fn parse_number(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, val) = recognize(pair(opt(one_of("+-")), digit1))(input)?;
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

fn parse_symbol(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    // Allowed Indetify
    let (input, name) = recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)?;
    if is_reserved(name) {
        Err(nom::Err::Error(nom::error::VerboseError {
            errors: vec![(input, VerboseErrorKind::Nom(ErrorKind::Tag))],
        }))
    } else {
        Ok((input, Expr::Symbol(name.to_string())))
    }
}

fn parse_selfeval(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map(
        alt((parse_number, parse_bool, parse_operater)),
        Expr::SelfEvaluation,
    )(input)
}

/// # Examples
/// ```scheme
/// (define x (+ 1 2))
/// ```
fn parse_define(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, _) = delimited(multispace0, tag("define"), multispace1)(input)?;
    let (input, symbol) = delimited(
        multispace0,
        recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_")))))),
        multispace1,
    )(input)?;
    let (input, value) = parse_expr(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Define(symbol.to_string(), Box::new(value))))
}

fn parse_procedure(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, func) = context("string", delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, args) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Procedure(Box::new(func), args)))
}

fn parse_lambda(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, _) = delimited(multispace0, tag("lambda"), multispace1)(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = char(')')(input)?;
    let (input, body) = delimited(multispace0, parse_expr, multispace0)(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Lambda(args, Box::new(body))))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((
        parse_selfeval,
        parse_procedure,
        parse_lambda,
        parse_symbol,
        parse_define,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::integer_positive("123", Ok(("", Atom::Num(123))))]
    #[case::integer_negative("-123", Ok(("", Atom::Num(-123))))]
    #[case::integer_negative("+123", Ok(("", Atom::Num(123))))]
    fn test_parse_number(
        #[case] input: &str,
        #[case] expected: IResult<&str, Atom, VerboseError<&str>>,
    ) {
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
                Expr::Procedure(
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
                Expr::Procedure(
                    Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                    vec![
                        Expr::Procedure(
                            Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Times))),
                            vec![
                                Expr::SelfEvaluation(Atom::Num(1)),
                                Expr::SelfEvaluation(Atom::Num(2))
                            ]
                        ),
                        Expr::Procedure(
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
                    Box::new(Expr::Procedure(
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
